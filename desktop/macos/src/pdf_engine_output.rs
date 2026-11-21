use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::Path;

use babeldoc_p2_stages::translation::glossary::Glossary;
use lopdf::{Dictionary, Document, Object, ObjectId, Stream, dictionary};

use super::{Cancel, PdfOptions, PdfResult, display};
use crate::APP_NAME;

pub(super) fn save(
    input: &Path,
    output: &Path,
    original: Document,
    mut mono: Document,
    selected: BTreeSet<usize>,
    glossary: Option<Glossary>,
    opts: &PdfOptions,
    cancel: &Cancel<'_>,
) -> Result<PdfResult, String> {
    fs::create_dir_all(output).map_err(display)?;
    let scratch = tempfile::Builder::new()
        .prefix(".pdf-")
        .tempdir_in(output)
        .map_err(display)?;
    let name = input
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("translation");
    let language: String = opts
        .lang_out
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
        .collect();
    let mono_name = format!("{name}.{language}.mono.pdf");
    let dual_name = format!("{name}.{language}.dual.pdf");
    let glossary_name = format!("{name}.{language}.glossary.csv");
    let producer = format!("{APP_NAME} / BabelDOC native translation");

    if opts.dual {
        cancel.check()?;
        let mut dual = compose_dual(original, &mono, &selected, opts, cancel)?;
        dual.change_producer(&producer);
        dual.compress();
        dual.save(scratch.path().join(&dual_name))
            .map_err(display)?;
    }
    if opts.mono {
        cancel.check()?;
        if opts.only_selected {
            let dropped = (0..mono.page_iter().count())
                .filter(|index| !selected.contains(index))
                .map(|index| index as u32 + 1)
                .collect::<Vec<_>>();
            mono.delete_pages(&dropped);
        }
        mono.prune_objects();
        mono.change_producer(&producer);
        mono.compress();
        mono.save(scratch.path().join(&mono_name))
            .map_err(display)?;
    }

    if let Some(glossary) = &glossary {
        fs::write(scratch.path().join(&glossary_name), glossary.to_csv()).map_err(display)?;
    }
    cancel.check()?;
    let publish = |name: &str, enabled: bool| {
        if enabled {
            let target = output.join(name);
            fs::rename(scratch.path().join(name), &target).map_err(display)?;
            Ok(Some(target))
        } else {
            Ok::<_, String>(None)
        }
    };
    Ok(PdfResult {
        mono: publish(&mono_name, opts.mono)?,
        dual: publish(&dual_name, opts.dual)?,
        glossary: publish(&glossary_name, glossary.is_some())?,
    })
}

fn compose_dual(
    mut original: Document,
    translated: &Document,
    selected: &BTreeSet<usize>,
    opts: &PdfOptions,
    cancel: &Cancel<'_>,
) -> Result<Document, String> {
    let mut translated = translated.clone();
    let source_pages = original.page_iter().collect::<Vec<_>>();
    let next = original
        .max_id
        .checked_add(1)
        .ok_or("The PDF has no remaining object IDs.")?;
    translated.renumber_objects_with(next);
    let translated_pages = translated.page_iter().collect::<Vec<_>>();
    if source_pages.len() != translated_pages.len() {
        return Err("Could not match the original pages to the translated pages.".into());
    }
    original.max_id = original.max_id.max(translated.max_id);
    original.objects.append(&mut translated.objects);
    let root = original
        .trailer
        .get(b"Root")
        .and_then(Object::as_reference)
        .map_err(display)?;
    let pages = original.new_object_id();
    let mut children = Vec::new();
    for (index, (source, translated)) in source_pages.into_iter().zip(translated_pages).enumerate()
    {
        cancel.check()?;
        if opts.only_selected && !selected.contains(&index) {
            continue;
        }
        let (left, right) = if opts.translated_first {
            (translated, source)
        } else {
            (source, translated)
        };
        if opts.alternating {
            attach_page(&mut original, left, pages)?;
            attach_page(&mut original, right, pages)?;
            children.extend([Object::Reference(left), Object::Reference(right)]);
        } else {
            let left_form = form(&mut original, left)?;
            let right_form = form(&mut original, right)?;
            insert_spread(&mut original, source, pages, left_form, right_form)?;
            children.push(Object::Reference(source));
        }
    }
    original.objects.insert(
        pages,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Count" => children.len() as i64,
            "Kids" => Object::Array(children),
        }),
    );
    original
        .get_dictionary_mut(root)
        .map_err(display)?
        .set("Pages", pages);
    original.prune_objects();
    Ok(original)
}

fn attach_page(document: &mut Document, page: ObjectId, parent: ObjectId) -> Result<(), String> {
    // Put inheritable entries on the page before changing its ancestry.
    let values = [b"Resources".as_slice(), b"MediaBox", b"CropBox", b"Rotate"]
        .into_iter()
        .map(|key| inherit(document, page, key).map(|value| (key, value.cloned())))
        .collect::<Result<Vec<_>, _>>()?;
    let page = document.get_dictionary_mut(page).map_err(display)?;
    for (key, value) in values {
        if let Some(value) = value {
            page.set(key, value);
        }
    }
    page.set("Parent", parent);
    Ok(())
}

fn inherit<'a>(
    document: &'a Document,
    mut page: ObjectId,
    key: &[u8],
) -> Result<Option<&'a Object>, String> {
    let mut seen = HashSet::new();
    while seen.insert(page) && seen.len() < 256 {
        let dictionary = document.get_dictionary(page).map_err(display)?;
        if let Ok(value) = dictionary.get(key) {
            return Ok(Some(value));
        }
        match dictionary.get(b"Parent").and_then(Object::as_reference) {
            Ok(parent) => page = parent,
            Err(_) => return Ok(None),
        }
    }
    Err("The PDF page tree contains a cycle.".into())
}

fn resolve<'a>(document: &'a Document, value: &'a Object) -> Result<&'a Object, String> {
    if let Object::Reference(id) = value {
        document.get_object(*id).map_err(display)
    } else {
        Ok(value)
    }
}

fn bounds(document: &Document, page: ObjectId) -> Result<[f64; 4], String> {
    let value = inherit(document, page, b"CropBox")?
        .or(inherit(document, page, b"MediaBox")?)
        .ok_or("The PDF is missing page dimensions.")?;
    let coordinates = resolve(document, value)?.as_array().map_err(display)?;
    if coordinates.len() != 4 {
        return Err("The PDF page dimensions have an invalid format.".into());
    }
    let mut rect = [0.; 4];
    for (index, coordinate) in coordinates.iter().enumerate() {
        rect[index] = resolve(document, coordinate)?.as_float().map_err(display)? as f64;
    }
    if !rect.iter().all(|coordinate| coordinate.is_finite())
        || rect[0] >= rect[2]
        || rect[1] >= rect[3]
    {
        return Err("The PDF page dimensions are invalid.".into());
    }
    Ok(rect)
}

struct PageForm {
    id: ObjectId,
    width: f64,
    height: f64,
    matrix: [f64; 6],
}

fn form(document: &mut Document, page: ObjectId) -> Result<PageForm, String> {
    let crop = bounds(document, page)?;
    let [x0, y0, x1, y1] = crop;
    let rotate = inherit(document, page, b"Rotate")?
        .map(|value| resolve(document, value).and_then(|value| value.as_i64().map_err(display)))
        .transpose()?
        .unwrap_or(0)
        .rem_euclid(360);
    let width = x1 - x0;
    let height = y1 - y0;
    let (width, height, mut matrix) = match rotate {
        90 => (height, width, [0., -1., 1., 0., -y0, x1]),
        180 => (width, height, [-1., 0., 0., -1., x1, y1]),
        270 => (height, width, [0., 1., -1., 0., y1, -x0]),
        _ => (width, height, [1., 0., 0., 1., -x0, -y0]),
    };
    let user_unit = document
        .get_dictionary(page)
        .map_err(display)?
        .get(b"UserUnit")
        .ok()
        .and_then(|value| resolve(document, value).ok())
        .and_then(|value| value.as_float().ok())
        .map(f64::from)
        .filter(|unit| unit.is_finite() && *unit > 0.)
        .unwrap_or(1.);
    for coefficient in &mut matrix {
        *coefficient *= user_unit;
    }
    let resource = inherit(document, page, b"Resources")?
        .cloned()
        .unwrap_or_else(|| Object::Dictionary(Dictionary::new()));
    let content = document.get_page_content(page);
    let mut dictionary = dictionary! {
        "Type" => "XObject",
        "Subtype" => "Form",
        "FormType" => 1,
        "BBox" => numbers(crop),
        "Resources" => resource,
    };
    if let Ok(group) = document
        .get_dictionary(page)
        .and_then(|page| page.get(b"Group"))
    {
        dictionary.set("Group", group.clone());
    }
    let id = document.add_object(Stream::new(dictionary, content));
    Ok(PageForm {
        id,
        width: width * user_unit,
        height: height * user_unit,
        matrix,
    })
}

fn insert_spread(
    document: &mut Document,
    page: ObjectId,
    parent: ObjectId,
    mut left: PageForm,
    mut right: PageForm,
) -> Result<(), String> {
    let width = left.width + right.width;
    let height = left.height.max(right.height);
    left.matrix[5] += height - left.height;
    right.matrix[4] += left.width;
    right.matrix[5] += height - right.height;
    let transform = |matrix: [f64; 6], name: &str| {
        let [a, b, c, d, e, f] = matrix;
        format!("q {a:.6} {b:.6} {c:.6} {d:.6} {e:.6} {f:.6} cm /{name} Do Q\n")
    };
    let body = transform(left.matrix, "BDLeft") + &transform(right.matrix, "BDRight");
    let stream = document.add_object(Stream::new(Dictionary::new(), body.into_bytes()));
    let resources =
        dictionary! { "XObject" => dictionary! { "BDLeft" => left.id, "BDRight" => right.id } };
    document.objects.insert(
        page,
        Object::Dictionary(dictionary! {
            "Type" => "Page",
            "Parent" => parent,
            "MediaBox" => numbers([0., 0., width, height]),
            "Resources" => resources,
            "Contents" => stream,
        }),
    );
    Ok(())
}

fn numbers<const N: usize>(numbers: [f64; N]) -> Object {
    Object::Array(
        numbers
            .into_iter()
            .map(|number| Object::Real(number as f32))
            .collect(),
    )
}
