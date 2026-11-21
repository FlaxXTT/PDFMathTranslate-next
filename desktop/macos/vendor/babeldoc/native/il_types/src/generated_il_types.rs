// Generated from babeldoc/format/pdf/document_il/il_version_1.py; do not edit.
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct BaseOperations {
    pub value: String,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct Box {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub x2: Option<f64>,
    pub y2: Option<f64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct GraphicState {
    // Runtime override: the field is an eager string or a lazy persistent-log view; evidence: babeldoc/format/pdf/document_il/frontend/il_creater_active.py:1008, babeldoc/format/pdf/document_il/frontend/il_creater_active_support.py:53, runtime-type evidence:82
    pub passthrough_per_char_instruction: Option<crate::PassthroughValue>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfAffineTransform {
    pub translation_x: Option<f64>,
    pub translation_y: Option<f64>,
    pub rotation: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub shear: Option<f64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfFontCharBoundingBox {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub x2: Option<f64>,
    pub y2: Option<f64>,
    pub char_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfInlineForm {
    pub form_data: Option<String>,
    pub image_parameters: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfMatrix {
    pub a: Option<f64>,
    pub b: Option<f64>,
    pub c: Option<f64>,
    pub d: Option<f64>,
    pub e: Option<f64>,
    pub f: Option<f64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfPath {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub op: Option<String>,
    pub has_xy: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfXobjForm {
    pub xref_id: Option<i64>,
    pub do_args: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct Cropbox {
    pub r#box: Option<std::boxed::Box<Box>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct Mediabox {
    pub r#box: Option<std::boxed::Box<Box>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PageLayout {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub id: Option<i64>,
    pub conf: Option<f64>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfFigure {
    pub r#box: Option<std::boxed::Box<Box>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfFont {
    pub pdf_font_char_bounding_box: Vec<PdfFontCharBoundingBox>,
    pub name: Option<String>,
    pub font_id: Option<String>,
    pub xref_id: Option<i64>,
    pub encoding_length: Option<i64>,
    pub font_subtype: Option<String>,
    pub type3_font_matrix: Option<String>,
    pub type3_font_bbox: Option<String>,
    pub type3_em_height: Option<f64>,
    // Runtime override: PyMuPDF font flags are runtime integers 0/1; evidence: babeldoc/format/pdf/document_il/utils/fontmap.py:300, runtime-type evidence:413
    pub bold: Option<crate::RuntimeBool>,
    // Runtime override: PyMuPDF font flags are runtime integers 0/1; evidence: babeldoc/format/pdf/document_il/utils/fontmap.py:301, runtime-type evidence:453
    pub italic: Option<crate::RuntimeBool>,
    // Runtime override: PyMuPDF font flags are runtime integers 0/1; evidence: babeldoc/format/pdf/document_il/utils/fontmap.py:302, runtime-type evidence:493
    pub monospace: Option<crate::RuntimeBool>,
    // Runtime override: PyMuPDF font flags are runtime integers 0/1; evidence: babeldoc/format/pdf/document_il/utils/fontmap.py:303, runtime-type evidence:530
    pub serif: Option<crate::RuntimeBool>,
    pub ascent: Option<f64>,
    pub descent: Option<f64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfFormSubtype {
    pub pdf_inline_form: Option<std::boxed::Box<PdfInlineForm>>,
    pub pdf_xobj_form: Option<std::boxed::Box<PdfXobjForm>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfOriginalPath {
    pub pdf_path: Option<std::boxed::Box<PdfPath>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfOriginalPathPrimitive {
    pub op: Option<String>,
    pub args: Vec<f64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfRectangle {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub graphic_state: Option<crate::Shared<GraphicState>>,
    pub debug_info: Option<bool>,
    pub fill_background: Option<bool>,
    pub xobj_id: Option<i64>,
    pub line_width: Option<f64>,
    pub render_order: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfStyle {
    pub graphic_state: Option<crate::Shared<GraphicState>>,
    pub font_id: Option<String>,
    pub font_size: Option<f64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct VisualBbox {
    pub r#box: Option<std::boxed::Box<Box>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfCharacter {
    pub pdf_style: Option<std::boxed::Box<PdfStyle>>,
    pub r#box: Option<std::boxed::Box<Box>>,
    pub visual_bbox: Option<std::boxed::Box<VisualBbox>>,
    pub vertical: Option<bool>,
    pub scale: Option<f64>,
    pub visual_roundtrip_text_scale_x: Option<f64>,
    pub visual_roundtrip_text_scale_y: Option<f64>,
    pub visual_roundtrip_text_render_mode: Option<i64>,
    pub pdf_character_id: Option<i64>,
    pub char_unicode: Option<String>,
    pub advance: Option<f64>,
    pub xobj_id: Option<i64>,
    pub debug_info: Option<bool>,
    pub formula_layout_id: Option<i64>,
    pub render_order: Option<i64>,
    pub sub_render_order: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfCurve {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub graphic_state: Option<crate::Shared<GraphicState>>,
    pub pdf_path: Vec<PdfPath>,
    pub pdf_original_path: Vec<PdfOriginalPath>,
    pub pdf_original_path_primitive: Option<std::boxed::Box<PdfOriginalPathPrimitive>>,
    // Runtime override: the active parser emits both a string marker and a bool; evidence: babeldoc/format/pdf/document_il/frontend/il_creater_active.py:1257, babeldoc/format/pdf/document_il/frontend/il_creater_active.py:1285, runtime-type evidence:45
    pub debug_info: Option<crate::CurveDebugInfo>,
    pub fill_background: Option<bool>,
    pub stroke_path: Option<bool>,
    pub evenodd: Option<bool>,
    pub passthrough_paint: Option<bool>,
    pub xobj_id: Option<i64>,
    pub render_order: Option<i64>,
    pub ctm: Vec<crate::PythonValue>,
    pub relocation_transform: Vec<crate::PythonValue>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfForm {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub graphic_state: Option<crate::Shared<GraphicState>>,
    pub pdf_matrix: Option<std::boxed::Box<PdfMatrix>>,
    pub pdf_affine_transform: Option<std::boxed::Box<PdfAffineTransform>>,
    pub pdf_form_subtype: Option<std::boxed::Box<PdfFormSubtype>>,
    pub xobj_id: Option<i64>,
    pub ctm: Vec<crate::PythonValue>,
    pub relocation_transform: Vec<crate::PythonValue>,
    pub render_order: Option<i64>,
    pub form_type: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfSameStyleUnicodeCharacters {
    pub pdf_style: Option<std::boxed::Box<PdfStyle>>,
    pub unicode: Option<String>,
    pub debug_info: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfXobject {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub pdf_font: Vec<crate::Shared<PdfFont>>,
    pub base_operations: Option<std::boxed::Box<BaseOperations>>,
    pub xobj_id: Option<i64>,
    pub xref_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfFormula {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub pdf_character: Vec<PdfCharacter>,
    pub pdf_curve: Vec<crate::Shared<PdfCurve>>,
    pub pdf_form: Vec<crate::Shared<PdfForm>>,
    pub x_offset: Option<f64>,
    pub y_offset: Option<f64>,
    pub x_advance: Option<f64>,
    pub line_id: Option<i64>,
    pub is_corner_mark: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfLine {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub pdf_character: Vec<PdfCharacter>,
    pub render_order: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfSameStyleCharacters {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub pdf_style: Option<std::boxed::Box<PdfStyle>>,
    pub pdf_character: Vec<PdfCharacter>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfParagraphComposition {
    pub pdf_line: Option<std::boxed::Box<PdfLine>>,
    pub pdf_formula: Option<std::boxed::Box<PdfFormula>>,
    pub pdf_same_style_characters: Option<std::boxed::Box<PdfSameStyleCharacters>>,
    pub pdf_character: Option<std::boxed::Box<PdfCharacter>>,
    pub pdf_same_style_unicode_characters: Option<std::boxed::Box<PdfSameStyleUnicodeCharacters>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct PdfParagraph {
    pub r#box: Option<std::boxed::Box<Box>>,
    pub pdf_style: Option<std::boxed::Box<PdfStyle>>,
    pub pdf_paragraph_composition: Vec<PdfParagraphComposition>,
    pub xobj_id: Option<i64>,
    pub unicode: Option<String>,
    pub scale: Option<f64>,
    pub optimal_scale: Option<f64>,
    pub vertical: Option<bool>,
    pub first_line_indent: Option<bool>,
    pub debug_id: Option<String>,
    pub layout_label: Option<String>,
    pub layout_id: Option<i64>,
    pub render_order: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(default)]
pub struct Page {
    pub mediabox: Option<std::boxed::Box<Mediabox>>,
    pub cropbox: Option<std::boxed::Box<Cropbox>>,
    pub pdf_xobject: Vec<PdfXobject>,
    pub page_layout: Vec<PageLayout>,
    pub pdf_rectangle: Vec<PdfRectangle>,
    pub pdf_font: Vec<crate::Shared<PdfFont>>,
    pub pdf_paragraph: Vec<PdfParagraph>,
    pub pdf_figure: Vec<PdfFigure>,
    pub pdf_character: Vec<PdfCharacter>,
    pub pdf_curve: Vec<crate::Shared<PdfCurve>>,
    pub pdf_form: Vec<crate::Shared<PdfForm>>,
    pub base_operations: Option<std::boxed::Box<BaseOperations>>,
    pub page_number: Option<i64>,
    pub unit: Option<String>,
}

pub const IL_CLASS_COUNT: usize = 31;
pub const IL_FIELD_COUNT: usize = 164;
pub const IL_RUNTIME_OVERRIDE_COUNT: usize = 6;
