use std::collections::{BTreeSet, HashMap};

use babeldoc_writer_core::{FontInstallResult, PageOutput};
use lopdf::{Dictionary, Document, Object, ObjectId, Stream, content::Content};
use subsetter::{GlyphRemapper, subset};

pub struct FontSubset {
    resources: HashMap<Vec<u8>, u32>,
    glyphs: HashMap<u32, BTreeSet<u16>>,
    complete: bool,
}

impl FontSubset {
    pub fn new(first_new_id: u32, fonts: &FontInstallResult) -> Self {
        let resources = fonts
            .installed
            .iter()
            .filter_map(|font| {
                let id = u32::try_from(font.xref_id).ok()?;
                (id >= first_new_id).then(|| (font.font_id.as_bytes().to_vec(), id))
            })
            .collect();
        Self {
            resources,
            glyphs: HashMap::new(),
            complete: true,
        }
    }

    pub fn record(&mut self, output: &PageOutput) {
        if !self.complete || self.resources.is_empty() {
            return;
        }
        self.complete = self.record_stream(&output.stream).is_some()
            && output
                .forms
                .iter()
                .all(|form| self.record_stream(&form.stream).is_some());
    }

    fn record_stream(&mut self, stream: &[u8]) -> Option<()> {
        let content = Content::decode_strict(stream).ok()?;
        let mut font = None;
        let mut states = Vec::new();
        for operation in content.operations {
            match operation.operator.as_str() {
                "q" => states.push(font),
                "Q" => font = states.pop()?,
                "Tf" => {
                    let resource = operation.operands.first()?.as_name().ok()?;
                    font = self.resources.get(resource).copied();
                }
                "Tj" | "'" | "\"" => {
                    if let Some(id) = font {
                        self.record_string(id, operation.operands.last()?)?;
                    }
                }
                "TJ" => {
                    if let Some(id) = font {
                        for item in operation.operands.first()?.as_array().ok()? {
                            match item {
                                Object::String(..) => self.record_string(id, item)?,
                                Object::Integer(_) | Object::Real(_) => (),
                                _ => return None,
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        states.is_empty().then_some(())
    }

    fn record_string(&mut self, font: u32, value: &Object) -> Option<()> {
        let bytes = value.as_str().ok()?;
        let (codes, overflow) = bytes.as_chunks::<2>();
        if !overflow.is_empty() {
            return None;
        }
        self.glyphs
            .entry(font)
            .or_default()
            .extend(codes.iter().copied().map(u16::from_be_bytes));
        Some(())
    }

    pub fn apply(self, document: &mut Document) {
        if self.complete {
            for (id, glyphs) in self.glyphs {
                if let Some(font) = SubsetFont::build(document, (id, 0), glyphs) {
                    font.replace(document);
                }
            }
        }
    }
}

struct SubsetFont {
    parent: ObjectId,
    descendant: ObjectId,
    descriptor: ObjectId,
    file: ObjectId,
    data: Vec<u8>,
    cid_to_gid: Vec<u8>,
    name: Object,
}

impl SubsetFont {
    fn build(document: &Document, parent: ObjectId, glyphs: BTreeSet<u16>) -> Option<Self> {
        let font = document.get_dictionary(parent).ok()?;
        if font.get(b"Subtype").ok()?.as_name().ok()? != b"Type0"
            || font.get(b"Encoding").ok()?.as_name().ok()? != b"Identity-H"
        {
            return None;
        }
        let [descendant] = font
            .get(b"DescendantFonts")
            .ok()?
            .as_array()
            .ok()?
            .as_slice()
        else {
            return None;
        };
        let descendant = descendant.as_reference().ok()?;
        let cid_font = document.get_dictionary(descendant).ok()?;
        if cid_font.get(b"Subtype").ok()?.as_name().ok()? != b"CIDFontType2"
            || cid_font.has(b"CIDToGIDMap")
        {
            return None;
        }
        let descriptor = reference(cid_font, b"FontDescriptor")?;
        let file = reference(document.get_dictionary(descriptor).ok()?, b"FontFile2")?;
        let stream = document.get_object(file).ok()?.as_stream().ok()?;
        let original = stream.get_plain_content().ok()?;
        if !original.starts_with(&[0, 1, 0, 0]) {
            return None;
        }
        let glyphs: Vec<_> = glyphs.into_iter().collect();
        let remapper = GlyphRemapper::new_from_glyphs(&glyphs);
        let data = subset(&original, 0, &remapper).ok()?;
        let mut cid_to_gid = vec![0; (usize::from(*glyphs.last()?) + 1) * 2];
        for cid in glyphs {
            let gid = remapper.get(cid)?;
            let start = usize::from(cid) * 2;
            cid_to_gid[start..start + 2].copy_from_slice(&gid.to_be_bytes());
        }
        if data.len() + cid_to_gid.len() >= original.len() {
            return None;
        }
        let mut tag = [b'A'; 6];
        let mut remainder = parent.0;
        for letter in tag.iter_mut().rev() {
            *letter += (remainder % 26) as u8;
            remainder /= 26;
        }
        let mut name = tag.to_vec();
        name.push(b'+');
        name.extend_from_slice(cid_font.get(b"BaseFont").ok()?.as_name().ok()?);
        Some(Self {
            parent,
            descendant,
            descriptor,
            file,
            data,
            cid_to_gid,
            name: Object::Name(name),
        })
    }

    fn replace(self, document: &mut Document) {
        // PDF text, metrics, and ToUnicode still address the original CIDs.
        // The map translates those CIDs to the compacted TrueType glyph IDs.
        let map = document.add_object(Stream::new(Dictionary::new(), self.cid_to_gid));
        if let Ok(font) = document.get_dictionary_mut(self.descendant) {
            font.set("CIDToGIDMap", Object::Reference(map));
            font.set("BaseFont", self.name.clone());
        }
        if let Ok(parent) = document.get_dictionary_mut(self.parent) {
            parent.set("BaseFont", self.name.clone());
        }
        if let Ok(descriptor) = document.get_dictionary_mut(self.descriptor) {
            descriptor.set("FontName", self.name);
        }
        if let Ok(object) = document.get_object_mut(self.file)
            && let Ok(stream) = object.as_stream_mut()
        {
            stream.dict.set("Length1", self.data.len() as i64);
            stream.set_plain_content(self.data);
        }
    }
}

fn reference(dictionary: &Dictionary, key: &[u8]) -> Option<ObjectId> {
    dictionary.get(key).ok()?.as_reference().ok()
}
