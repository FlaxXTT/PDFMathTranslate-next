//! Native orchestration of BabelDOC's parser, layout stages and PDF writer.
//! Translation transport is injected by the workbench.
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use babeldoc_ir_core::{ReadOnlyView, WriterView};
use babeldoc_p2_stages::font_mapper::{
    FontMapperWithSourceStyles, font_mapper_with_source_styles_from_verified_asset_paths,
};
use babeldoc_p2_stages::stages::{
    layout_parser, paragraph_finder, paragraph_finder_inputs, styles_and_formulas,
    styles_and_formulas_config, typesetting,
};
use babeldoc_p2_stages::{Cancellation, NoopProgress, StageContext};
use babeldoc_writer_core as writer;
use serde_json::{Value, json};

#[path = "pdf_engine_output.rs"]
mod output;
#[path = "paper_sections.rs"]
mod paper_sections;
#[path = "pdf_engine_text.rs"]
mod paragraphs;

const FONT_CATALOG: &str = include_str!("data/babeldoc-fonts.json");
const DOC_LAYOUT: &str = "doclayout_yolo_docstructbench_imgsz1024.onnx";
const ONNX_LIBRARY: &str = "libonnxruntime.1.32.0.dylib";

#[derive(Clone, Debug)]
pub struct PdfOptions {
    pub pages: String,
    pub lang_out: String,
    pub mono: bool,
    pub dual: bool,
    pub alternating: bool,
    pub translated_first: bool,
    pub only_selected: bool,
    pub min_text_length: usize,
    pub font_family: Option<String>,
    pub glossary: bool,
}

impl Default for PdfOptions {
    fn default() -> Self {
        Self {
            pages: String::new(),
            lang_out: "zh-CN".into(),
            mono: true,
            dual: true,
            alternating: false,
            translated_first: false,
            only_selected: false,
            min_text_length: 1,
            font_family: None,
            glossary: false,
        }
    }
}

#[derive(Debug)]
pub struct PdfResult {
    pub mono: Option<PathBuf>,
    pub dual: Option<PathBuf>,
    pub glossary: Option<PathBuf>,
}

struct Cancel<'a>(&'a AtomicBool);

impl Cancel<'_> {
    fn check(&self) -> Result<(), String> {
        if self.is_cancelled() {
            return Err("Canceled".into());
        }
        Ok(())
    }
}

impl Cancellation for Cancel<'_> {
    fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

impl babeldoc_parser_core::ParseObserver for Cancel<'_> {
    fn page_start(&mut self) -> babeldoc_parser_core::Result<()> {
        self.check()
            .map_err(|detail| babeldoc_parser_core::Error::new("Cancelled", detail))
    }

    fn retain_page_text(&self) -> bool {
        false
    }
}

pub fn check_pdf(path: &Path) -> Result<(), String> {
    let document = writer::PdfDocument::load(path).map_err(display)?;
    if document.is_encrypted() {
        return Err("This PDF requires a password.".into());
    }
    if !document.page_iter().any(|_| true) {
        return Err("This PDF has no pages.".into());
    }
    Ok(())
}

fn select_pages(pages: &str, count: usize) -> Result<BTreeSet<usize>, String> {
    let pages = pages.trim();
    if pages.is_empty() || pages.eq_ignore_ascii_case("all") || pages == "*" {
        return Ok((0..count).collect());
    }
    let number = |item: &str| {
        item.trim()
            .parse::<usize>()
            .ok()
            .filter(|page| *page > 0)
            .ok_or_else(|| format!("Invalid page range ({item})."))
    };
    let mut selected = BTreeSet::new();
    for range in pages.replace('，', ",").split(',') {
        let (start, end) = if let Some((start, end)) = range.split_once('-') {
            let start = if start.trim().is_empty() {
                1
            } else {
                number(start)?
            };
            let end = if end.trim().is_empty() {
                count
            } else {
                number(end)?
            };
            (start, end)
        } else {
            let page = number(range)?;
            (page, page)
        };
        if start > end && start <= count {
            return Err(format!("Invalid page range ({range})."));
        }
        if start <= count {
            selected.extend(start - 1..end.min(count));
        }
    }
    if selected.is_empty() {
        return Err("The selected page range does not contain any PDF pages.".into());
    }
    Ok(selected)
}

pub fn run(
    input: &Path,
    output: &Path,
    assets: &Path,
    opts: &PdfOptions,
    cancellation: &AtomicBool,
    progress: impl Fn(u8, &str) + Send + Sync,
    mut translate: impl FnMut(&[String], &[(String, String)]) -> Result<Vec<(String, bool)>, String>,
    mut extract_terms: impl FnMut(&str) -> Result<String, String>,
) -> Result<(PdfResult, usize), String> {
    let mut cancel = Cancel(cancellation);
    if !opts.mono && !opts.dual {
        return Err("Select at least one output format.".into());
    }
    cancel.check()?;
    progress(5, "Reading PDF");

    let source = writer::PdfDocument::load(input).map_err(display)?;
    if source.is_encrypted() {
        return Err("This PDF requires a password.".into());
    }
    let count = source.page_iter().count();
    if count == 0 {
        return Err("This PDF has no pages.".into());
    }
    let selected = select_pages(&opts.pages, count)?;
    let sections = paper_sections::inspect(input, || cancel.check())?;
    let translation_pages: BTreeSet<_> = selected
        .iter()
        .copied()
        .filter(|&page| !sections.whole_page(page))
        .collect();
    if translation_pages.is_empty() {
        progress(96, "Saving original text");
        return output::save(
            input,
            output,
            source.clone(),
            source,
            selected,
            None,
            opts,
            &cancel,
        )
        .map(|result| (result, 0));
    }
    let config = babeldoc_parser_core::Config {
        pages: Some(translation_pages.iter().copied().collect()),
        normalize_page_geometry: true,
        ..Default::default()
    };
    let parsed =
        babeldoc_parser_core::parse_observed(input, config, &mut cancel).map_err(display)?;
    let page_objects = writer::PageObjectMap::new(parsed.page_objects);
    let mut document = parsed.document;

    cancel.check()?;
    progress(12, "Detecting layout");
    let mut model = crate::layout_engine::LayoutModel::new(
        &assets.join("models").join(DOC_LAYOUT),
        &assets.join("models").join(ONNX_LIBRARY),
    )?;
    let page_inputs = layout_parser::page_inputs(&mut document, false).map_err(display)?;
    for (index, page) in page_inputs.iter().enumerate() {
        cancel.check()?;
        let Some(number) = page
            .page_number
            .and_then(|number| usize::try_from(number).ok())
        else {
            continue;
        };
        if !translation_pages.contains(&number) {
            continue;
        }
        let (mediabox, boxes) = model.infer(input, number)?;
        layout_parser::apply_model(
            &mut document,
            &layout_parser::PageResult {
                index: Some(index),
                mediabox: Some(mediabox),
                boxes,
            },
        )
        .map_err(display)?;
        progress(
            12 + (10 * (index + 1) / page_inputs.len().max(1)) as u8,
            "Detecting layout",
        );
    }
    drop(model);

    let catalog: Value = serde_json::from_str(FONT_CATALOG).map_err(display)?;
    let (font_catalog, font_family) = font_setup(&catalog, &opts.lang_out);
    let FontMapperWithSourceStyles {
        mapper,
        source_styles,
    } = load_fonts(
        assets,
        font_catalog,
        font_family,
        opts.font_family.as_deref(),
    )?;
    let mut stages = StageContext::new(&cancel, &NoopProgress);
    layout_parser::fallback(&mut document, false, &NoopProgress).map_err(display)?;

    progress(26, "Analyzing paragraphs and formulas");
    let paragraph_config = paragraph_finder_inputs::ParagraphFinderConfig {
        merge_alternating_line_numbers: Some(true),
        short_line_split_factor: 0.8,
        ..Default::default()
    };
    let mut debug_ids = paragraph_finder_inputs::RandomDebugIdSource;
    paragraph_finder::process_with(
        &mut paragraph_finder_inputs::ParagraphFinderInputs {
            config: &paragraph_config,
            font_mapper: &mapper,
            debug_ids: &mut debug_ids,
        },
        &mut document,
        &mut stages,
    )
    .map_err(display)?;
    let styles = styles_and_formulas_config::StylesAndFormulasConfig::new((*mapper).clone());
    styles_and_formulas::process(&styles, &mut document, &mut stages).map_err(display)?;

    let (jobs, preserved) = paragraphs::collect(&mut document, opts.min_text_length, &sections)?;
    if jobs.is_empty() {
        if preserved || translation_pages.len() < selected.len() {
            progress(96, "Saving original text");
            return output::save(
                input,
                output,
                source.clone(),
                source,
                selected,
                None,
                opts,
                &cancel,
            )
            .map(|result| (result, 0));
        }
        return Err("No translatable text was found. Convert scanned pages to a searchable PDF first.".into());
    }
    let mut translated_page_indices = BTreeSet::new();
    let glossary = if opts.glossary {
        paragraphs::extract_glossary(
            &jobs,
            &opts.lang_out,
            &cancel,
            &progress,
            &mut extract_terms,
        )?
    } else {
        None
    };
    progress(35, "Translating text");
    let mut translated = 0;
    let mut retained_paragraphs = 0;
    for batch in jobs.chunks(12) {
        cancel.check()?;
        let texts = batch.iter().map(|job| job.text.clone()).collect::<Vec<_>>();
        let terms = glossary
            .as_ref()
            .map(|glossary| glossary.active_entries_for_text(&texts.join("\n")))
            .unwrap_or_default();
        let mut translate = |texts: &[String]| translate(texts, &terms);
        let values = translate(&texts)?;
        if values.len() != batch.len() {
            return Err("The translation service returned a different number of paragraphs than requested.".into());
        }
        for (job, (value, kept_source)) in batch.iter().zip(values) {
            cancel.check()?;
            let (applied, retained) = if kept_source && value == job.text {
                (false, true)
            } else {
                match paragraphs::apply(&mut document, job, &value) {
                    Ok(()) => (true, kept_source),
                    Err(error) => {
                        cancel.check()?;
                        let Some((repaired, repaired_kept_source)) =
                            job.preserve_markers(&mut translate)?
                        else {
                            return Err(error);
                        };
                        let apply_repair = !repaired_kept_source || repaired != job.text;
                        if apply_repair {
                            paragraphs::apply(&mut document, job, &repaired)?;
                        }
                        (apply_repair, repaired_kept_source)
                    }
                }
            };
            if applied {
                translated_page_indices.insert(job.page);
            }
            if retained {
                retained_paragraphs += 1;
            }
            translated += 1;
        }
        progress(35 + (40 * translated / jobs.len()) as u8, "Translating text");
    }

    cancel.check()?;
    if translated_page_indices.is_empty() {
        progress(96, "Saving original text");
        return output::save(
            input,
            output,
            source.clone(),
            source,
            selected,
            glossary,
            opts,
            &cancel,
        )
        .map(|result| (result, retained_paragraphs));
    }
    progress(78, "Typesetting translation");
    let source_styles: HashMap<_, _> = source_styles.into_iter().collect();
    typesetting::Typesetting::new(&opts.lang_out, mapper.clone(), true, source_styles)
        .process(&mut document, &mut stages)
        .map_err(display)?;

    cancel.check()?;
    progress(88, "Generating PDF");
    let mut mono = source.clone();
    let ids = font_ids(font_family)?;
    let base = font_family["base"]
        .as_array()
        .and_then(|names| names.first())
        .and_then(Value::as_str)
        .ok_or("The base typesetting font is missing.")?;
    let faces = ids
        .iter()
        .map(|id| {
            mapper
                .font_by_id(id)
                .ok_or_else(|| format!("Font {id} is missing."))
        })
        .chain([mapper
            .font_by_id("base")
            .ok_or_else(|| "The base typesetting font is missing.".into())])
        .collect::<Result<Vec<_>, String>>()?;
    let metadata = ids
        .iter()
        .map(String::as_str)
        .chain([base])
        .map(|id| {
            let info = &font_catalog[id];
            Ok(writer::FontMetadata {
                font_name: info["font_name"].as_str().ok_or("The font name is missing.")?.to_owned(),
                ascent: info["ascent"].as_f64().ok_or("The font ascent is missing.")?,
                descent: info["descent"].as_f64().ok_or("The font descent is missing.")?,
                encoding_length: info["encoding_length"].as_i64().ok_or("The font encoding width is missing.")?,
            })
        })
        .collect::<Result<Vec<_>, &str>>()?;
    let assets_for_writer = ids
        .iter()
        .map(String::as_str)
        .chain(["base"])
        .zip(faces.iter().zip(&metadata))
        .map(|(id, (face, metadata))| writer::FontAsset { id, face, metadata })
        .collect::<Vec<_>>();
    let used = writer::used_font_ids(&document.view::<WriterView>()).map_err(display)?;
    let first_new_id = mono.max_id.saturating_add(1);
    let installed = writer::add_fonts_with_used_ids_from_page_objects(
        &mut mono,
        &mut document.view::<WriterView>(),
        &assets_for_writer,
        &used,
        &page_objects,
        |_| {
            if cancel.is_cancelled() {
                Err(writer::WriterError::Invalid("Canceled".into()))
            } else {
                Ok(())
            }
        },
    )
    .map_err(display)?;
    let mut font_subset = crate::font_subset::FontSubset::new(first_new_id, &installed);

    let view = document.view::<ReadOnlyView>();
    for (index, page) in view
        .document()
        .page_iter(&view)
        .map_err(display)?
        .enumerate()
    {
        cancel.check()?;
        let page = page.map_err(display)?;
        if !translated_page_indices.contains(&index) {
            continue;
        }
        let instructions =
            writer::build_page_output(&view, &page, &writer::BuildOptions::default())
                .map_err(display)?;
        font_subset.record(&instructions);
        writer::apply_page_output_with_page_objects(&mut mono, &instructions, &page_objects)
            .map_err(display)?;
    }
    cancel.check()?;
    font_subset.apply(&mut mono);
    progress(96, "Saving translation");
    output::save(
        input, output, source, mono, selected, glossary, opts, &cancel,
    )
    .map(|result| (result, retained_paragraphs))
}

fn font_setup<'a>(catalog: &'a Value, lang: &str) -> (&'a Value, &'a Value) {
    let lang = lang.to_ascii_uppercase();
    let code = if lang.contains("KR") || lang == "KO" {
        "kr"
    } else if lang.contains("JA") || lang.contains("JP") {
        "jp"
    } else if lang.contains("HK") {
        "hk"
    } else if lang.contains("TW") {
        "tw"
    } else if lang.contains("CN") || lang == "ZH" {
        "cn"
    } else {
        "en"
    };
    (&catalog["fonts"], &catalog["families"][code])
}

fn font_ids(family: &Value) -> Result<Vec<String>, String> {
    let mut ids = Vec::new();
    for group in ["normal", "script", "fallback", "base"] {
        let members = family[group].as_array().ok_or("The font catalog format is invalid.")?;
        for value in members {
            let id = value.as_str().ok_or("The font filename is invalid.")?;
            if !ids.iter().any(|existing| existing == id) {
                ids.push(id.to_owned());
            }
        }
    }
    Ok(ids)
}

fn load_fonts(
    assets: &Path,
    catalog: &Value,
    family: &Value,
    preferred: Option<&str>,
) -> Result<FontMapperWithSourceStyles, String> {
    let mut paths = serde_json::Map::new();
    let mut hashes = serde_json::Map::new();
    for id in font_ids(family)? {
        paths.insert(id.clone(), json!(assets.join("fonts").join(&id)));
        hashes.insert(id.clone(), catalog[id]["sha3_256"].clone());
    }
    font_mapper_with_source_styles_from_verified_asset_paths(
        family,
        &Value::Object(paths),
        &Value::Object(hashes),
        preferred,
    )
    .map_err(display)
}

fn display(error: impl std::fmt::Display) -> String {
    error.to_string()
}
