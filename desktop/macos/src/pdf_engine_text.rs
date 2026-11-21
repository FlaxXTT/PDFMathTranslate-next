use std::collections::BTreeMap;

use babeldoc_ir_core::{
    self as ir, AccessError, Cursor, Document, FieldValue, ReadOnlyView, TranslationView, TreeView,
    types,
};
use babeldoc_p2_stages::shared::layout_helper::normalize_unicode_nfkc;
use babeldoc_p2_stages::translation::{
    automatic_term_extractor::{
        TermParagraphFacts, TermTracking, TrackingPage, parse_terms, plan_page,
    },
    glossary::{Glossary, finalize_terms, normalize_source},
    ordinary_translation::normalize_response_punctuation,
    paragraph_helpers::is_pure_numeric_text,
};

use super::{Cancel, display, paper_sections::Sections};

#[derive(Clone, Copy, PartialEq, Eq)]
enum MarkerKind {
    Formula,
    StyleStart,
    StyleEnd,
}

struct Marker {
    kind: MarkerKind,
    index: usize,
    token: String,
}

pub(super) struct Job {
    pub(super) page: usize,
    paragraph: usize,
    pub text: String,
    marked_source: Option<String>,
    markers: Vec<Marker>,
}

impl Job {
    pub(super) fn preserve_markers(
        &self,
        translate: &mut impl FnMut(&[String]) -> Result<Vec<(String, bool)>, String>,
    ) -> Result<Option<(String, bool)>, String> {
        if self.markers.is_empty() {
            return Ok(None);
        }
        let mut start = 0;
        let mut fragments = Vec::new();
        for marker in &self.markers {
            let offset = self.text[start..]
                .find(&marker.token)
                .ok_or("Could not find a formula or formatting marker in the original text.")?
                + start;
            fragments.push(&self.text[start..offset]);
            start = offset + marker.token.len();
        }
        fragments.push(&self.text[start..]);
        let inputs = fragments
            .iter()
            .filter(|part| part.chars().any(char::is_alphabetic))
            .map(|part| part.trim().to_owned())
            .collect::<Vec<_>>();
        let mut translated = translate(&inputs)?.into_iter();
        let mut output = String::new();
        let mut kept_source = false;
        for (index, fragment) in fragments.into_iter().enumerate() {
            if fragment.chars().any(char::is_alphabetic) {
                let (replaced, retained) = translated.next().ok_or("The translation service omitted a text segment.")?;
                if replaced.trim().is_empty() {
                    return Err("The translation service returned an empty segment.".into());
                }
                kept_source |= retained;
                output.push_str(&fragment[..fragment.len() - fragment.trim_start().len()]);
                output.push_str(replaced.trim());
                output.push_str(&fragment[fragment.trim_end().len()..]);
            } else {
                output.push_str(fragment);
            }
            if let Some(marker) = self.markers.get(index) {
                output.push_str(&marker.token);
            }
        }
        Ok(Some((output, kept_source)))
    }
}

pub(super) fn collect(
    document: &mut Document,
    min_length: usize,
    sections: &Sections,
) -> Result<(Vec<Job>, bool), String> {
    let view = document.view::<ReadOnlyView>();
    let mut jobs = Vec::new();
    let mut preserved = false;
    for (page_index, page) in view
        .document()
        .page_iter(&view)
        .map_err(display)?
        .enumerate()
    {
        let page = page.map_err(display)?;
        let number = page
            .page_number(&view)
            .map_err(display)?
            .and_then(|value| usize::try_from(value).ok())
            .ok_or("The PDF page number is invalid.")?;
        for (paragraph_index, paragraph) in
            page.pdf_paragraph_iter(&view).map_err(display)?.enumerate()
        {
            let paragraph = paragraph.map_err(display)?;
            let original = paragraph
                .unicode(&view)
                .map_err(display)?
                .as_deref()
                .unwrap_or_default();
            let original = original.trim();
            let bounds = if let Some(bounds) = paragraph.r#box(&view).map_err(display)? {
                match [
                    bounds.x(&view).map_err(display)?,
                    bounds.y(&view).map_err(display)?,
                    bounds.x2(&view).map_err(display)?,
                    bounds.y2(&view).map_err(display)?,
                ] {
                    [Some(x), Some(y), Some(x2), Some(y2)] => Some([x, y, x2, y2]),
                    _ => None,
                }
            } else {
                None
            };
            if sections.contains(number, bounds, original) {
                preserved = true;
                continue;
            }
            if original.chars().count() < min_length
                || !original.chars().any(char::is_alphabetic)
                || is_pure_numeric_text(original)
                || is_standalone_equation(original)
                || paragraph.vertical(&view).map_err(display)?.unwrap_or(false)
            {
                continue;
            }
            let compos = paragraph
                .pdf_paragraph_composition_iter(&view)
                .map_err(display)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(display)?;
            if compos.is_empty() {
                continue;
            }
            let mut pieces = Vec::new();
            let mut markers = Vec::new();
            let mut has_prose = false;
            let paragraph_style = paragraph.pdf_style(&view).map_err(display)?;
            let mut marker_id = 1;
            for (index, composition) in compos.iter().enumerate() {
                if composition.pdf_formula(&view).map_err(display)?.is_some() {
                    let suffix = free_id(original, &mut marker_id);
                    push_marker(
                        &mut pieces,
                        &mut markers,
                        MarkerKind::Formula,
                        index,
                        suffix,
                    );
                } else if let Some(line) = composition.pdf_line(&view).map_err(display)? {
                    push_characters(
                        &mut pieces,
                        line.pdf_character_iter(&view).map_err(display)?,
                        &view,
                    )?;
                    has_prose = true;
                } else if let Some(same) = composition
                    .pdf_same_style_characters(&view)
                    .map_err(display)?
                {
                    let style = same.pdf_style(&view).map_err(display)?;
                    let different = !same_style(paragraph_style.as_ref(), style.as_ref(), &view)?;
                    let suffix = if different {
                        free_id(original, &mut marker_id)
                    } else {
                        0
                    };
                    if different {
                        push_marker(
                            &mut pieces,
                            &mut markers,
                            MarkerKind::StyleStart,
                            index,
                            suffix,
                        );
                    }
                    push_characters(
                        &mut pieces,
                        same.pdf_character_iter(&view).map_err(display)?,
                        &view,
                    )?;
                    if different {
                        push_marker(
                            &mut pieces,
                            &mut markers,
                            MarkerKind::StyleEnd,
                            index,
                            suffix,
                        );
                    }
                    has_prose = true;
                } else if let Some(character) = composition.pdf_character(&view).map_err(display)? {
                    push_characters(&mut pieces, [Ok(character)], &view)?;
                    has_prose = true;
                } else if let Some(text) = composition
                    .pdf_same_style_unicode_characters(&view)
                    .map_err(display)?
                {
                    if let Some(value) = text.unicode(&view).map_err(display)? {
                        pieces.push(Piece::Literal(value.clone()));
                        has_prose = true;
                    }
                }
            }
            if !has_prose || character_ids_unresolved(&pieces) {
                continue;
            }
            // Use BabelDOC's extracted reading order for plain text. Reconstruct
            // from styled components only when we must encode anchored spans.
            let text = if markers.is_empty() {
                original.to_owned()
            } else {
                compose(&pieces)
            };
            if text.trim().is_empty() {
                continue;
            }
            jobs.push(Job {
                page: page_index,
                paragraph: paragraph_index,
                marked_source: (!markers.is_empty()).then(|| original.to_owned()),
                text,
                markers,
            });
        }
    }
    Ok((jobs, preserved))
}

pub(super) fn extract_glossary(
    jobs: &[Job],
    language: &str,
    cancel: &Cancel<'_>,
    progress: &impl Fn(u8, &str),
    extract_terms: &mut impl FnMut(&str) -> Result<String, String>,
) -> Result<Option<Glossary>, String> {
    let pages = jobs.chunk_by(|left, right| left.page == right.page);
    let mut tracking = TermTracking {
        page: vec![TrackingPage::default(); pages.clone().count()],
    };
    let mut terms = Vec::new();
    progress(30, "Extracting terminology");
    for (page, jobs) in pages.enumerate() {
        cancel.check()?;
        let facts = jobs
            .iter()
            .map(|job| TermParagraphFacts {
                debug_id: Some(job.paragraph.to_string()),
                unicode: Some(job.marked_source.as_ref().unwrap_or(&job.text).clone()),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        let (batches, _) = plan_page(page, &facts, language, &[], &mut tracking);
        for batch in batches {
            cancel.check()?;
            let reply = extract_terms(&batch.prompt)?;
            let source = normalize_source(&batch.paragraphs.join(" "));
            let found = parse_terms(&reply).map_err(|_| "The translation service returned an invalid glossary.")?;
            terms.extend(
                found
                    .into_iter()
                    .filter(|(term, _)| source.contains(&normalize_source(term))),
            );
        }
        progress(
            30 + (5 * (page + 1) / tracking.page.len()) as u8,
            "Extracting terminology",
        );
    }
    finalize_terms("Document glossary", &terms).map_err(display)
}

fn is_standalone_equation(text: &str) -> bool {
    // Some PDFs assign formula variables the same italic font as prose.
    // BabelDOC still identifies their operators, but incorrectly sends the
    // remaining variables as text. Keep only mathematical expressions here.
    let equation = text.trim_end_matches('.');
    let Some((left, right)) = equation.split_once(['=', '≠', '≈', '≡', '≤', '≥', '∝'])
    else {
        return false;
    };
    let is_operand = |side: &str| {
        side.chars().any(char::is_alphanumeric)
            && side.chars().all(|ch| {
                ch.is_ascii_alphanumeric()
                    || matches!(ch, '\u{0370}'..='\u{03ff}' | '\u{2070}'..='\u{209f}' | '\u{2200}'..='\u{22ff}')
                    || ch.is_whitespace()
                    || "+-*/^_.!%,()[]{}|∞≠≈≡≤≥∝-=∂∇±×÷−′″".contains(ch)
            })
            && side.split(|ch: char| !ch.is_ascii_alphabetic()).all(|word| {
                word.len() <= 2
                    || matches!(
                        word,
                        "sin" | "cos" | "tan" | "log" | "exp" | "lim" | "det" | "max" | "min"
                    )
            })
    };
    is_operand(left) && is_operand(right)
}

fn same_style(
    left: Option<&Cursor<types::PdfStyle>>,
    right: Option<&Cursor<types::PdfStyle>>,
    view: &TreeView<'_, ReadOnlyView>,
) -> Result<bool, String> {
    let (Some(left), Some(right)) = (left, right) else {
        return Ok(left.is_none() && right.is_none());
    };
    if left.font_id(view).map_err(display)? != right.font_id(view).map_err(display)? {
        return Ok(false);
    }
    let left_size = left.font_size(view).map_err(display)?.unwrap_or_default();
    let right_size = right.font_size(view).map_err(display)?.unwrap_or_default();
    if (left_size - right_size).abs() > 0.02 {
        return Ok(false);
    }
    let instructions = |style: &Cursor<types::PdfStyle>| {
        let Some(state) = style.graphic_state(view).map_err(display)? else {
            return Ok(None);
        };
        let reference = state
            .passthrough_per_char_instruction(view)
            .map_err(display)?;
        view.render_instructions(reference).map_err(display)
    };
    Ok(instructions(left)? == instructions(right)?)
}

enum Piece {
    Character {
        text: String,
        bounds: Option<[f64; 4]>,
    },
    Literal(String),
}

fn character_ids_unresolved(pieces: &[Piece]) -> bool {
    let mut total = 0;
    let mut unresolved = 0;
    for piece in pieces {
        if let Piece::Character { text, .. } = piece {
            total += 1;
            if text
                .strip_prefix("(cid:")
                .and_then(|digits| digits.strip_suffix(')'))
                .is_some_and(|digits| {
                    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
                })
            {
                unresolved += 1;
            }
        }
    }
    unresolved * 5 > total * 4
}

fn token(kind: MarkerKind, id: usize) -> String {
    let letter = match kind {
        MarkerKind::Formula => 'F',
        MarkerKind::StyleStart => 'L',
        MarkerKind::StyleEnd => 'R',
    };
    format!("[[BD_{letter}_{id}]]")
}

fn free_id(text: &str, next: &mut usize) -> usize {
    loop {
        let id = *next;
        *next += 1;
        if [
            MarkerKind::Formula,
            MarkerKind::StyleStart,
            MarkerKind::StyleEnd,
        ]
        .into_iter()
        .all(|kind| !text.contains(&token(kind, id)))
        {
            return id;
        }
    }
}

fn push_marker(
    pieces: &mut Vec<Piece>,
    markers: &mut Vec<Marker>,
    kind: MarkerKind,
    index: usize,
    id: usize,
) {
    let token = token(kind, id);
    pieces.push(Piece::Literal(token.clone()));
    markers.push(Marker { kind, index, token });
}

fn push_characters(
    pieces: &mut Vec<Piece>,
    characters: impl IntoIterator<Item = Result<Cursor<types::PdfCharacter>, AccessError>>,
    view: &TreeView<'_, ReadOnlyView>,
) -> Result<(), String> {
    for character in characters {
        let character = character.map_err(display)?;
        let text = character
            .char_unicode(view)
            .map_err(display)?
            .clone()
            .unwrap_or_default();
        let bounds = character
            .r#box(view)
            .map_err(display)?
            .map(|rect| {
                Ok::<_, AccessError>([
                    rect.x(view)?.unwrap_or_default(),
                    rect.y(view)?.unwrap_or_default(),
                    rect.x2(view)?.unwrap_or_default(),
                    rect.y2(view)?.unwrap_or_default(),
                ])
            })
            .transpose()
            .map_err(display)?;
        pieces.push(Piece::Character { text, bounds });
    }
    Ok(())
}

fn compose(pieces: &[Piece]) -> String {
    let mut distances = pieces
        .windows(2)
        .filter_map(|pair| match (&pair[0], &pair[1]) {
            (
                Piece::Character {
                    bounds: Some(a), ..
                },
                Piece::Character {
                    bounds: Some(b), ..
                },
            ) if b[0] - a[2] > 1.0 => Some(b[0] - a[2]),
            _ => None,
        })
        .collect::<Vec<_>>();
    distances.sort_by(f64::total_cmp);
    distances.dedup_by(|left, right| left == right);
    let spacing = distances
        .get(1)
        .or_else(|| distances.first())
        .copied()
        .unwrap_or(1.0);
    let mut output = String::new();
    for (position, piece) in pieces.iter().enumerate() {
        match piece {
            Piece::Literal(text) => output.push_str(text),
            Piece::Character { text, bounds } => {
                output.push_str(text);
                if text == " " {
                    continue;
                }
                if let (
                    Some(a),
                    Some(Piece::Character {
                        bounds: Some(b), ..
                    }),
                ) = (bounds, pieces.get(position + 1))
                {
                    let wide = (a[2] - a[0]).max(b[2] - b[0]);
                    if b[0] - a[2] >= spacing || b[3] < a[1] || b[2] < a[0] - wide * 10.0 {
                        output.push(' ');
                    }
                }
            }
        }
    }
    normalize_unicode_nfkc(&output)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn apply(document: &mut Document, job: &Job, output: &str) -> Result<(), String> {
    let output = normalize_response_punctuation(output.trim());
    if output.trim().is_empty() {
        return Err("The translation service returned an empty paragraph.".into());
    }
    let mut occurrences = BTreeMap::new();
    for marker in &job.markers {
        let mut matches = output.match_indices(&marker.token);
        let Some((position, _)) = matches.next() else {
            return Err("The translation altered formula or formatting markers. Retry or choose a different model.".into());
        };
        if matches.next().is_some() {
            return Err("The translation duplicated formula or formatting markers. Retry or choose a different model.".into());
        }
        occurrences.insert(position, marker);
    }
    let mut view = document.view::<TranslationView>();
    let paragraph = view
        .document()
        .page_at(&view, job.page)
        .map_err(display)?
        .pdf_paragraph_at(&view, job.paragraph)
        .map_err(display)?;
    let mut compos = Vec::new();
    let mut pos = 0;
    let mut active_style = None;
    let mut plain = String::new();
    for (offset, marker) in occurrences {
        if offset < pos {
            return Err("Markers in the translation are out of order.".into());
        }
        let text = &output[pos..offset];
        if !text.is_empty() {
            let style = if let Some(index) = active_style {
                paragraph
                    .pdf_paragraph_composition_at(&view, index)
                    .map_err(display)?
                    .pdf_same_style_characters(&view)
                    .map_err(display)?
                    .ok_or("Could not read the original formatting.")?
                    .pdf_style(&view)
                    .map_err(display)?
            } else {
                paragraph.pdf_style(&view).map_err(display)?
            };
            plain.push_str(text);
            compos.push(text_composition(text, style, &mut view)?);
        }
        match marker.kind {
            MarkerKind::Formula => {
                let formula = paragraph
                    .pdf_paragraph_composition_at(&view, marker.index)
                    .map_err(display)?
                    .pdf_formula(&view)
                    .map_err(display)?
                    .ok_or("Could not read a formula from the original.")?;
                compos.push(
                    ir::PdfParagraphCompositionInit {
                        pdf_formula: Some(formula.share(&mut view).map_err(display)?),
                        ..Default::default()
                    }
                    .into_value(),
                );
            }
            MarkerKind::StyleStart if active_style.is_none() => active_style = Some(marker.index),
            MarkerKind::StyleEnd if active_style == Some(marker.index) => active_style = None,
            _ => return Err("Formatting markers in the translation do not match. Retry or choose a different model.".into()),
        }
        pos = offset + marker.token.len();
    }
    if active_style.is_some() {
        return Err("A formatting marker in the translation was left open.".into());
    }
    if pos < output.len() {
        let text = &output[pos..];
        plain.push_str(text);
        let style = paragraph.pdf_style(&view).map_err(display)?;
        compos.push(text_composition(text, style, &mut view)?);
    }
    if compos.is_empty() {
        return Err("The translation service did not return usable text.".into());
    }
    paragraph
        .set_unicode(&mut view, Some(plain))
        .map_err(display)?;
    paragraph
        .set_pdf_paragraph_composition(&mut view, compos)
        .map_err(display)
}

fn text_composition(
    text: &str,
    style: Option<Cursor<types::PdfStyle>>,
    view: &mut TreeView<'_, TranslationView>,
) -> Result<FieldValue<types::PdfParagraphComposition>, String> {
    let styled = ir::PdfSameStyleUnicodeCharactersInit {
        pdf_style: style
            .map(|style| style.share(view))
            .transpose()
            .map_err(display)?,
        unicode: Some(text.to_owned()),
        debug_info: None,
    }
    .into_value();
    Ok(ir::PdfParagraphCompositionInit {
        pdf_same_style_unicode_characters: Some(styled),
        ..Default::default()
    }
    .into_value())
}
