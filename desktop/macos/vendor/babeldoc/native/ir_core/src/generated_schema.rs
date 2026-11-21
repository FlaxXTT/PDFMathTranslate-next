// Generated owned tree and document-table accessors from the IL schema.
impl<P:Permission> TreeView<'_,P> {pub fn instruction_text(&self,value:&Option<Instructions>)->Result<Option<String>,AccessError> {render_instructions(value,&self.document_ref().history)}}
pub const CLASS_COUNT:usize=32;
#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)] pub enum Kind {
BaseOperations,
Box,
GraphicState,
PdfAffineTransform,
PdfFontCharBoundingBox,
PdfInlineForm,
PdfMatrix,
PdfPath,
PdfXobjForm,
Cropbox,
Mediabox,
PageLayout,
PdfFigure,
PdfFont,
PdfFormSubtype,
PdfOriginalPath,
PdfOriginalPathPrimitive,
PdfRectangle,
PdfStyle,
VisualBbox,
PdfCharacter,
PdfCurve,
PdfForm,
PdfSameStyleUnicodeCharacters,
PdfXobject,
PdfFormula,
PdfLine,
PdfSameStyleCharacters,
PdfParagraphComposition,
PdfParagraph,
Page,
Document,
}
impl Kind {pub fn name(self)->&'static str {match self {
Self::BaseOperations=>"BaseOperations",
Self::Box=>"Box",
Self::GraphicState=>"GraphicState",
Self::PdfAffineTransform=>"PdfAffineTransform",
Self::PdfFontCharBoundingBox=>"PdfFontCharBoundingBox",
Self::PdfInlineForm=>"PdfInlineForm",
Self::PdfMatrix=>"PdfMatrix",
Self::PdfPath=>"PdfPath",
Self::PdfXobjForm=>"PdfXobjForm",
Self::Cropbox=>"Cropbox",
Self::Mediabox=>"Mediabox",
Self::PageLayout=>"PageLayout",
Self::PdfFigure=>"PdfFigure",
Self::PdfFont=>"PdfFont",
Self::PdfFormSubtype=>"PdfFormSubtype",
Self::PdfOriginalPath=>"PdfOriginalPath",
Self::PdfOriginalPathPrimitive=>"PdfOriginalPathPrimitive",
Self::PdfRectangle=>"PdfRectangle",
Self::PdfStyle=>"PdfStyle",
Self::VisualBbox=>"VisualBbox",
Self::PdfCharacter=>"PdfCharacter",
Self::PdfCurve=>"PdfCurve",
Self::PdfForm=>"PdfForm",
Self::PdfSameStyleUnicodeCharacters=>"PdfSameStyleUnicodeCharacters",
Self::PdfXobject=>"PdfXobject",
Self::PdfFormula=>"PdfFormula",
Self::PdfLine=>"PdfLine",
Self::PdfSameStyleCharacters=>"PdfSameStyleCharacters",
Self::PdfParagraphComposition=>"PdfParagraphComposition",
Self::PdfParagraph=>"PdfParagraph",
Self::Page=>"Page",
Self::Document=>"Document",
}} pub fn is_pool(self)->bool {match self {
Self::GraphicState=>true,
Self::PdfFont=>true,
Self::PdfFontCharBoundingBox=>true,
_=>false}} pub fn is_table(self)->bool {match self {
Self::Box=>true,
Self::GraphicState=>true,
Self::Page=>true,
Self::PageLayout=>true,
Self::PdfCharacter=>true,
Self::PdfCurve=>true,
Self::PdfFont=>true,
Self::PdfFontCharBoundingBox=>true,
Self::PdfForm=>true,
Self::PdfFormula=>true,
Self::PdfLine=>true,
Self::PdfParagraph=>true,
Self::PdfParagraphComposition=>true,
Self::PdfRectangle=>true,
Self::PdfSameStyleCharacters=>true,
Self::PdfSameStyleUnicodeCharacters=>true,
Self::PdfStyle=>true,
Self::PdfXobject=>true,
Self::VisualBbox=>true,
_=>false}}}
pub enum Edge {
Inline(std::boxed::Box<Node>),Shared(usize),
ImmutableBaseOperations(Arc<ImmutableNode<DataBaseOperations>>),
ImmutableBox(Arc<ImmutableNode<DataBox>>),
ImmutableGraphicState(Arc<ImmutableNode<DataGraphicState>>),
ImmutablePdfAffineTransform(Arc<ImmutableNode<DataPdfAffineTransform>>),
ImmutablePdfFontCharBoundingBox(Arc<ImmutableNode<DataPdfFontCharBoundingBox>>),
ImmutablePdfInlineForm(Arc<ImmutableNode<DataPdfInlineForm>>),
ImmutablePdfMatrix(Arc<ImmutableNode<DataPdfMatrix>>),
ImmutablePdfPath(Arc<ImmutableNode<DataPdfPath>>),
ImmutablePdfXobjForm(Arc<ImmutableNode<DataPdfXobjForm>>),
ImmutableCropbox(Arc<ImmutableNode<DataCropbox>>),
ImmutableMediabox(Arc<ImmutableNode<DataMediabox>>),
ImmutablePageLayout(Arc<ImmutableNode<DataPageLayout>>),
ImmutablePdfFigure(Arc<ImmutableNode<DataPdfFigure>>),
ImmutablePdfFont(Arc<ImmutableNode<DataPdfFont>>),
ImmutablePdfFormSubtype(Arc<ImmutableNode<DataPdfFormSubtype>>),
ImmutablePdfOriginalPath(Arc<ImmutableNode<DataPdfOriginalPath>>),
ImmutablePdfOriginalPathPrimitive(Arc<ImmutableNode<DataPdfOriginalPathPrimitive>>),
ImmutablePdfRectangle(Arc<ImmutableNode<DataPdfRectangle>>),
ImmutablePdfStyle(Arc<ImmutableNode<DataPdfStyle>>),
ImmutableVisualBbox(Arc<ImmutableNode<DataVisualBbox>>),
ImmutablePdfCharacter(Arc<ImmutableNode<DataPdfCharacter>>),
ImmutablePdfCurve(Arc<ImmutableNode<DataPdfCurve>>),
ImmutablePdfForm(Arc<ImmutableNode<DataPdfForm>>),
ImmutablePdfSameStyleUnicodeCharacters(Arc<ImmutableNode<DataPdfSameStyleUnicodeCharacters>>),
ImmutablePdfXobject(Arc<ImmutableNode<DataPdfXobject>>),
ImmutablePdfFormula(Arc<ImmutableNode<DataPdfFormula>>),
ImmutablePdfLine(Arc<ImmutableNode<DataPdfLine>>),
ImmutablePdfSameStyleCharacters(Arc<ImmutableNode<DataPdfSameStyleCharacters>>),
ImmutablePdfParagraphComposition(Arc<ImmutableNode<DataPdfParagraphComposition>>),
ImmutablePdfParagraph(Arc<ImmutableNode<DataPdfParagraph>>),
ImmutablePage(Arc<ImmutableNode<DataPage>>),
ImmutableDocument(Arc<ImmutableNode<DataDocument>>),
}
pub mod types {use super::{Arc,Edge,ImmutableNode,Kind,SchemaType,Document as OwnedDocument,AccessError,ROOT};
#[derive(Debug)] pub enum BaseOperations {}
impl SchemaType for BaseOperations {const KIND:Kind=Kind::BaseOperations;const NAME:&'static str="BaseOperations";type Data=super::DataBaseOperations;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutableBaseOperations(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutableBaseOperations(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum Box {}
impl SchemaType for Box {const KIND:Kind=Kind::Box;const NAME:&'static str="Box";type Data=super::DataBox;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutableBox(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutableBox(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::Box{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.r#box[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::Box{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.r#box[location.index].data)}
}
#[derive(Debug)] pub enum GraphicState {}
impl SchemaType for GraphicState {const KIND:Kind=Kind::GraphicState;const NAME:&'static str="GraphicState";type Data=super::DataGraphicState;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutableGraphicState(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutableGraphicState(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::GraphicState{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.graphicstate[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::GraphicState{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.graphicstate[location.index].data)}
}
#[derive(Debug)] pub enum PdfAffineTransform {}
impl SchemaType for PdfAffineTransform {const KIND:Kind=Kind::PdfAffineTransform;const NAME:&'static str="PdfAffineTransform";type Data=super::DataPdfAffineTransform;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfAffineTransform(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfAffineTransform(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfFontCharBoundingBox {}
impl SchemaType for PdfFontCharBoundingBox {const KIND:Kind=Kind::PdfFontCharBoundingBox;const NAME:&'static str="PdfFontCharBoundingBox";type Data=super::DataPdfFontCharBoundingBox;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfFontCharBoundingBox(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfFontCharBoundingBox(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfFontCharBoundingBox{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdffontcharboundingbox[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfFontCharBoundingBox{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdffontcharboundingbox[location.index].data)}
}
#[derive(Debug)] pub enum PdfInlineForm {}
impl SchemaType for PdfInlineForm {const KIND:Kind=Kind::PdfInlineForm;const NAME:&'static str="PdfInlineForm";type Data=super::DataPdfInlineForm;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfInlineForm(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfInlineForm(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfMatrix {}
impl SchemaType for PdfMatrix {const KIND:Kind=Kind::PdfMatrix;const NAME:&'static str="PdfMatrix";type Data=super::DataPdfMatrix;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfMatrix(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfMatrix(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfPath {}
impl SchemaType for PdfPath {const KIND:Kind=Kind::PdfPath;const NAME:&'static str="PdfPath";type Data=super::DataPdfPath;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfPath(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfPath(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfXobjForm {}
impl SchemaType for PdfXobjForm {const KIND:Kind=Kind::PdfXobjForm;const NAME:&'static str="PdfXobjForm";type Data=super::DataPdfXobjForm;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfXobjForm(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfXobjForm(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum Cropbox {}
impl SchemaType for Cropbox {const KIND:Kind=Kind::Cropbox;const NAME:&'static str="Cropbox";type Data=super::DataCropbox;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutableCropbox(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutableCropbox(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum Mediabox {}
impl SchemaType for Mediabox {const KIND:Kind=Kind::Mediabox;const NAME:&'static str="Mediabox";type Data=super::DataMediabox;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutableMediabox(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutableMediabox(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PageLayout {}
impl SchemaType for PageLayout {const KIND:Kind=Kind::PageLayout;const NAME:&'static str="PageLayout";type Data=super::DataPageLayout;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePageLayout(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePageLayout(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PageLayout{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pagelayout[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PageLayout{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pagelayout[location.index].data)}
}
#[derive(Debug)] pub enum PdfFigure {}
impl SchemaType for PdfFigure {const KIND:Kind=Kind::PdfFigure;const NAME:&'static str="PdfFigure";type Data=super::DataPdfFigure;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfFigure(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfFigure(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfFont {}
impl SchemaType for PdfFont {const KIND:Kind=Kind::PdfFont;const NAME:&'static str="PdfFont";type Data=super::DataPdfFont;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfFont(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfFont(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfFont{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdffont[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfFont{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdffont[location.index].data)}
}
#[derive(Debug)] pub enum PdfFormSubtype {}
impl SchemaType for PdfFormSubtype {const KIND:Kind=Kind::PdfFormSubtype;const NAME:&'static str="PdfFormSubtype";type Data=super::DataPdfFormSubtype;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfFormSubtype(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfFormSubtype(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfOriginalPath {}
impl SchemaType for PdfOriginalPath {const KIND:Kind=Kind::PdfOriginalPath;const NAME:&'static str="PdfOriginalPath";type Data=super::DataPdfOriginalPath;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfOriginalPath(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfOriginalPath(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfOriginalPathPrimitive {}
impl SchemaType for PdfOriginalPathPrimitive {const KIND:Kind=Kind::PdfOriginalPathPrimitive;const NAME:&'static str="PdfOriginalPathPrimitive";type Data=super::DataPdfOriginalPathPrimitive;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfOriginalPathPrimitive(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfOriginalPathPrimitive(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let _=(doc,index);Err(AccessError::WrongType)}
}
#[derive(Debug)] pub enum PdfRectangle {}
impl SchemaType for PdfRectangle {const KIND:Kind=Kind::PdfRectangle;const NAME:&'static str="PdfRectangle";type Data=super::DataPdfRectangle;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfRectangle(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfRectangle(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfRectangle{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfrectangle[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfRectangle{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfrectangle[location.index].data)}
}
#[derive(Debug)] pub enum PdfStyle {}
impl SchemaType for PdfStyle {const KIND:Kind=Kind::PdfStyle;const NAME:&'static str="PdfStyle";type Data=super::DataPdfStyle;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfStyle(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfStyle(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfStyle{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfstyle[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfStyle{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfstyle[location.index].data)}
}
#[derive(Debug)] pub enum VisualBbox {}
impl SchemaType for VisualBbox {const KIND:Kind=Kind::VisualBbox;const NAME:&'static str="VisualBbox";type Data=super::DataVisualBbox;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutableVisualBbox(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutableVisualBbox(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::VisualBbox{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.visualbbox[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::VisualBbox{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.visualbbox[location.index].data)}
}
#[derive(Debug)] pub enum PdfCharacter {}
impl SchemaType for PdfCharacter {const KIND:Kind=Kind::PdfCharacter;const NAME:&'static str="PdfCharacter";type Data=super::DataPdfCharacter;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfCharacter(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfCharacter(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfCharacter{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfcharacter[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfCharacter{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfcharacter[location.index].data)}
}
#[derive(Debug)] pub enum PdfCurve {}
impl SchemaType for PdfCurve {const KIND:Kind=Kind::PdfCurve;const NAME:&'static str="PdfCurve";type Data=super::DataPdfCurve;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfCurve(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfCurve(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfCurve{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfcurve[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfCurve{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfcurve[location.index].data)}
}
#[derive(Debug)] pub enum PdfForm {}
impl SchemaType for PdfForm {const KIND:Kind=Kind::PdfForm;const NAME:&'static str="PdfForm";type Data=super::DataPdfForm;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfForm(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfForm(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfForm{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfform[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfForm{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfform[location.index].data)}
}
#[derive(Debug)] pub enum PdfSameStyleUnicodeCharacters {}
impl SchemaType for PdfSameStyleUnicodeCharacters {const KIND:Kind=Kind::PdfSameStyleUnicodeCharacters;const NAME:&'static str="PdfSameStyleUnicodeCharacters";type Data=super::DataPdfSameStyleUnicodeCharacters;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfSameStyleUnicodeCharacters(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfSameStyleUnicodeCharacters(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfSameStyleUnicodeCharacters{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfsamestyleunicodecharacters[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfSameStyleUnicodeCharacters{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfsamestyleunicodecharacters[location.index].data)}
}
#[derive(Debug)] pub enum PdfXobject {}
impl SchemaType for PdfXobject {const KIND:Kind=Kind::PdfXobject;const NAME:&'static str="PdfXobject";type Data=super::DataPdfXobject;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfXobject(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfXobject(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfXobject{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfxobject[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfXobject{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfxobject[location.index].data)}
}
#[derive(Debug)] pub enum PdfFormula {}
impl SchemaType for PdfFormula {const KIND:Kind=Kind::PdfFormula;const NAME:&'static str="PdfFormula";type Data=super::DataPdfFormula;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfFormula(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfFormula(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfFormula{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfformula[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfFormula{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfformula[location.index].data)}
}
#[derive(Debug)] pub enum PdfLine {}
impl SchemaType for PdfLine {const KIND:Kind=Kind::PdfLine;const NAME:&'static str="PdfLine";type Data=super::DataPdfLine;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfLine(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfLine(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfLine{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfline[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfLine{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfline[location.index].data)}
}
#[derive(Debug)] pub enum PdfSameStyleCharacters {}
impl SchemaType for PdfSameStyleCharacters {const KIND:Kind=Kind::PdfSameStyleCharacters;const NAME:&'static str="PdfSameStyleCharacters";type Data=super::DataPdfSameStyleCharacters;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfSameStyleCharacters(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfSameStyleCharacters(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfSameStyleCharacters{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfsamestylecharacters[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfSameStyleCharacters{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfsamestylecharacters[location.index].data)}
}
#[derive(Debug)] pub enum PdfParagraphComposition {}
impl SchemaType for PdfParagraphComposition {const KIND:Kind=Kind::PdfParagraphComposition;const NAME:&'static str="PdfParagraphComposition";type Data=super::DataPdfParagraphComposition;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfParagraphComposition(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfParagraphComposition(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfParagraphComposition{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfparagraphcomposition[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfParagraphComposition{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfparagraphcomposition[location.index].data)}
}
#[derive(Debug)] pub enum PdfParagraph {}
impl SchemaType for PdfParagraph {const KIND:Kind=Kind::PdfParagraph;const NAME:&'static str="PdfParagraph";type Data=super::DataPdfParagraph;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePdfParagraph(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePdfParagraph(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfParagraph{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.pdfparagraph[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::PdfParagraph{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.pdfparagraph[location.index].data)}
}
#[derive(Debug)] pub enum Page {}
impl SchemaType for Page {const KIND:Kind=Kind::Page;const NAME:&'static str="Page";type Data=super::DataPage;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutablePage(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutablePage(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::Page{return Err(AccessError::WrongType)};let tables=doc.table.scope(location.scope);Ok(& tables.page[location.index].data)}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{let location=*doc.table.nodes.get(index).ok_or(AccessError::UnknownSlot)?;if location.kind!=Kind::Page{return Err(AccessError::WrongType)};let tables=doc.table.scope_mut(location.scope);Ok(&mut tables.page[location.index].data)}
}
#[derive(Debug)] pub enum Document {}
impl SchemaType for Document {const KIND:Kind=Kind::Document;const NAME:&'static str="Document";type Data=super::DataDocument;fn from_immutable_edge(edge:&Edge)->Option<&Arc<ImmutableNode<Self::Data>>>{if let Edge::ImmutableDocument(value)=edge{Some(value)}else{None}}fn into_immutable_edge(value:Arc<ImmutableNode<Self::Data>>)->Edge{Edge::ImmutableDocument(value)}
fn data(doc:& OwnedDocument,index:usize)->Result<& Self::Data,AccessError>{if index!=ROOT{return Err(AccessError::WrongType)};if let super::NodeData::Document(n)=& doc.root.data{Ok(n)}else{Err(AccessError::WrongType)}}
fn data_mut(doc:&mut OwnedDocument,index:usize)->Result<&mut Self::Data,AccessError>{if index!=ROOT{return Err(AccessError::WrongType)};if let super::NodeData::Document(n)=&mut doc.root.data{Ok(n)}else{Err(AccessError::WrongType)}}
}
}
pub enum NodeData {
BaseOperations(DataBaseOperations),
Box(DataBox),
GraphicState(DataGraphicState),
PdfAffineTransform(DataPdfAffineTransform),
PdfFontCharBoundingBox(DataPdfFontCharBoundingBox),
PdfInlineForm(DataPdfInlineForm),
PdfMatrix(DataPdfMatrix),
PdfPath(DataPdfPath),
PdfXobjForm(DataPdfXobjForm),
Cropbox(DataCropbox),
Mediabox(DataMediabox),
PageLayout(DataPageLayout),
PdfFigure(DataPdfFigure),
PdfFont(DataPdfFont),
PdfFormSubtype(DataPdfFormSubtype),
PdfOriginalPath(DataPdfOriginalPath),
PdfOriginalPathPrimitive(DataPdfOriginalPathPrimitive),
PdfRectangle(DataPdfRectangle),
PdfStyle(DataPdfStyle),
VisualBbox(DataVisualBbox),
PdfCharacter(DataPdfCharacter),
PdfCurve(DataPdfCurve),
PdfForm(DataPdfForm),
PdfSameStyleUnicodeCharacters(DataPdfSameStyleUnicodeCharacters),
PdfXobject(DataPdfXobject),
PdfFormula(DataPdfFormula),
PdfLine(DataPdfLine),
PdfSameStyleCharacters(DataPdfSameStyleCharacters),
PdfParagraphComposition(DataPdfParagraphComposition),
PdfParagraph(DataPdfParagraph),
Page(DataPage),
Document(DataDocument),
}
#[derive(Default)] pub struct DataBaseOperations {
pub value:String,
}
#[derive(Default)] pub struct BaseOperationsInit {
pub value:String,
}
impl BaseOperationsInit {pub fn into_value(self)->FieldValue<types::BaseOperations> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::BaseOperations(DataBaseOperations {
value:self.value,
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::BaseOperations> {
pub fn value<P:Permission>(&self,v:&TreeView<'_,P>)->Result<&'n String,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(&n.value)}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<BaseOperationsInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(BaseOperationsInit {
value:n.value.clone(),
})}}
#[derive(Default)] pub struct DataBox {
pub x:Option<f64>,
pub y:Option<f64>,
pub x2:Option<f64>,
pub y2:Option<f64>,
}
#[derive(Default)] pub struct BoxInit {
pub x:Option<f64>,
pub y:Option<f64>,
pub x2:Option<f64>,
pub y2:Option<f64>,
}
impl BoxInit {pub fn into_value(self)->FieldValue<types::Box> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::Box(DataBox {
x:self.x,
y:self.y,
x2:self.x2,
y2:self.y2,
}),binding)}}
impl Cursor<types::Box> {
pub fn x<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.x)}
pub fn set_x<P:Permission+Writes<types::Box>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.x=value;Ok(())}
pub fn y<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.y)}
pub fn set_y<P:Permission+Writes<types::Box>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.y=value;Ok(())}
pub fn x2<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.x2)}
pub fn set_x2<P:Permission+Writes<types::Box>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.x2=value;Ok(())}
pub fn y2<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.y2)}
pub fn set_y2<P:Permission+Writes<types::Box>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.y2=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<BoxInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(BoxInit {
x:n.x.clone(),
y:n.y.clone(),
x2:n.x2.clone(),
y2:n.y2.clone(),
})}}
#[derive(Default)] pub struct DataGraphicState {
pub passthrough_per_char_instruction:Option<Instructions>,
}
#[derive(Default)] pub struct GraphicStateInit {
pub passthrough_per_char_instruction:Option<Instructions>,
}
impl GraphicStateInit {pub fn into_value(self)->FieldValue<types::GraphicState> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::GraphicState(DataGraphicState {
passthrough_per_char_instruction:self.passthrough_per_char_instruction,
}),binding)}}
impl Cursor<types::GraphicState> {
pub fn passthrough_per_char_instruction<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<Instructions>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.passthrough_per_char_instruction)}
pub fn set_passthrough_per_char_instruction<P:Permission+Writes<types::GraphicState>>(&self,v:&mut TreeView<'_,P>,value:Option<Instructions>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.passthrough_per_char_instruction=value;Ok(())}
pub fn instruction_text<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<String>,AccessError>{render_instructions(self.passthrough_per_char_instruction(v)?,&v.document_ref().history)}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<GraphicStateInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(GraphicStateInit {
passthrough_per_char_instruction:n.passthrough_per_char_instruction.clone(),
})}}
#[derive(Default)] pub struct DataPdfAffineTransform {
pub translation_x:Option<f64>,
pub translation_y:Option<f64>,
pub rotation:Option<f64>,
pub scale_x:Option<f64>,
pub scale_y:Option<f64>,
pub shear:Option<f64>,
}
#[derive(Default)] pub struct PdfAffineTransformInit {
pub translation_x:Option<f64>,
pub translation_y:Option<f64>,
pub rotation:Option<f64>,
pub scale_x:Option<f64>,
pub scale_y:Option<f64>,
pub shear:Option<f64>,
}
impl PdfAffineTransformInit {pub fn into_value(self)->FieldValue<types::PdfAffineTransform> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::PdfAffineTransform(DataPdfAffineTransform {
translation_x:self.translation_x,
translation_y:self.translation_y,
rotation:self.rotation,
scale_x:self.scale_x,
scale_y:self.scale_y,
shear:self.shear,
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfAffineTransform> {
pub fn translation_x<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.translation_x)}
pub fn translation_y<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.translation_y)}
pub fn rotation<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.rotation)}
pub fn scale_x<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.scale_x)}
pub fn scale_y<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.scale_y)}
pub fn shear<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.shear)}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfAffineTransformInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfAffineTransformInit {
translation_x:n.translation_x.clone(),
translation_y:n.translation_y.clone(),
rotation:n.rotation.clone(),
scale_x:n.scale_x.clone(),
scale_y:n.scale_y.clone(),
shear:n.shear.clone(),
})}}
#[derive(Default)] pub struct DataPdfFontCharBoundingBox {
pub x:Option<f64>,
pub y:Option<f64>,
pub x2:Option<f64>,
pub y2:Option<f64>,
pub char_id:Option<i64>,
}
#[derive(Default)] pub struct PdfFontCharBoundingBoxInit {
pub x:Option<f64>,
pub y:Option<f64>,
pub x2:Option<f64>,
pub y2:Option<f64>,
pub char_id:Option<i64>,
}
impl PdfFontCharBoundingBoxInit {pub fn into_value(self)->FieldValue<types::PdfFontCharBoundingBox> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::PdfFontCharBoundingBox(DataPdfFontCharBoundingBox {
x:self.x,
y:self.y,
x2:self.x2,
y2:self.y2,
char_id:self.char_id,
}),binding)}}
impl Cursor<types::PdfFontCharBoundingBox> {
pub fn x<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.x)}
pub fn set_x<P:Permission+Writes<types::PdfFontCharBoundingBox>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.x=value;Ok(())}
pub fn y<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.y)}
pub fn set_y<P:Permission+Writes<types::PdfFontCharBoundingBox>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.y=value;Ok(())}
pub fn x2<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.x2)}
pub fn set_x2<P:Permission+Writes<types::PdfFontCharBoundingBox>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.x2=value;Ok(())}
pub fn y2<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.y2)}
pub fn set_y2<P:Permission+Writes<types::PdfFontCharBoundingBox>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.y2=value;Ok(())}
pub fn char_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.char_id)}
pub fn set_char_id<P:Permission+Writes<types::PdfFontCharBoundingBox>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.char_id=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfFontCharBoundingBoxInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfFontCharBoundingBoxInit {
x:n.x.clone(),
y:n.y.clone(),
x2:n.x2.clone(),
y2:n.y2.clone(),
char_id:n.char_id.clone(),
})}}
#[derive(Default)] pub struct DataPdfInlineForm {
pub form_data:Option<String>,
pub image_parameters:Option<String>,
}
#[derive(Default)] pub struct PdfInlineFormInit {
pub form_data:Option<String>,
pub image_parameters:Option<String>,
}
impl PdfInlineFormInit {pub fn into_value(self)->FieldValue<types::PdfInlineForm> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::PdfInlineForm(DataPdfInlineForm {
form_data:self.form_data,
image_parameters:self.image_parameters,
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfInlineForm> {
pub fn form_data<P:Permission>(&self,v:&TreeView<'_,P>)->Result<&'n Option<String>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(&n.form_data)}
pub fn image_parameters<P:Permission>(&self,v:&TreeView<'_,P>)->Result<&'n Option<String>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(&n.image_parameters)}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfInlineFormInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfInlineFormInit {
form_data:n.form_data.clone(),
image_parameters:n.image_parameters.clone(),
})}}
#[derive(Default)] pub struct DataPdfMatrix {
pub a:Option<f64>,
pub b:Option<f64>,
pub c:Option<f64>,
pub d:Option<f64>,
pub e:Option<f64>,
pub f:Option<f64>,
}
#[derive(Default)] pub struct PdfMatrixInit {
pub a:Option<f64>,
pub b:Option<f64>,
pub c:Option<f64>,
pub d:Option<f64>,
pub e:Option<f64>,
pub f:Option<f64>,
}
impl PdfMatrixInit {pub fn into_value(self)->FieldValue<types::PdfMatrix> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::PdfMatrix(DataPdfMatrix {
a:self.a,
b:self.b,
c:self.c,
d:self.d,
e:self.e,
f:self.f,
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfMatrix> {
pub fn a<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.a)}
pub fn b<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.b)}
pub fn c<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.c)}
pub fn d<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.d)}
pub fn e<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.e)}
pub fn f<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.f)}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfMatrixInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfMatrixInit {
a:n.a.clone(),
b:n.b.clone(),
c:n.c.clone(),
d:n.d.clone(),
e:n.e.clone(),
f:n.f.clone(),
})}}
#[derive(Default)] pub struct PdfPathInit {
pub x:Option<f64>,
pub y:Option<f64>,
pub op:Option<String>,
pub has_xy:Option<bool>,
}
impl PdfPathInit {pub fn into_value(self)->FieldValue<types::PdfPath> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::PdfPath(DataPdfPath::new(self.x,self.y,self.op.map(PathOp::from),self.has_xy)),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfPath> {
pub fn x<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.x())}
pub fn y<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.y())}
pub fn op<P:Permission>(&self,v:&TreeView<'_,P>)->Result<&'n Option<PathOp>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(&n.op)}
pub fn has_xy<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<bool>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.has_xy())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfPathInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfPathInit {
x:n.x().clone(),
y:n.y().clone(),
op:n.op.as_deref().map(str::to_owned),
has_xy:n.has_xy().clone(),
})}}
#[derive(Default)] pub struct DataPdfXobjForm {
pub xref_id:Option<i64>,
pub do_args:Option<String>,
}
#[derive(Default)] pub struct PdfXobjFormInit {
pub xref_id:Option<i64>,
pub do_args:Option<String>,
}
impl PdfXobjFormInit {pub fn into_value(self)->FieldValue<types::PdfXobjForm> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::PdfXobjForm(DataPdfXobjForm {
xref_id:self.xref_id,
do_args:self.do_args,
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfXobjForm> {
pub fn xref_id<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<i64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(n.xref_id)}
pub fn do_args<P:Permission>(&self,v:&TreeView<'_,P>)->Result<&'n Option<String>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(&n.do_args)}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfXobjFormInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfXobjFormInit {
xref_id:n.xref_id.clone(),
do_args:n.do_args.clone(),
})}}
#[derive(Default)] pub struct DataCropbox {
pub r#box:Option<Edge>,
}
#[derive(Default)] pub struct CropboxInit {
pub r#box:Option<FieldValue<types::Box>>,
}
impl CropboxInit {pub fn into_value(self)->FieldValue<types::Cropbox> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
FieldValue::new(NodeData::Cropbox(DataCropbox {
r#box:self.r#box.map(|v|v.edge),
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::Cropbox> {
pub fn r#box<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<CropboxInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(CropboxInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
})}}
#[derive(Default)] pub struct DataMediabox {
pub r#box:Option<Edge>,
}
#[derive(Default)] pub struct MediaboxInit {
pub r#box:Option<FieldValue<types::Box>>,
}
impl MediaboxInit {pub fn into_value(self)->FieldValue<types::Mediabox> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
FieldValue::new(NodeData::Mediabox(DataMediabox {
r#box:self.r#box.map(|v|v.edge),
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::Mediabox> {
pub fn r#box<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<MediaboxInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(MediaboxInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
})}}
#[derive(Default)] pub struct DataPageLayout {
pub r#box:Option<Edge>,
pub id:Option<i64>,
pub conf:Option<f64>,
pub class_name:Option<String>,
}
#[derive(Default)] pub struct PageLayoutInit {
pub r#box:Option<FieldValue<types::Box>>,
pub id:Option<i64>,
pub conf:Option<f64>,
pub class_name:Option<String>,
}
impl PageLayoutInit {pub fn into_value(self)->FieldValue<types::PageLayout> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
FieldValue::new(NodeData::PageLayout(DataPageLayout {
r#box:self.r#box.map(|v|v.edge),
id:self.id,
conf:self.conf,
class_name:self.class_name,
}),binding)}}
impl Cursor<types::PageLayout> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PageLayout>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.id)}
pub fn set_id<P:Permission+Writes<types::PageLayout>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.id=value;Ok(())}
pub fn conf<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.conf)}
pub fn set_conf<P:Permission+Writes<types::PageLayout>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.conf=value;Ok(())}
pub fn class_name<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.class_name)}
pub fn set_class_name<P:Permission+Writes<types::PageLayout>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.class_name=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PageLayoutInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PageLayoutInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
id:n.id.clone(),
conf:n.conf.clone(),
class_name:n.class_name.clone(),
})}}
#[derive(Default)] pub struct DataPdfFigure {
pub r#box:Option<Edge>,
}
#[derive(Default)] pub struct PdfFigureInit {
pub r#box:Option<FieldValue<types::Box>>,
}
impl PdfFigureInit {pub fn into_value(self)->FieldValue<types::PdfFigure> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfFigure(DataPdfFigure {
r#box:self.r#box.map(|v|v.edge),
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfFigure> {
pub fn r#box<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfFigureInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfFigureInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
})}}
#[derive(Default)] pub struct DataPdfFont {
pub pdf_font_char_bounding_box:Vec<Edge>,
pub name:Option<String>,
pub font_id:Option<String>,
pub xref_id:Option<i64>,
pub encoding_length:Option<i64>,
pub font_subtype:Option<String>,
pub type3_font_matrix:Option<String>,
pub type3_font_bbox:Option<String>,
pub type3_em_height:Option<f64>,
pub bold:Option<babeldoc_il_types::RuntimeBool>,
pub italic:Option<babeldoc_il_types::RuntimeBool>,
pub monospace:Option<babeldoc_il_types::RuntimeBool>,
pub serif:Option<babeldoc_il_types::RuntimeBool>,
pub ascent:Option<f64>,
pub descent:Option<f64>,
}
#[derive(Default)] pub struct PdfFontInit {
pub pdf_font_char_bounding_box:Vec<FieldValue<types::PdfFontCharBoundingBox>>,
pub name:Option<String>,
pub font_id:Option<String>,
pub xref_id:Option<i64>,
pub encoding_length:Option<i64>,
pub font_subtype:Option<String>,
pub type3_font_matrix:Option<String>,
pub type3_font_bbox:Option<String>,
pub type3_em_height:Option<f64>,
pub bold:Option<babeldoc_il_types::RuntimeBool>,
pub italic:Option<babeldoc_il_types::RuntimeBool>,
pub monospace:Option<babeldoc_il_types::RuntimeBool>,
pub serif:Option<babeldoc_il_types::RuntimeBool>,
pub ascent:Option<f64>,
pub descent:Option<f64>,
}
impl PdfFontInit {pub fn into_value(self)->FieldValue<types::PdfFont> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
for child in &self.pdf_font_char_bounding_box {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfFont(DataPdfFont {
pdf_font_char_bounding_box:self.pdf_font_char_bounding_box.into_iter().map(|v|v.edge).collect(),
name:self.name,
font_id:self.font_id,
xref_id:self.xref_id,
encoding_length:self.encoding_length,
font_subtype:self.font_subtype,
type3_font_matrix:self.type3_font_matrix,
type3_font_bbox:self.type3_font_bbox,
type3_em_height:self.type3_em_height,
bold:self.bold,
italic:self.italic,
monospace:self.monospace,
serif:self.serif,
ascent:self.ascent,
descent:self.descent,
}),binding)}}
impl Cursor<types::PdfFont> {
pub fn pdf_font_char_bounding_box_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_font_char_bounding_box.len())}
pub fn pdf_font_char_bounding_box_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfFontCharBoundingBox>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfFontCharBoundingBox>::from_edge(n.pdf_font_char_bounding_box.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_font_char_bounding_box_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfFontCharBoundingBox>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_font_char_bounding_box.iter().map(move |edge|Cursor::<types::PdfFontCharBoundingBox>::from_edge(edge, generation)))}
pub fn set_pdf_font_char_bounding_box<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfFontCharBoundingBox>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_font_char_bounding_box=prepared;Ok(())}
pub fn push_pdf_font_char_bounding_box<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfFontCharBoundingBox>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_font_char_bounding_box.push(prepared);Ok(())}
pub fn name<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.name)}
pub fn set_name<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.name=value;Ok(())}
pub fn font_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.font_id)}
pub fn set_font_id<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.font_id=value;Ok(())}
pub fn xref_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xref_id)}
pub fn set_xref_id<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xref_id=value;Ok(())}
pub fn encoding_length<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.encoding_length)}
pub fn set_encoding_length<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.encoding_length=value;Ok(())}
pub fn font_subtype<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.font_subtype)}
pub fn set_font_subtype<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.font_subtype=value;Ok(())}
pub fn type3_font_matrix<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.type3_font_matrix)}
pub fn set_type3_font_matrix<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.type3_font_matrix=value;Ok(())}
pub fn type3_font_bbox<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.type3_font_bbox)}
pub fn set_type3_font_bbox<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.type3_font_bbox=value;Ok(())}
pub fn type3_em_height<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.type3_em_height)}
pub fn set_type3_em_height<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.type3_em_height=value;Ok(())}
pub fn bold<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<babeldoc_il_types::RuntimeBool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.bold)}
pub fn set_bold<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<babeldoc_il_types::RuntimeBool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.bold=value;Ok(())}
pub fn italic<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<babeldoc_il_types::RuntimeBool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.italic)}
pub fn set_italic<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<babeldoc_il_types::RuntimeBool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.italic=value;Ok(())}
pub fn monospace<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<babeldoc_il_types::RuntimeBool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.monospace)}
pub fn set_monospace<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<babeldoc_il_types::RuntimeBool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.monospace=value;Ok(())}
pub fn serif<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<babeldoc_il_types::RuntimeBool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.serif)}
pub fn set_serif<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<babeldoc_il_types::RuntimeBool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.serif=value;Ok(())}
pub fn ascent<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.ascent)}
pub fn set_ascent<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.ascent=value;Ok(())}
pub fn descent<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.descent)}
pub fn set_descent<P:Permission+Writes<types::PdfFont>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.descent=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfFontInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfFontInit {
pdf_font_char_bounding_box:n.pdf_font_char_bounding_box.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
name:n.name.clone(),
font_id:n.font_id.clone(),
xref_id:n.xref_id.clone(),
encoding_length:n.encoding_length.clone(),
font_subtype:n.font_subtype.clone(),
type3_font_matrix:n.type3_font_matrix.clone(),
type3_font_bbox:n.type3_font_bbox.clone(),
type3_em_height:n.type3_em_height.clone(),
bold:n.bold.clone(),
italic:n.italic.clone(),
monospace:n.monospace.clone(),
serif:n.serif.clone(),
ascent:n.ascent.clone(),
descent:n.descent.clone(),
})}}
#[derive(Default)] pub struct DataPdfFormSubtype {
pub pdf_inline_form:Option<Edge>,
pub pdf_xobj_form:Option<Edge>,
}
#[derive(Default)] pub struct PdfFormSubtypeInit {
pub pdf_inline_form:Option<FieldValue<types::PdfInlineForm>>,
pub pdf_xobj_form:Option<FieldValue<types::PdfXobjForm>>,
}
impl PdfFormSubtypeInit {pub fn into_value(self)->FieldValue<types::PdfFormSubtype> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.pdf_inline_form {binding.merge(child.binding);}
if let Some(child)=&self.pdf_xobj_form {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfFormSubtype(DataPdfFormSubtype {
pdf_inline_form:self.pdf_inline_form.map(|v|v.edge),
pdf_xobj_form:self.pdf_xobj_form.map(|v|v.edge),
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfFormSubtype> {
pub fn pdf_inline_form<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<Borrowed<'n,types::PdfInlineForm>>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;n.pdf_inline_form.as_ref().map(|edge|Borrowed::<types::PdfInlineForm>::from_edge(edge, generation)).transpose()}
pub fn pdf_xobj_form<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<Borrowed<'n,types::PdfXobjForm>>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;n.pdf_xobj_form.as_ref().map(|edge|Borrowed::<types::PdfXobjForm>::from_edge(edge, generation)).transpose()}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfFormSubtypeInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfFormSubtypeInit {
pdf_inline_form:n.pdf_inline_form.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_xobj_form:n.pdf_xobj_form.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
})}}
#[derive(Default)] pub struct DataPdfOriginalPath {
pub pdf_path:Option<Edge>,
}
#[derive(Default)] pub struct PdfOriginalPathInit {
pub pdf_path:Option<FieldValue<types::PdfPath>>,
}
impl PdfOriginalPathInit {pub fn into_value(self)->FieldValue<types::PdfOriginalPath> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.pdf_path {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfOriginalPath(DataPdfOriginalPath {
pdf_path:self.pdf_path.map(|v|v.edge),
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfOriginalPath> {
pub fn pdf_path<P:Permission>(&self,v:&TreeView<'_,P>)->Result<Option<Borrowed<'n,types::PdfPath>>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;n.pdf_path.as_ref().map(|edge|Borrowed::<types::PdfPath>::from_edge(edge, generation)).transpose()}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfOriginalPathInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfOriginalPathInit {
pdf_path:n.pdf_path.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
})}}
#[derive(Default)] pub struct DataPdfOriginalPathPrimitive {
pub op:Option<String>,
pub args:Vec<f64>,
}
#[derive(Default)] pub struct PdfOriginalPathPrimitiveInit {
pub op:Option<String>,
pub args:Vec<f64>,
}
impl PdfOriginalPathPrimitiveInit {pub fn into_value(self)->FieldValue<types::PdfOriginalPathPrimitive> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
FieldValue::new(NodeData::PdfOriginalPathPrimitive(DataPdfOriginalPathPrimitive {
op:self.op,
args:self.args,
}),binding)}}
#[allow(unused_variables)] impl<'n> Borrowed<'n,types::PdfOriginalPathPrimitive> {
pub fn op<P:Permission>(&self,v:&TreeView<'_,P>)->Result<&'n Option<String>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(&n.op)}
pub fn args<P:Permission>(&self,v:&TreeView<'_,P>)->Result<&'n Vec<f64>,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(&n.args)}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfOriginalPathPrimitiveInit,AccessError>{self.validate(v)?;#[allow(unused_variables)] let generation=v.document_ref().generation;let n=&self.node().data;Ok(PdfOriginalPathPrimitiveInit {
op:n.op.clone(),
args:n.args.clone(),
})}}
#[derive(Default)] pub struct DataPdfRectangle {
pub r#box:Option<Edge>,
pub graphic_state:Option<Edge>,
pub debug_info:Option<bool>,
pub fill_background:Option<bool>,
pub xobj_id:Option<i64>,
pub line_width:Option<f64>,
pub render_order:Option<i64>,
}
#[derive(Default)] pub struct PdfRectangleInit {
pub r#box:Option<FieldValue<types::Box>>,
pub graphic_state:Option<FieldValue<types::GraphicState>>,
pub debug_info:Option<bool>,
pub fill_background:Option<bool>,
pub xobj_id:Option<i64>,
pub line_width:Option<f64>,
pub render_order:Option<i64>,
}
impl PdfRectangleInit {pub fn into_value(self)->FieldValue<types::PdfRectangle> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
if let Some(child)=&self.graphic_state {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfRectangle(DataPdfRectangle {
r#box:self.r#box.map(|v|v.edge),
graphic_state:self.graphic_state.map(|v|v.edge),
debug_info:self.debug_info,
fill_background:self.fill_background,
xobj_id:self.xobj_id,
line_width:self.line_width,
render_order:self.render_order,
}),binding)}}
impl Cursor<types::PdfRectangle> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfRectangle>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn graphic_state<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::GraphicState>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.graphic_state.as_ref().map(|edge|Cursor::<types::GraphicState>::from_edge(edge, generation)).transpose()}
pub fn set_graphic_state<P:Permission+Writes<types::PdfRectangle>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::GraphicState>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.graphic_state=prepared;Ok(())}
pub fn debug_info<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.debug_info)}
pub fn set_debug_info<P:Permission+Writes<types::PdfRectangle>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.debug_info=value;Ok(())}
pub fn fill_background<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.fill_background)}
pub fn set_fill_background<P:Permission+Writes<types::PdfRectangle>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.fill_background=value;Ok(())}
pub fn xobj_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xobj_id)}
pub fn set_xobj_id<P:Permission+Writes<types::PdfRectangle>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xobj_id=value;Ok(())}
pub fn line_width<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.line_width)}
pub fn set_line_width<P:Permission+Writes<types::PdfRectangle>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.line_width=value;Ok(())}
pub fn render_order<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.render_order)}
pub fn set_render_order<P:Permission+Writes<types::PdfRectangle>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.render_order=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfRectangleInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfRectangleInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
graphic_state:n.graphic_state.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
debug_info:n.debug_info.clone(),
fill_background:n.fill_background.clone(),
xobj_id:n.xobj_id.clone(),
line_width:n.line_width.clone(),
render_order:n.render_order.clone(),
})}}
#[derive(Default)] pub struct DataPdfStyle {
pub graphic_state:Option<Edge>,
pub font_id:Option<String>,
pub font_size:Option<f64>,
}
#[derive(Default)] pub struct PdfStyleInit {
pub graphic_state:Option<FieldValue<types::GraphicState>>,
pub font_id:Option<String>,
pub font_size:Option<f64>,
}
impl PdfStyleInit {pub fn into_value(self)->FieldValue<types::PdfStyle> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.graphic_state {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfStyle(DataPdfStyle {
graphic_state:self.graphic_state.map(|v|v.edge),
font_id:self.font_id,
font_size:self.font_size,
}),binding)}}
impl Cursor<types::PdfStyle> {
pub fn graphic_state<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::GraphicState>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.graphic_state.as_ref().map(|edge|Cursor::<types::GraphicState>::from_edge(edge, generation)).transpose()}
pub fn set_graphic_state<P:Permission+Writes<types::PdfStyle>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::GraphicState>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.graphic_state=prepared;Ok(())}
pub fn font_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.font_id)}
pub fn set_font_id<P:Permission+Writes<types::PdfStyle>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.font_id=value;Ok(())}
pub fn font_size<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.font_size)}
pub fn set_font_size<P:Permission+Writes<types::PdfStyle>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.font_size=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfStyleInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfStyleInit {
graphic_state:n.graphic_state.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
font_id:n.font_id.clone(),
font_size:n.font_size.clone(),
})}}
#[derive(Default)] pub struct DataVisualBbox {
pub r#box:Option<Edge>,
}
#[derive(Default)] pub struct VisualBboxInit {
pub r#box:Option<FieldValue<types::Box>>,
}
impl VisualBboxInit {pub fn into_value(self)->FieldValue<types::VisualBbox> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
FieldValue::new(NodeData::VisualBbox(DataVisualBbox {
r#box:self.r#box.map(|v|v.edge),
}),binding)}}
impl Cursor<types::VisualBbox> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::VisualBbox>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<VisualBboxInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(VisualBboxInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
})}}
#[derive(Default)] pub struct DataPdfCharacter {
pub pdf_style:Option<Edge>,
pub r#box:Option<Edge>,
pub visual_bbox:Option<Edge>,
pub vertical:Option<bool>,
pub scale:Option<f64>,
pub visual_roundtrip_text_scale_x:Option<f64>,
pub visual_roundtrip_text_scale_y:Option<f64>,
pub visual_roundtrip_text_render_mode:Option<i64>,
pub pdf_character_id:Option<i64>,
pub char_unicode:Option<String>,
pub advance:Option<f64>,
pub xobj_id:Option<i64>,
pub debug_info:Option<bool>,
pub formula_layout_id:Option<i64>,
pub render_order:Option<i64>,
pub sub_render_order:Option<i64>,
}
#[derive(Default)] pub struct PdfCharacterInit {
pub pdf_style:Option<FieldValue<types::PdfStyle>>,
pub r#box:Option<FieldValue<types::Box>>,
pub visual_bbox:Option<FieldValue<types::VisualBbox>>,
pub vertical:Option<bool>,
pub scale:Option<f64>,
pub visual_roundtrip_text_scale_x:Option<f64>,
pub visual_roundtrip_text_scale_y:Option<f64>,
pub visual_roundtrip_text_render_mode:Option<i64>,
pub pdf_character_id:Option<i64>,
pub char_unicode:Option<String>,
pub advance:Option<f64>,
pub xobj_id:Option<i64>,
pub debug_info:Option<bool>,
pub formula_layout_id:Option<i64>,
pub render_order:Option<i64>,
pub sub_render_order:Option<i64>,
}
impl PdfCharacterInit {pub fn into_value(self)->FieldValue<types::PdfCharacter> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.pdf_style {binding.merge(child.binding);}
if let Some(child)=&self.r#box {binding.merge(child.binding);}
if let Some(child)=&self.visual_bbox {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfCharacter(DataPdfCharacter {
pdf_style:self.pdf_style.map(|v|v.edge),
r#box:self.r#box.map(|v|v.edge),
visual_bbox:self.visual_bbox.map(|v|v.edge),
vertical:self.vertical,
scale:self.scale,
visual_roundtrip_text_scale_x:self.visual_roundtrip_text_scale_x,
visual_roundtrip_text_scale_y:self.visual_roundtrip_text_scale_y,
visual_roundtrip_text_render_mode:self.visual_roundtrip_text_render_mode,
pdf_character_id:self.pdf_character_id,
char_unicode:self.char_unicode,
advance:self.advance,
xobj_id:self.xobj_id,
debug_info:self.debug_info,
formula_layout_id:self.formula_layout_id,
render_order:self.render_order,
sub_render_order:self.sub_render_order,
}),binding)}}
impl Cursor<types::PdfCharacter> {
pub fn pdf_style<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfStyle>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_style.as_ref().map(|edge|Cursor::<types::PdfStyle>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_style<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfStyle>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_style=prepared;Ok(())}
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn visual_bbox<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::VisualBbox>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.visual_bbox.as_ref().map(|edge|Cursor::<types::VisualBbox>::from_edge(edge, generation)).transpose()}
pub fn set_visual_bbox<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::VisualBbox>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.visual_bbox=prepared;Ok(())}
pub fn vertical<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.vertical)}
pub fn set_vertical<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.vertical=value;Ok(())}
pub fn scale<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.scale)}
pub fn set_scale<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.scale=value;Ok(())}
pub fn visual_roundtrip_text_scale_x<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.visual_roundtrip_text_scale_x)}
pub fn set_visual_roundtrip_text_scale_x<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.visual_roundtrip_text_scale_x=value;Ok(())}
pub fn visual_roundtrip_text_scale_y<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.visual_roundtrip_text_scale_y)}
pub fn set_visual_roundtrip_text_scale_y<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.visual_roundtrip_text_scale_y=value;Ok(())}
pub fn visual_roundtrip_text_render_mode<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.visual_roundtrip_text_render_mode)}
pub fn set_visual_roundtrip_text_render_mode<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.visual_roundtrip_text_render_mode=value;Ok(())}
pub fn pdf_character_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character_id)}
pub fn set_pdf_character_id<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.pdf_character_id=value;Ok(())}
pub fn char_unicode<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.char_unicode)}
pub fn set_char_unicode<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.char_unicode=value;Ok(())}
pub fn advance<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.advance)}
pub fn set_advance<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.advance=value;Ok(())}
pub fn xobj_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xobj_id)}
pub fn set_xobj_id<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xobj_id=value;Ok(())}
pub fn debug_info<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.debug_info)}
pub fn set_debug_info<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.debug_info=value;Ok(())}
pub fn formula_layout_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.formula_layout_id)}
pub fn set_formula_layout_id<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.formula_layout_id=value;Ok(())}
pub fn render_order<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.render_order)}
pub fn set_render_order<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.render_order=value;Ok(())}
pub fn sub_render_order<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.sub_render_order)}
pub fn set_sub_render_order<P:Permission+Writes<types::PdfCharacter>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.sub_render_order=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfCharacterInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfCharacterInit {
pdf_style:n.pdf_style.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
visual_bbox:n.visual_bbox.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
vertical:n.vertical.clone(),
scale:n.scale.clone(),
visual_roundtrip_text_scale_x:n.visual_roundtrip_text_scale_x.clone(),
visual_roundtrip_text_scale_y:n.visual_roundtrip_text_scale_y.clone(),
visual_roundtrip_text_render_mode:n.visual_roundtrip_text_render_mode.clone(),
pdf_character_id:n.pdf_character_id.clone(),
char_unicode:n.char_unicode.clone(),
advance:n.advance.clone(),
xobj_id:n.xobj_id.clone(),
debug_info:n.debug_info.clone(),
formula_layout_id:n.formula_layout_id.clone(),
render_order:n.render_order.clone(),
sub_render_order:n.sub_render_order.clone(),
})}}
#[derive(Default)] pub struct DataPdfCurve {
pub r#box:Option<Edge>,
pub graphic_state:Option<Edge>,
pub pdf_path:Vec<Edge>,
pub pdf_original_path:Vec<Edge>,
pub pdf_original_path_primitive:Option<Edge>,
pub debug_info:Option<babeldoc_il_types::CurveDebugInfo>,
pub fill_background:Option<bool>,
pub stroke_path:Option<bool>,
pub evenodd:Option<bool>,
pub passthrough_paint:Option<bool>,
pub xobj_id:Option<i64>,
pub render_order:Option<i64>,
pub ctm:CurveCtm,
pub relocation_transform:Vec<babeldoc_il_types::PythonValue>,
}
#[derive(Default)] pub struct PdfCurveInit {
pub r#box:Option<FieldValue<types::Box>>,
pub graphic_state:Option<FieldValue<types::GraphicState>>,
pub pdf_path:Vec<FieldValue<types::PdfPath>>,
pub pdf_original_path:Vec<FieldValue<types::PdfOriginalPath>>,
pub pdf_original_path_primitive:Option<FieldValue<types::PdfOriginalPathPrimitive>>,
pub debug_info:Option<babeldoc_il_types::CurveDebugInfo>,
pub fill_background:Option<bool>,
pub stroke_path:Option<bool>,
pub evenodd:Option<bool>,
pub passthrough_paint:Option<bool>,
pub xobj_id:Option<i64>,
pub render_order:Option<i64>,
pub ctm:Vec<babeldoc_il_types::PythonValue>,
pub relocation_transform:Vec<babeldoc_il_types::PythonValue>,
}
impl PdfCurveInit {pub fn into_value(self)->FieldValue<types::PdfCurve> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
if let Some(child)=&self.graphic_state {binding.merge(child.binding);}
for child in &self.pdf_path {binding.merge(child.binding);}
for child in &self.pdf_original_path {binding.merge(child.binding);}
if let Some(child)=&self.pdf_original_path_primitive {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfCurve(DataPdfCurve {
r#box:self.r#box.map(|v|v.edge),
graphic_state:self.graphic_state.map(|v|v.edge),
pdf_path:self.pdf_path.into_iter().map(|v|v.edge).collect(),
pdf_original_path:self.pdf_original_path.into_iter().map(|v|v.edge).collect(),
pdf_original_path_primitive:self.pdf_original_path_primitive.map(|v|v.edge),
debug_info:self.debug_info,
fill_background:self.fill_background,
stroke_path:self.stroke_path,
evenodd:self.evenodd,
passthrough_paint:self.passthrough_paint,
xobj_id:self.xobj_id,
render_order:self.render_order,
ctm:self.ctm.into(),
relocation_transform:self.relocation_transform,
}),binding)}}
impl Cursor<types::PdfCurve> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn graphic_state<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::GraphicState>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.graphic_state.as_ref().map(|edge|Cursor::<types::GraphicState>::from_edge(edge, generation)).transpose()}
pub fn set_graphic_state<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::GraphicState>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.graphic_state=prepared;Ok(())}
pub fn pdf_path_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_path.len())}
pub fn pdf_path_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Borrowed<'a,types::PdfPath>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Borrowed::<types::PdfPath>::from_edge(n.pdf_path.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_path_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Borrowed<'a,types::PdfPath>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_path.iter().map(move |edge|Borrowed::<types::PdfPath>::from_edge(edge, generation)))}
pub fn set_pdf_path<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfPath>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_path=prepared;Ok(())}
pub fn push_pdf_path<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfPath>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_path.push(prepared);Ok(())}
pub fn pdf_original_path_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_original_path.len())}
pub fn pdf_original_path_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Borrowed<'a,types::PdfOriginalPath>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Borrowed::<types::PdfOriginalPath>::from_edge(n.pdf_original_path.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_original_path_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Borrowed<'a,types::PdfOriginalPath>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_original_path.iter().map(move |edge|Borrowed::<types::PdfOriginalPath>::from_edge(edge, generation)))}
pub fn set_pdf_original_path<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfOriginalPath>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_original_path=prepared;Ok(())}
pub fn push_pdf_original_path<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfOriginalPath>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_original_path.push(prepared);Ok(())}
pub fn pdf_original_path_primitive<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::PdfOriginalPathPrimitive>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_original_path_primitive.as_ref().map(|edge|Borrowed::<types::PdfOriginalPathPrimitive>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_original_path_primitive<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfOriginalPathPrimitive>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_original_path_primitive=prepared;Ok(())}
pub fn debug_info<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<babeldoc_il_types::CurveDebugInfo>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.debug_info)}
pub fn set_debug_info<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Option<babeldoc_il_types::CurveDebugInfo>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.debug_info=value;Ok(())}
pub fn fill_background<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.fill_background)}
pub fn set_fill_background<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.fill_background=value;Ok(())}
pub fn stroke_path<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.stroke_path)}
pub fn set_stroke_path<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.stroke_path=value;Ok(())}
pub fn evenodd<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.evenodd)}
pub fn set_evenodd<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.evenodd=value;Ok(())}
pub fn passthrough_paint<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.passthrough_paint)}
pub fn set_passthrough_paint<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.passthrough_paint=value;Ok(())}
pub fn xobj_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xobj_id)}
pub fn set_xobj_id<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xobj_id=value;Ok(())}
pub fn render_order<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.render_order)}
pub fn set_render_order<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.render_order=value;Ok(())}
pub fn ctm<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a CurveCtm,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.ctm)}
pub fn set_ctm<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Vec<babeldoc_il_types::PythonValue>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.ctm=value.into();Ok(())}
pub fn relocation_transform<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Vec<babeldoc_il_types::PythonValue>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.relocation_transform)}
pub fn set_relocation_transform<P:Permission+Writes<types::PdfCurve>>(&self,v:&mut TreeView<'_,P>,value:Vec<babeldoc_il_types::PythonValue>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.relocation_transform=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfCurveInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfCurveInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
graphic_state:n.graphic_state.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_path:n.pdf_path.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_original_path:n.pdf_original_path.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_original_path_primitive:n.pdf_original_path_primitive.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
debug_info:n.debug_info.clone(),
fill_background:n.fill_background.clone(),
stroke_path:n.stroke_path.clone(),
evenodd:n.evenodd.clone(),
passthrough_paint:n.passthrough_paint.clone(),
xobj_id:n.xobj_id.clone(),
render_order:n.render_order.clone(),
ctm:n.ctm.to_vec(),
relocation_transform:n.relocation_transform.clone(),
})}}
#[derive(Default)] pub struct DataPdfForm {
pub r#box:Option<Edge>,
pub graphic_state:Option<Edge>,
pub pdf_matrix:Option<Edge>,
pub pdf_affine_transform:Option<Edge>,
pub pdf_form_subtype:Option<Edge>,
pub xobj_id:Option<i64>,
pub ctm:Vec<babeldoc_il_types::PythonValue>,
pub relocation_transform:Vec<babeldoc_il_types::PythonValue>,
pub render_order:Option<i64>,
pub form_type:Option<String>,
}
#[derive(Default)] pub struct PdfFormInit {
pub r#box:Option<FieldValue<types::Box>>,
pub graphic_state:Option<FieldValue<types::GraphicState>>,
pub pdf_matrix:Option<FieldValue<types::PdfMatrix>>,
pub pdf_affine_transform:Option<FieldValue<types::PdfAffineTransform>>,
pub pdf_form_subtype:Option<FieldValue<types::PdfFormSubtype>>,
pub xobj_id:Option<i64>,
pub ctm:Vec<babeldoc_il_types::PythonValue>,
pub relocation_transform:Vec<babeldoc_il_types::PythonValue>,
pub render_order:Option<i64>,
pub form_type:Option<String>,
}
impl PdfFormInit {pub fn into_value(self)->FieldValue<types::PdfForm> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
if let Some(child)=&self.graphic_state {binding.merge(child.binding);}
if let Some(child)=&self.pdf_matrix {binding.merge(child.binding);}
if let Some(child)=&self.pdf_affine_transform {binding.merge(child.binding);}
if let Some(child)=&self.pdf_form_subtype {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfForm(DataPdfForm {
r#box:self.r#box.map(|v|v.edge),
graphic_state:self.graphic_state.map(|v|v.edge),
pdf_matrix:self.pdf_matrix.map(|v|v.edge),
pdf_affine_transform:self.pdf_affine_transform.map(|v|v.edge),
pdf_form_subtype:self.pdf_form_subtype.map(|v|v.edge),
xobj_id:self.xobj_id,
ctm:self.ctm,
relocation_transform:self.relocation_transform,
render_order:self.render_order,
form_type:self.form_type,
}),binding)}}
impl Cursor<types::PdfForm> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn graphic_state<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::GraphicState>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.graphic_state.as_ref().map(|edge|Cursor::<types::GraphicState>::from_edge(edge, generation)).transpose()}
pub fn set_graphic_state<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::GraphicState>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.graphic_state=prepared;Ok(())}
pub fn pdf_matrix<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::PdfMatrix>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_matrix.as_ref().map(|edge|Borrowed::<types::PdfMatrix>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_matrix<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfMatrix>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_matrix=prepared;Ok(())}
pub fn pdf_affine_transform<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::PdfAffineTransform>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_affine_transform.as_ref().map(|edge|Borrowed::<types::PdfAffineTransform>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_affine_transform<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfAffineTransform>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_affine_transform=prepared;Ok(())}
pub fn pdf_form_subtype<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::PdfFormSubtype>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_form_subtype.as_ref().map(|edge|Borrowed::<types::PdfFormSubtype>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_form_subtype<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfFormSubtype>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_form_subtype=prepared;Ok(())}
pub fn xobj_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xobj_id)}
pub fn set_xobj_id<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xobj_id=value;Ok(())}
pub fn ctm<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Vec<babeldoc_il_types::PythonValue>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.ctm)}
pub fn set_ctm<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,value:Vec<babeldoc_il_types::PythonValue>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.ctm=value;Ok(())}
pub fn relocation_transform<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Vec<babeldoc_il_types::PythonValue>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.relocation_transform)}
pub fn set_relocation_transform<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,value:Vec<babeldoc_il_types::PythonValue>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.relocation_transform=value;Ok(())}
pub fn render_order<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.render_order)}
pub fn set_render_order<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.render_order=value;Ok(())}
pub fn form_type<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.form_type)}
pub fn set_form_type<P:Permission+Writes<types::PdfForm>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.form_type=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfFormInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfFormInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
graphic_state:n.graphic_state.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_matrix:n.pdf_matrix.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_affine_transform:n.pdf_affine_transform.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_form_subtype:n.pdf_form_subtype.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
xobj_id:n.xobj_id.clone(),
ctm:n.ctm.clone(),
relocation_transform:n.relocation_transform.clone(),
render_order:n.render_order.clone(),
form_type:n.form_type.clone(),
})}}
#[derive(Default)] pub struct DataPdfSameStyleUnicodeCharacters {
pub pdf_style:Option<Edge>,
pub unicode:Option<String>,
pub debug_info:Option<bool>,
}
#[derive(Default)] pub struct PdfSameStyleUnicodeCharactersInit {
pub pdf_style:Option<FieldValue<types::PdfStyle>>,
pub unicode:Option<String>,
pub debug_info:Option<bool>,
}
impl PdfSameStyleUnicodeCharactersInit {pub fn into_value(self)->FieldValue<types::PdfSameStyleUnicodeCharacters> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.pdf_style {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfSameStyleUnicodeCharacters(DataPdfSameStyleUnicodeCharacters {
pdf_style:self.pdf_style.map(|v|v.edge),
unicode:self.unicode,
debug_info:self.debug_info,
}),binding)}}
impl Cursor<types::PdfSameStyleUnicodeCharacters> {
pub fn pdf_style<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfStyle>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_style.as_ref().map(|edge|Cursor::<types::PdfStyle>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_style<P:Permission+Writes<types::PdfSameStyleUnicodeCharacters>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfStyle>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_style=prepared;Ok(())}
pub fn unicode<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.unicode)}
pub fn set_unicode<P:Permission+Writes<types::PdfSameStyleUnicodeCharacters>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.unicode=value;Ok(())}
pub fn debug_info<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.debug_info)}
pub fn set_debug_info<P:Permission+Writes<types::PdfSameStyleUnicodeCharacters>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.debug_info=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfSameStyleUnicodeCharactersInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfSameStyleUnicodeCharactersInit {
pdf_style:n.pdf_style.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
unicode:n.unicode.clone(),
debug_info:n.debug_info.clone(),
})}}
#[derive(Default)] pub struct DataPdfXobject {
pub r#box:Option<Edge>,
pub pdf_font:Vec<Edge>,
pub base_operations:Option<Edge>,
pub xobj_id:Option<i64>,
pub xref_id:Option<i64>,
}
#[derive(Default)] pub struct PdfXobjectInit {
pub r#box:Option<FieldValue<types::Box>>,
pub pdf_font:Vec<FieldValue<types::PdfFont>>,
pub base_operations:Option<FieldValue<types::BaseOperations>>,
pub xobj_id:Option<i64>,
pub xref_id:Option<i64>,
}
impl PdfXobjectInit {pub fn into_value(self)->FieldValue<types::PdfXobject> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
for child in &self.pdf_font {binding.merge(child.binding);}
if let Some(child)=&self.base_operations {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfXobject(DataPdfXobject {
r#box:self.r#box.map(|v|v.edge),
pdf_font:self.pdf_font.into_iter().map(|v|v.edge).collect(),
base_operations:self.base_operations.map(|v|v.edge),
xobj_id:self.xobj_id,
xref_id:self.xref_id,
}),binding)}}
impl Cursor<types::PdfXobject> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfXobject>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn pdf_font_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_font.len())}
pub fn pdf_font_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfFont>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfFont>::from_edge(n.pdf_font.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_font_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfFont>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_font.iter().map(move |edge|Cursor::<types::PdfFont>::from_edge(edge, generation)))}
pub fn set_pdf_font<P:Permission+Writes<types::PdfXobject>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfFont>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_font=prepared;Ok(())}
pub fn push_pdf_font<P:Permission+Writes<types::PdfXobject>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfFont>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_font.push(prepared);Ok(())}
pub fn base_operations<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::BaseOperations>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.base_operations.as_ref().map(|edge|Borrowed::<types::BaseOperations>::from_edge(edge, generation)).transpose()}
pub fn set_base_operations<P:Permission+Writes<types::PdfXobject>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::BaseOperations>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.base_operations=prepared;Ok(())}
pub fn xobj_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xobj_id)}
pub fn set_xobj_id<P:Permission+Writes<types::PdfXobject>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xobj_id=value;Ok(())}
pub fn xref_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xref_id)}
pub fn set_xref_id<P:Permission+Writes<types::PdfXobject>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xref_id=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfXobjectInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfXobjectInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_font:n.pdf_font.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
base_operations:n.base_operations.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
xobj_id:n.xobj_id.clone(),
xref_id:n.xref_id.clone(),
})}}
#[derive(Default)] pub struct DataPdfFormula {
pub r#box:Option<Edge>,
pub pdf_character:Vec<Edge>,
pub pdf_curve:Vec<Edge>,
pub pdf_form:Vec<Edge>,
pub x_offset:Option<f64>,
pub y_offset:Option<f64>,
pub x_advance:Option<f64>,
pub line_id:Option<i64>,
pub is_corner_mark:Option<bool>,
}
#[derive(Default)] pub struct PdfFormulaInit {
pub r#box:Option<FieldValue<types::Box>>,
pub pdf_character:Vec<FieldValue<types::PdfCharacter>>,
pub pdf_curve:Vec<FieldValue<types::PdfCurve>>,
pub pdf_form:Vec<FieldValue<types::PdfForm>>,
pub x_offset:Option<f64>,
pub y_offset:Option<f64>,
pub x_advance:Option<f64>,
pub line_id:Option<i64>,
pub is_corner_mark:Option<bool>,
}
impl PdfFormulaInit {pub fn into_value(self)->FieldValue<types::PdfFormula> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
for child in &self.pdf_character {binding.merge(child.binding);}
for child in &self.pdf_curve {binding.merge(child.binding);}
for child in &self.pdf_form {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfFormula(DataPdfFormula {
r#box:self.r#box.map(|v|v.edge),
pdf_character:self.pdf_character.into_iter().map(|v|v.edge).collect(),
pdf_curve:self.pdf_curve.into_iter().map(|v|v.edge).collect(),
pdf_form:self.pdf_form.into_iter().map(|v|v.edge).collect(),
x_offset:self.x_offset,
y_offset:self.y_offset,
x_advance:self.x_advance,
line_id:self.line_id,
is_corner_mark:self.is_corner_mark,
}),binding)}}
impl Cursor<types::PdfFormula> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn pdf_character_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.len())}
pub fn pdf_character_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfCharacter>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfCharacter>::from_edge(n.pdf_character.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_character_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfCharacter>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.iter().map(move |edge|Cursor::<types::PdfCharacter>::from_edge(edge, generation)))}
pub fn set_pdf_character<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfCharacter>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_character=prepared;Ok(())}
pub fn push_pdf_character<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfCharacter>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_character.push(prepared);Ok(())}
pub fn pdf_curve_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_curve.len())}
pub fn pdf_curve_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfCurve>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfCurve>::from_edge(n.pdf_curve.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_curve_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfCurve>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_curve.iter().map(move |edge|Cursor::<types::PdfCurve>::from_edge(edge, generation)))}
pub fn set_pdf_curve<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfCurve>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_curve=prepared;Ok(())}
pub fn push_pdf_curve<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfCurve>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_curve.push(prepared);Ok(())}
pub fn pdf_form_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_form.len())}
pub fn pdf_form_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfForm>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfForm>::from_edge(n.pdf_form.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_form_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfForm>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_form.iter().map(move |edge|Cursor::<types::PdfForm>::from_edge(edge, generation)))}
pub fn set_pdf_form<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfForm>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_form=prepared;Ok(())}
pub fn push_pdf_form<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfForm>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_form.push(prepared);Ok(())}
pub fn x_offset<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.x_offset)}
pub fn set_x_offset<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.x_offset=value;Ok(())}
pub fn y_offset<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.y_offset)}
pub fn set_y_offset<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.y_offset=value;Ok(())}
pub fn x_advance<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.x_advance)}
pub fn set_x_advance<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.x_advance=value;Ok(())}
pub fn line_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.line_id)}
pub fn set_line_id<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.line_id=value;Ok(())}
pub fn is_corner_mark<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.is_corner_mark)}
pub fn set_is_corner_mark<P:Permission+Writes<types::PdfFormula>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.is_corner_mark=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfFormulaInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfFormulaInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_character:n.pdf_character.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_curve:n.pdf_curve.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_form:n.pdf_form.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
x_offset:n.x_offset.clone(),
y_offset:n.y_offset.clone(),
x_advance:n.x_advance.clone(),
line_id:n.line_id.clone(),
is_corner_mark:n.is_corner_mark.clone(),
})}}
#[derive(Default)] pub struct DataPdfLine {
pub r#box:Option<Edge>,
pub pdf_character:Vec<Edge>,
pub render_order:Option<i64>,
}
#[derive(Default)] pub struct PdfLineInit {
pub r#box:Option<FieldValue<types::Box>>,
pub pdf_character:Vec<FieldValue<types::PdfCharacter>>,
pub render_order:Option<i64>,
}
impl PdfLineInit {pub fn into_value(self)->FieldValue<types::PdfLine> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
for child in &self.pdf_character {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfLine(DataPdfLine {
r#box:self.r#box.map(|v|v.edge),
pdf_character:self.pdf_character.into_iter().map(|v|v.edge).collect(),
render_order:self.render_order,
}),binding)}}
impl Cursor<types::PdfLine> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfLine>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn pdf_character_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.len())}
pub fn pdf_character_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfCharacter>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfCharacter>::from_edge(n.pdf_character.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_character_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfCharacter>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.iter().map(move |edge|Cursor::<types::PdfCharacter>::from_edge(edge, generation)))}
pub fn set_pdf_character<P:Permission+Writes<types::PdfLine>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfCharacter>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_character=prepared;Ok(())}
pub fn push_pdf_character<P:Permission+Writes<types::PdfLine>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfCharacter>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_character.push(prepared);Ok(())}
pub fn render_order<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.render_order)}
pub fn set_render_order<P:Permission+Writes<types::PdfLine>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.render_order=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfLineInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfLineInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_character:n.pdf_character.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
render_order:n.render_order.clone(),
})}}
#[derive(Default)] pub struct DataPdfSameStyleCharacters {
pub r#box:Option<Edge>,
pub pdf_style:Option<Edge>,
pub pdf_character:Vec<Edge>,
}
#[derive(Default)] pub struct PdfSameStyleCharactersInit {
pub r#box:Option<FieldValue<types::Box>>,
pub pdf_style:Option<FieldValue<types::PdfStyle>>,
pub pdf_character:Vec<FieldValue<types::PdfCharacter>>,
}
impl PdfSameStyleCharactersInit {pub fn into_value(self)->FieldValue<types::PdfSameStyleCharacters> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
if let Some(child)=&self.pdf_style {binding.merge(child.binding);}
for child in &self.pdf_character {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfSameStyleCharacters(DataPdfSameStyleCharacters {
r#box:self.r#box.map(|v|v.edge),
pdf_style:self.pdf_style.map(|v|v.edge),
pdf_character:self.pdf_character.into_iter().map(|v|v.edge).collect(),
}),binding)}}
impl Cursor<types::PdfSameStyleCharacters> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfSameStyleCharacters>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn pdf_style<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfStyle>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_style.as_ref().map(|edge|Cursor::<types::PdfStyle>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_style<P:Permission+Writes<types::PdfSameStyleCharacters>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfStyle>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_style=prepared;Ok(())}
pub fn pdf_character_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.len())}
pub fn pdf_character_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfCharacter>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfCharacter>::from_edge(n.pdf_character.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_character_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfCharacter>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.iter().map(move |edge|Cursor::<types::PdfCharacter>::from_edge(edge, generation)))}
pub fn set_pdf_character<P:Permission+Writes<types::PdfSameStyleCharacters>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfCharacter>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_character=prepared;Ok(())}
pub fn push_pdf_character<P:Permission+Writes<types::PdfSameStyleCharacters>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfCharacter>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_character.push(prepared);Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfSameStyleCharactersInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfSameStyleCharactersInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_style:n.pdf_style.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_character:n.pdf_character.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
})}}
#[derive(Default)] pub struct DataPdfParagraphComposition {
pub pdf_line:Option<Edge>,
pub pdf_formula:Option<Edge>,
pub pdf_same_style_characters:Option<Edge>,
pub pdf_character:Option<Edge>,
pub pdf_same_style_unicode_characters:Option<Edge>,
}
#[derive(Default)] pub struct PdfParagraphCompositionInit {
pub pdf_line:Option<FieldValue<types::PdfLine>>,
pub pdf_formula:Option<FieldValue<types::PdfFormula>>,
pub pdf_same_style_characters:Option<FieldValue<types::PdfSameStyleCharacters>>,
pub pdf_character:Option<FieldValue<types::PdfCharacter>>,
pub pdf_same_style_unicode_characters:Option<FieldValue<types::PdfSameStyleUnicodeCharacters>>,
}
impl PdfParagraphCompositionInit {pub fn into_value(self)->FieldValue<types::PdfParagraphComposition> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.pdf_line {binding.merge(child.binding);}
if let Some(child)=&self.pdf_formula {binding.merge(child.binding);}
if let Some(child)=&self.pdf_same_style_characters {binding.merge(child.binding);}
if let Some(child)=&self.pdf_character {binding.merge(child.binding);}
if let Some(child)=&self.pdf_same_style_unicode_characters {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfParagraphComposition(DataPdfParagraphComposition {
pdf_line:self.pdf_line.map(|v|v.edge),
pdf_formula:self.pdf_formula.map(|v|v.edge),
pdf_same_style_characters:self.pdf_same_style_characters.map(|v|v.edge),
pdf_character:self.pdf_character.map(|v|v.edge),
pdf_same_style_unicode_characters:self.pdf_same_style_unicode_characters.map(|v|v.edge),
}),binding)}}
impl Cursor<types::PdfParagraphComposition> {
pub fn pdf_line<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfLine>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_line.as_ref().map(|edge|Cursor::<types::PdfLine>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_line<P:Permission+Writes<types::PdfParagraphComposition>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfLine>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_line=prepared;Ok(())}
pub fn pdf_formula<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfFormula>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_formula.as_ref().map(|edge|Cursor::<types::PdfFormula>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_formula<P:Permission+Writes<types::PdfParagraphComposition>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfFormula>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_formula=prepared;Ok(())}
pub fn pdf_same_style_characters<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfSameStyleCharacters>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_same_style_characters.as_ref().map(|edge|Cursor::<types::PdfSameStyleCharacters>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_same_style_characters<P:Permission+Writes<types::PdfParagraphComposition>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfSameStyleCharacters>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_same_style_characters=prepared;Ok(())}
pub fn pdf_character<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfCharacter>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_character.as_ref().map(|edge|Cursor::<types::PdfCharacter>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_character<P:Permission+Writes<types::PdfParagraphComposition>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfCharacter>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_character=prepared;Ok(())}
pub fn pdf_same_style_unicode_characters<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfSameStyleUnicodeCharacters>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_same_style_unicode_characters.as_ref().map(|edge|Cursor::<types::PdfSameStyleUnicodeCharacters>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_same_style_unicode_characters<P:Permission+Writes<types::PdfParagraphComposition>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfSameStyleUnicodeCharacters>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_same_style_unicode_characters=prepared;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfParagraphCompositionInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfParagraphCompositionInit {
pdf_line:n.pdf_line.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_formula:n.pdf_formula.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_same_style_characters:n.pdf_same_style_characters.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_character:n.pdf_character.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_same_style_unicode_characters:n.pdf_same_style_unicode_characters.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
})}}
#[derive(Default)] pub struct DataPdfParagraph {
pub r#box:Option<Edge>,
pub pdf_style:Option<Edge>,
pub pdf_paragraph_composition:Vec<Edge>,
pub xobj_id:Option<i64>,
pub unicode:Option<String>,
pub scale:Option<f64>,
pub optimal_scale:Option<f64>,
pub vertical:Option<bool>,
pub first_line_indent:Option<bool>,
pub debug_id:Option<String>,
pub layout_label:Option<String>,
pub layout_id:Option<i64>,
pub render_order:Option<i64>,
}
#[derive(Default)] pub struct PdfParagraphInit {
pub r#box:Option<FieldValue<types::Box>>,
pub pdf_style:Option<FieldValue<types::PdfStyle>>,
pub pdf_paragraph_composition:Vec<FieldValue<types::PdfParagraphComposition>>,
pub xobj_id:Option<i64>,
pub unicode:Option<String>,
pub scale:Option<f64>,
pub optimal_scale:Option<f64>,
pub vertical:Option<bool>,
pub first_line_indent:Option<bool>,
pub debug_id:Option<String>,
pub layout_label:Option<String>,
pub layout_id:Option<i64>,
pub render_order:Option<i64>,
}
impl PdfParagraphInit {pub fn into_value(self)->FieldValue<types::PdfParagraph> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.r#box {binding.merge(child.binding);}
if let Some(child)=&self.pdf_style {binding.merge(child.binding);}
for child in &self.pdf_paragraph_composition {binding.merge(child.binding);}
FieldValue::new(NodeData::PdfParagraph(DataPdfParagraph {
r#box:self.r#box.map(|v|v.edge),
pdf_style:self.pdf_style.map(|v|v.edge),
pdf_paragraph_composition:self.pdf_paragraph_composition.into_iter().map(|v|v.edge).collect(),
xobj_id:self.xobj_id,
unicode:self.unicode,
scale:self.scale,
optimal_scale:self.optimal_scale,
vertical:self.vertical,
first_line_indent:self.first_line_indent,
debug_id:self.debug_id,
layout_label:self.layout_label,
layout_id:self.layout_id,
render_order:self.render_order,
}),binding)}}
impl Cursor<types::PdfParagraph> {
pub fn r#box<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::Box>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.r#box.as_ref().map(|edge|Cursor::<types::Box>::from_edge(edge, generation)).transpose()}
pub fn set_box<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Box>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.r#box=prepared;Ok(())}
pub fn pdf_style<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Cursor<types::PdfStyle>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.pdf_style.as_ref().map(|edge|Cursor::<types::PdfStyle>::from_edge(edge, generation)).transpose()}
pub fn set_pdf_style<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::PdfStyle>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.pdf_style=prepared;Ok(())}
pub fn pdf_paragraph_composition_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_paragraph_composition.len())}
pub fn pdf_paragraph_composition_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfParagraphComposition>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfParagraphComposition>::from_edge(n.pdf_paragraph_composition.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_paragraph_composition_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfParagraphComposition>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_paragraph_composition.iter().map(move |edge|Cursor::<types::PdfParagraphComposition>::from_edge(edge, generation)))}
pub fn set_pdf_paragraph_composition<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfParagraphComposition>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_paragraph_composition=prepared;Ok(())}
pub fn push_pdf_paragraph_composition<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfParagraphComposition>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_paragraph_composition.push(prepared);Ok(())}
pub fn xobj_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.xobj_id)}
pub fn set_xobj_id<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.xobj_id=value;Ok(())}
pub fn unicode<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.unicode)}
pub fn set_unicode<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.unicode=value;Ok(())}
pub fn scale<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.scale)}
pub fn set_scale<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.scale=value;Ok(())}
pub fn optimal_scale<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<f64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.optimal_scale)}
pub fn set_optimal_scale<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<f64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.optimal_scale=value;Ok(())}
pub fn vertical<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.vertical)}
pub fn set_vertical<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.vertical=value;Ok(())}
pub fn first_line_indent<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<bool>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.first_line_indent)}
pub fn set_first_line_indent<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<bool>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.first_line_indent=value;Ok(())}
pub fn debug_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.debug_id)}
pub fn set_debug_id<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.debug_id=value;Ok(())}
pub fn layout_label<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.layout_label)}
pub fn set_layout_label<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.layout_label=value;Ok(())}
pub fn layout_id<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.layout_id)}
pub fn set_layout_id<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.layout_id=value;Ok(())}
pub fn render_order<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.render_order)}
pub fn set_render_order<P:Permission+Writes<types::PdfParagraph>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.render_order=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PdfParagraphInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PdfParagraphInit {
r#box:n.r#box.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_style:n.pdf_style.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_paragraph_composition:n.pdf_paragraph_composition.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
xobj_id:n.xobj_id.clone(),
unicode:n.unicode.clone(),
scale:n.scale.clone(),
optimal_scale:n.optimal_scale.clone(),
vertical:n.vertical.clone(),
first_line_indent:n.first_line_indent.clone(),
debug_id:n.debug_id.clone(),
layout_label:n.layout_label.clone(),
layout_id:n.layout_id.clone(),
render_order:n.render_order.clone(),
})}}
#[derive(Default)] pub struct DataPage {
pub mediabox:Option<Edge>,
pub cropbox:Option<Edge>,
pub pdf_xobject:Vec<Edge>,
pub page_layout:Vec<Edge>,
pub pdf_rectangle:Vec<Edge>,
pub pdf_font:Vec<Edge>,
pub pdf_paragraph:Vec<Edge>,
pub pdf_figure:Vec<Edge>,
pub pdf_character:Vec<Edge>,
pub pdf_curve:Vec<Edge>,
pub pdf_form:Vec<Edge>,
pub base_operations:Option<Edge>,
pub page_number:Option<i64>,
pub unit:Option<String>,
}
#[derive(Default)] pub struct PageInit {
pub mediabox:Option<FieldValue<types::Mediabox>>,
pub cropbox:Option<FieldValue<types::Cropbox>>,
pub pdf_xobject:Vec<FieldValue<types::PdfXobject>>,
pub page_layout:Vec<FieldValue<types::PageLayout>>,
pub pdf_rectangle:Vec<FieldValue<types::PdfRectangle>>,
pub pdf_font:Vec<FieldValue<types::PdfFont>>,
pub pdf_paragraph:Vec<FieldValue<types::PdfParagraph>>,
pub pdf_figure:Vec<FieldValue<types::PdfFigure>>,
pub pdf_character:Vec<FieldValue<types::PdfCharacter>>,
pub pdf_curve:Vec<FieldValue<types::PdfCurve>>,
pub pdf_form:Vec<FieldValue<types::PdfForm>>,
pub base_operations:Option<FieldValue<types::BaseOperations>>,
pub page_number:Option<i64>,
pub unit:Option<String>,
}
impl PageInit {pub fn into_value(self)->FieldValue<types::Page> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
if let Some(child)=&self.mediabox {binding.merge(child.binding);}
if let Some(child)=&self.cropbox {binding.merge(child.binding);}
for child in &self.pdf_xobject {binding.merge(child.binding);}
for child in &self.page_layout {binding.merge(child.binding);}
for child in &self.pdf_rectangle {binding.merge(child.binding);}
for child in &self.pdf_font {binding.merge(child.binding);}
for child in &self.pdf_paragraph {binding.merge(child.binding);}
for child in &self.pdf_figure {binding.merge(child.binding);}
for child in &self.pdf_character {binding.merge(child.binding);}
for child in &self.pdf_curve {binding.merge(child.binding);}
for child in &self.pdf_form {binding.merge(child.binding);}
if let Some(child)=&self.base_operations {binding.merge(child.binding);}
FieldValue::new(NodeData::Page(DataPage {
mediabox:self.mediabox.map(|v|v.edge),
cropbox:self.cropbox.map(|v|v.edge),
pdf_xobject:self.pdf_xobject.into_iter().map(|v|v.edge).collect(),
page_layout:self.page_layout.into_iter().map(|v|v.edge).collect(),
pdf_rectangle:self.pdf_rectangle.into_iter().map(|v|v.edge).collect(),
pdf_font:self.pdf_font.into_iter().map(|v|v.edge).collect(),
pdf_paragraph:self.pdf_paragraph.into_iter().map(|v|v.edge).collect(),
pdf_figure:self.pdf_figure.into_iter().map(|v|v.edge).collect(),
pdf_character:self.pdf_character.into_iter().map(|v|v.edge).collect(),
pdf_curve:self.pdf_curve.into_iter().map(|v|v.edge).collect(),
pdf_form:self.pdf_form.into_iter().map(|v|v.edge).collect(),
base_operations:self.base_operations.map(|v|v.edge),
page_number:self.page_number,
unit:self.unit,
}),binding)}}
impl Cursor<types::Page> {
pub fn mediabox<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::Mediabox>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.mediabox.as_ref().map(|edge|Borrowed::<types::Mediabox>::from_edge(edge, generation)).transpose()}
pub fn set_mediabox<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Mediabox>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.mediabox=prepared;Ok(())}
pub fn cropbox<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::Cropbox>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.cropbox.as_ref().map(|edge|Borrowed::<types::Cropbox>::from_edge(edge, generation)).transpose()}
pub fn set_cropbox<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::Cropbox>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.cropbox=prepared;Ok(())}
pub fn pdf_xobject_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_xobject.len())}
pub fn pdf_xobject_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfXobject>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfXobject>::from_edge(n.pdf_xobject.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_xobject_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfXobject>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_xobject.iter().map(move |edge|Cursor::<types::PdfXobject>::from_edge(edge, generation)))}
pub fn set_pdf_xobject<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfXobject>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_xobject=prepared;Ok(())}
pub fn push_pdf_xobject<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfXobject>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_xobject.push(prepared);Ok(())}
pub fn page_layout_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.page_layout.len())}
pub fn page_layout_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PageLayout>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PageLayout>::from_edge(n.page_layout.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn page_layout_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PageLayout>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.page_layout.iter().map(move |edge|Cursor::<types::PageLayout>::from_edge(edge, generation)))}
pub fn set_page_layout<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PageLayout>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.page_layout=prepared;Ok(())}
pub fn push_page_layout<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PageLayout>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.page_layout.push(prepared);Ok(())}
pub fn pdf_rectangle_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_rectangle.len())}
pub fn pdf_rectangle_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfRectangle>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfRectangle>::from_edge(n.pdf_rectangle.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_rectangle_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfRectangle>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_rectangle.iter().map(move |edge|Cursor::<types::PdfRectangle>::from_edge(edge, generation)))}
pub fn set_pdf_rectangle<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfRectangle>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_rectangle=prepared;Ok(())}
pub fn push_pdf_rectangle<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfRectangle>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_rectangle.push(prepared);Ok(())}
pub fn pdf_font_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_font.len())}
pub fn pdf_font_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfFont>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfFont>::from_edge(n.pdf_font.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_font_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfFont>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_font.iter().map(move |edge|Cursor::<types::PdfFont>::from_edge(edge, generation)))}
pub fn set_pdf_font<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfFont>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_font=prepared;Ok(())}
pub fn push_pdf_font<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfFont>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_font.push(prepared);Ok(())}
pub fn pdf_paragraph_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_paragraph.len())}
pub fn pdf_paragraph_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfParagraph>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfParagraph>::from_edge(n.pdf_paragraph.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_paragraph_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfParagraph>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_paragraph.iter().map(move |edge|Cursor::<types::PdfParagraph>::from_edge(edge, generation)))}
pub fn set_pdf_paragraph<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfParagraph>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_paragraph=prepared;Ok(())}
pub fn push_pdf_paragraph<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfParagraph>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_paragraph.push(prepared);Ok(())}
pub fn pdf_figure_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_figure.len())}
pub fn pdf_figure_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Borrowed<'a,types::PdfFigure>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Borrowed::<types::PdfFigure>::from_edge(n.pdf_figure.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_figure_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Borrowed<'a,types::PdfFigure>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_figure.iter().map(move |edge|Borrowed::<types::PdfFigure>::from_edge(edge, generation)))}
pub fn set_pdf_figure<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfFigure>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_figure=prepared;Ok(())}
pub fn push_pdf_figure<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfFigure>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_figure.push(prepared);Ok(())}
pub fn pdf_character_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.len())}
pub fn pdf_character_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfCharacter>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfCharacter>::from_edge(n.pdf_character.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_character_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfCharacter>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_character.iter().map(move |edge|Cursor::<types::PdfCharacter>::from_edge(edge, generation)))}
pub fn set_pdf_character<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfCharacter>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_character=prepared;Ok(())}
pub fn push_pdf_character<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfCharacter>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_character.push(prepared);Ok(())}
pub fn pdf_curve_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_curve.len())}
pub fn pdf_curve_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfCurve>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfCurve>::from_edge(n.pdf_curve.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_curve_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfCurve>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_curve.iter().map(move |edge|Cursor::<types::PdfCurve>::from_edge(edge, generation)))}
pub fn set_pdf_curve<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfCurve>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_curve=prepared;Ok(())}
pub fn push_pdf_curve<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfCurve>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_curve.push(prepared);Ok(())}
pub fn pdf_form_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_form.len())}
pub fn pdf_form_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::PdfForm>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::PdfForm>::from_edge(n.pdf_form.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn pdf_form_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::PdfForm>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.pdf_form.iter().map(move |edge|Cursor::<types::PdfForm>::from_edge(edge, generation)))}
pub fn set_pdf_form<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::PdfForm>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.pdf_form=prepared;Ok(())}
pub fn push_pdf_form<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::PdfForm>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.pdf_form.push(prepared);Ok(())}
pub fn base_operations<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<Borrowed<'a,types::BaseOperations>>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;n.base_operations.as_ref().map(|edge|Borrowed::<types::BaseOperations>::from_edge(edge, generation)).transpose()}
pub fn set_base_operations<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,values:Option<FieldValue<types::BaseOperations>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.map(|c|Ok::<_,AccessError>(v.prepare(c)?)).transpose()?;let n=self.data_mut(v)?;n.base_operations=prepared;Ok(())}
pub fn page_number<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.page_number)}
pub fn set_page_number<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.page_number=value;Ok(())}
pub fn unit<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<&'a Option<String>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(&n.unit)}
pub fn set_unit<P:Permission+Writes<types::Page>>(&self,v:&mut TreeView<'_,P>,value:Option<String>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.unit=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<PageInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(PageInit {
mediabox:n.mediabox.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
cropbox:n.cropbox.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
pdf_xobject:n.pdf_xobject.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
page_layout:n.page_layout.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_rectangle:n.pdf_rectangle.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_font:n.pdf_font.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_paragraph:n.pdf_paragraph.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_figure:n.pdf_figure.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_character:n.pdf_character.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_curve:n.pdf_curve.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
pdf_form:n.pdf_form.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
base_operations:n.base_operations.as_ref().map(|e|FieldValue::from_edge(e.share(),generation)),
page_number:n.page_number.clone(),
unit:n.unit.clone(),
})}}
#[derive(Default)] pub struct DataDocument {
pub page:Vec<Edge>,
pub total_pages:Option<i64>,
}
#[derive(Default)] pub struct DocumentInit {
pub page:Vec<FieldValue<types::Page>>,
pub total_pages:Option<i64>,
}
impl DocumentInit {pub fn into_value(self)->FieldValue<types::Document> {#[allow(unused_mut)] let mut binding=Binding::Fresh;
for child in &self.page {binding.merge(child.binding);}
FieldValue::new(NodeData::Document(DataDocument {
page:self.page.into_iter().map(|v|v.edge).collect(),
total_pages:self.total_pages,
}),binding)}}
impl Cursor<types::Document> {
pub fn page_len<P:Permission>(&self,v:&TreeView<'_,P>)->Result<usize,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.page.len())}
pub fn page_at<'a,P:Permission>(&self,v:&'a TreeView<'_,P>,i:usize)->Result<Cursor<types::Page>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Cursor::<types::Page>::from_edge(n.page.get(i).ok_or(AccessError::UnknownSlot)?, generation)}
pub fn page_iter<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<impl Iterator<Item=Result<Cursor<types::Page>,AccessError>>+'a,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.page.iter().map(move |edge|Cursor::<types::Page>::from_edge(edge, generation)))}
pub fn set_page<P:Permission+Writes<types::Document>>(&self,v:&mut TreeView<'_,P>,values:Vec<FieldValue<types::Page>>)->Result<(),AccessError>{self.validate(v)?;let prepared=values.into_iter().map(|c|Ok::<_,AccessError>(v.prepare(c)?)).collect::<Result<Vec<_>,_>>()?;let n=self.data_mut(v)?;n.page=prepared;Ok(())}
pub fn push_page<P:Permission+Writes<types::Document>>(&self,v:&mut TreeView<'_,P>,value:FieldValue<types::Page>)->Result<(),AccessError>{self.validate(v)?;let prepared=v.prepare(value)?;let n=self.data_mut(v)?;n.page.push(prepared);Ok(())}
pub fn total_pages<'a,P:Permission>(&self,v:&'a TreeView<'_,P>)->Result<Option<i64>,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(n.total_pages)}
pub fn set_total_pages<P:Permission+Writes<types::Document>>(&self,v:&mut TreeView<'_,P>,value:Option<i64>)->Result<(),AccessError>{let n=self.data_mut(v)?;n.total_pages=value;Ok(())}
pub fn copy_shallow<P:Permission>(&self,v:&mut TreeView<'_,P>)->Result<DocumentInit,AccessError>{#[allow(unused_variables)] let generation=v.document_ref().generation;let n=self.data(v)?;Ok(DocumentInit {
page:n.page.iter().map(|e|FieldValue::from_edge(e.share(),generation)).collect(),
total_pages:n.total_pages.clone(),
})}}
pub type ImmutableBaseOperations=ImmutableNode<DataBaseOperations>;
pub type ImmutableBox=ImmutableNode<DataBox>;
pub type ImmutableGraphicState=ImmutableNode<DataGraphicState>;
pub type ImmutablePdfAffineTransform=ImmutableNode<DataPdfAffineTransform>;
pub type ImmutablePdfFontCharBoundingBox=ImmutableNode<DataPdfFontCharBoundingBox>;
pub type ImmutablePdfInlineForm=ImmutableNode<DataPdfInlineForm>;
pub type ImmutablePdfMatrix=ImmutableNode<DataPdfMatrix>;
pub type ImmutablePdfPath=ImmutableNode<DataPdfPath>;
pub type ImmutablePdfXobjForm=ImmutableNode<DataPdfXobjForm>;
pub type ImmutableCropbox=ImmutableNode<DataCropbox>;
pub type ImmutableMediabox=ImmutableNode<DataMediabox>;
pub type ImmutablePageLayout=ImmutableNode<DataPageLayout>;
pub type ImmutablePdfFigure=ImmutableNode<DataPdfFigure>;
pub type ImmutablePdfFont=ImmutableNode<DataPdfFont>;
pub type ImmutablePdfFormSubtype=ImmutableNode<DataPdfFormSubtype>;
pub type ImmutablePdfOriginalPath=ImmutableNode<DataPdfOriginalPath>;
pub type ImmutablePdfOriginalPathPrimitive=ImmutableNode<DataPdfOriginalPathPrimitive>;
pub type ImmutablePdfRectangle=ImmutableNode<DataPdfRectangle>;
pub type ImmutablePdfStyle=ImmutableNode<DataPdfStyle>;
pub type ImmutableVisualBbox=ImmutableNode<DataVisualBbox>;
pub type ImmutablePdfCharacter=ImmutableNode<DataPdfCharacter>;
pub type ImmutablePdfCurve=ImmutableNode<DataPdfCurve>;
pub type ImmutablePdfForm=ImmutableNode<DataPdfForm>;
pub type ImmutablePdfSameStyleUnicodeCharacters=ImmutableNode<DataPdfSameStyleUnicodeCharacters>;
pub type ImmutablePdfXobject=ImmutableNode<DataPdfXobject>;
pub type ImmutablePdfFormula=ImmutableNode<DataPdfFormula>;
pub type ImmutablePdfLine=ImmutableNode<DataPdfLine>;
pub type ImmutablePdfSameStyleCharacters=ImmutableNode<DataPdfSameStyleCharacters>;
pub type ImmutablePdfParagraphComposition=ImmutableNode<DataPdfParagraphComposition>;
pub type ImmutablePdfParagraph=ImmutableNode<DataPdfParagraph>;
pub type ImmutablePage=ImmutableNode<DataPage>;
pub type ImmutableDocument=ImmutableNode<DataDocument>;
#[allow(unused_variables)] impl NodeData {pub fn kind(&self)->Kind {match self {
Self::BaseOperations(_)=>Kind::BaseOperations,
Self::Box(_)=>Kind::Box,
Self::GraphicState(_)=>Kind::GraphicState,
Self::PdfAffineTransform(_)=>Kind::PdfAffineTransform,
Self::PdfFontCharBoundingBox(_)=>Kind::PdfFontCharBoundingBox,
Self::PdfInlineForm(_)=>Kind::PdfInlineForm,
Self::PdfMatrix(_)=>Kind::PdfMatrix,
Self::PdfPath(_)=>Kind::PdfPath,
Self::PdfXobjForm(_)=>Kind::PdfXobjForm,
Self::Cropbox(_)=>Kind::Cropbox,
Self::Mediabox(_)=>Kind::Mediabox,
Self::PageLayout(_)=>Kind::PageLayout,
Self::PdfFigure(_)=>Kind::PdfFigure,
Self::PdfFont(_)=>Kind::PdfFont,
Self::PdfFormSubtype(_)=>Kind::PdfFormSubtype,
Self::PdfOriginalPath(_)=>Kind::PdfOriginalPath,
Self::PdfOriginalPathPrimitive(_)=>Kind::PdfOriginalPathPrimitive,
Self::PdfRectangle(_)=>Kind::PdfRectangle,
Self::PdfStyle(_)=>Kind::PdfStyle,
Self::VisualBbox(_)=>Kind::VisualBbox,
Self::PdfCharacter(_)=>Kind::PdfCharacter,
Self::PdfCurve(_)=>Kind::PdfCurve,
Self::PdfForm(_)=>Kind::PdfForm,
Self::PdfSameStyleUnicodeCharacters(_)=>Kind::PdfSameStyleUnicodeCharacters,
Self::PdfXobject(_)=>Kind::PdfXobject,
Self::PdfFormula(_)=>Kind::PdfFormula,
Self::PdfLine(_)=>Kind::PdfLine,
Self::PdfSameStyleCharacters(_)=>Kind::PdfSameStyleCharacters,
Self::PdfParagraphComposition(_)=>Kind::PdfParagraphComposition,
Self::PdfParagraph(_)=>Kind::PdfParagraph,
Self::Page(_)=>Kind::Page,
Self::Document(_)=>Kind::Document,
}}
pub fn edges_mut(&mut self)->Vec<&mut Edge>{let mut out=vec![];match self {
Self::BaseOperations(n)=>{
},
Self::Box(n)=>{
},
Self::GraphicState(n)=>{
},
Self::PdfAffineTransform(n)=>{
},
Self::PdfFontCharBoundingBox(n)=>{
},
Self::PdfInlineForm(n)=>{
},
Self::PdfMatrix(n)=>{
},
Self::PdfPath(n)=>{
},
Self::PdfXobjForm(n)=>{
},
Self::Cropbox(n)=>{
out.extend(n.r#box.iter_mut());
},
Self::Mediabox(n)=>{
out.extend(n.r#box.iter_mut());
},
Self::PageLayout(n)=>{
out.extend(n.r#box.iter_mut());
},
Self::PdfFigure(n)=>{
out.extend(n.r#box.iter_mut());
},
Self::PdfFont(n)=>{
out.extend(n.pdf_font_char_bounding_box.iter_mut());
},
Self::PdfFormSubtype(n)=>{
out.extend(n.pdf_inline_form.iter_mut());
out.extend(n.pdf_xobj_form.iter_mut());
},
Self::PdfOriginalPath(n)=>{
out.extend(n.pdf_path.iter_mut());
},
Self::PdfOriginalPathPrimitive(n)=>{
},
Self::PdfRectangle(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.graphic_state.iter_mut());
},
Self::PdfStyle(n)=>{
out.extend(n.graphic_state.iter_mut());
},
Self::VisualBbox(n)=>{
out.extend(n.r#box.iter_mut());
},
Self::PdfCharacter(n)=>{
out.extend(n.pdf_style.iter_mut());
out.extend(n.r#box.iter_mut());
out.extend(n.visual_bbox.iter_mut());
},
Self::PdfCurve(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.graphic_state.iter_mut());
out.extend(n.pdf_path.iter_mut());
out.extend(n.pdf_original_path.iter_mut());
out.extend(n.pdf_original_path_primitive.iter_mut());
},
Self::PdfForm(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.graphic_state.iter_mut());
out.extend(n.pdf_matrix.iter_mut());
out.extend(n.pdf_affine_transform.iter_mut());
out.extend(n.pdf_form_subtype.iter_mut());
},
Self::PdfSameStyleUnicodeCharacters(n)=>{
out.extend(n.pdf_style.iter_mut());
},
Self::PdfXobject(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.pdf_font.iter_mut());
out.extend(n.base_operations.iter_mut());
},
Self::PdfFormula(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.pdf_character.iter_mut());
out.extend(n.pdf_curve.iter_mut());
out.extend(n.pdf_form.iter_mut());
},
Self::PdfLine(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.pdf_character.iter_mut());
},
Self::PdfSameStyleCharacters(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.pdf_style.iter_mut());
out.extend(n.pdf_character.iter_mut());
},
Self::PdfParagraphComposition(n)=>{
out.extend(n.pdf_line.iter_mut());
out.extend(n.pdf_formula.iter_mut());
out.extend(n.pdf_same_style_characters.iter_mut());
out.extend(n.pdf_character.iter_mut());
out.extend(n.pdf_same_style_unicode_characters.iter_mut());
},
Self::PdfParagraph(n)=>{
out.extend(n.r#box.iter_mut());
out.extend(n.pdf_style.iter_mut());
out.extend(n.pdf_paragraph_composition.iter_mut());
},
Self::Page(n)=>{
out.extend(n.mediabox.iter_mut());
out.extend(n.cropbox.iter_mut());
out.extend(n.pdf_xobject.iter_mut());
out.extend(n.page_layout.iter_mut());
out.extend(n.pdf_rectangle.iter_mut());
out.extend(n.pdf_font.iter_mut());
out.extend(n.pdf_paragraph.iter_mut());
out.extend(n.pdf_figure.iter_mut());
out.extend(n.pdf_character.iter_mut());
out.extend(n.pdf_curve.iter_mut());
out.extend(n.pdf_form.iter_mut());
out.extend(n.base_operations.iter_mut());
},
Self::Document(n)=>{
out.extend(n.page.iter_mut());
},
}out}
pub fn edges(&self)->Vec<&Edge>{let mut out=vec![];match self {
Self::BaseOperations(n)=>{
},
Self::Box(n)=>{
},
Self::GraphicState(n)=>{
},
Self::PdfAffineTransform(n)=>{
},
Self::PdfFontCharBoundingBox(n)=>{
},
Self::PdfInlineForm(n)=>{
},
Self::PdfMatrix(n)=>{
},
Self::PdfPath(n)=>{
},
Self::PdfXobjForm(n)=>{
},
Self::Cropbox(n)=>{
out.extend(n.r#box.iter());
},
Self::Mediabox(n)=>{
out.extend(n.r#box.iter());
},
Self::PageLayout(n)=>{
out.extend(n.r#box.iter());
},
Self::PdfFigure(n)=>{
out.extend(n.r#box.iter());
},
Self::PdfFont(n)=>{
out.extend(n.pdf_font_char_bounding_box.iter());
},
Self::PdfFormSubtype(n)=>{
out.extend(n.pdf_inline_form.iter());
out.extend(n.pdf_xobj_form.iter());
},
Self::PdfOriginalPath(n)=>{
out.extend(n.pdf_path.iter());
},
Self::PdfOriginalPathPrimitive(n)=>{
},
Self::PdfRectangle(n)=>{
out.extend(n.r#box.iter());
out.extend(n.graphic_state.iter());
},
Self::PdfStyle(n)=>{
out.extend(n.graphic_state.iter());
},
Self::VisualBbox(n)=>{
out.extend(n.r#box.iter());
},
Self::PdfCharacter(n)=>{
out.extend(n.pdf_style.iter());
out.extend(n.r#box.iter());
out.extend(n.visual_bbox.iter());
},
Self::PdfCurve(n)=>{
out.extend(n.r#box.iter());
out.extend(n.graphic_state.iter());
out.extend(n.pdf_path.iter());
out.extend(n.pdf_original_path.iter());
out.extend(n.pdf_original_path_primitive.iter());
},
Self::PdfForm(n)=>{
out.extend(n.r#box.iter());
out.extend(n.graphic_state.iter());
out.extend(n.pdf_matrix.iter());
out.extend(n.pdf_affine_transform.iter());
out.extend(n.pdf_form_subtype.iter());
},
Self::PdfSameStyleUnicodeCharacters(n)=>{
out.extend(n.pdf_style.iter());
},
Self::PdfXobject(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_font.iter());
out.extend(n.base_operations.iter());
},
Self::PdfFormula(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_character.iter());
out.extend(n.pdf_curve.iter());
out.extend(n.pdf_form.iter());
},
Self::PdfLine(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_character.iter());
},
Self::PdfSameStyleCharacters(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_style.iter());
out.extend(n.pdf_character.iter());
},
Self::PdfParagraphComposition(n)=>{
out.extend(n.pdf_line.iter());
out.extend(n.pdf_formula.iter());
out.extend(n.pdf_same_style_characters.iter());
out.extend(n.pdf_character.iter());
out.extend(n.pdf_same_style_unicode_characters.iter());
},
Self::PdfParagraph(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_style.iter());
out.extend(n.pdf_paragraph_composition.iter());
},
Self::Page(n)=>{
out.extend(n.mediabox.iter());
out.extend(n.cropbox.iter());
out.extend(n.pdf_xobject.iter());
out.extend(n.page_layout.iter());
out.extend(n.pdf_rectangle.iter());
out.extend(n.pdf_font.iter());
out.extend(n.pdf_paragraph.iter());
out.extend(n.pdf_figure.iter());
out.extend(n.pdf_character.iter());
out.extend(n.pdf_curve.iter());
out.extend(n.pdf_form.iter());
out.extend(n.base_operations.iter());
},
Self::Document(n)=>{
out.extend(n.page.iter());
},
}out}
}
pub enum NodeDataRef<'a> {
BaseOperations(&'a DataBaseOperations),
Box(&'a DataBox),
GraphicState(&'a DataGraphicState),
PdfAffineTransform(&'a DataPdfAffineTransform),
PdfFontCharBoundingBox(&'a DataPdfFontCharBoundingBox),
PdfInlineForm(&'a DataPdfInlineForm),
PdfMatrix(&'a DataPdfMatrix),
PdfPath(&'a DataPdfPath),
PdfXobjForm(&'a DataPdfXobjForm),
Cropbox(&'a DataCropbox),
Mediabox(&'a DataMediabox),
PageLayout(&'a DataPageLayout),
PdfFigure(&'a DataPdfFigure),
PdfFont(&'a DataPdfFont),
PdfFormSubtype(&'a DataPdfFormSubtype),
PdfOriginalPath(&'a DataPdfOriginalPath),
PdfOriginalPathPrimitive(&'a DataPdfOriginalPathPrimitive),
PdfRectangle(&'a DataPdfRectangle),
PdfStyle(&'a DataPdfStyle),
VisualBbox(&'a DataVisualBbox),
PdfCharacter(&'a DataPdfCharacter),
PdfCurve(&'a DataPdfCurve),
PdfForm(&'a DataPdfForm),
PdfSameStyleUnicodeCharacters(&'a DataPdfSameStyleUnicodeCharacters),
PdfXobject(&'a DataPdfXobject),
PdfFormula(&'a DataPdfFormula),
PdfLine(&'a DataPdfLine),
PdfSameStyleCharacters(&'a DataPdfSameStyleCharacters),
PdfParagraphComposition(&'a DataPdfParagraphComposition),
PdfParagraph(&'a DataPdfParagraph),
Page(&'a DataPage),
Document(&'a DataDocument),
}
impl NodeData {fn as_ref(&self)->NodeDataRef<'_>{match self{
Self::BaseOperations(n)=>NodeDataRef::BaseOperations(n),
Self::Box(n)=>NodeDataRef::Box(n),
Self::GraphicState(n)=>NodeDataRef::GraphicState(n),
Self::PdfAffineTransform(n)=>NodeDataRef::PdfAffineTransform(n),
Self::PdfFontCharBoundingBox(n)=>NodeDataRef::PdfFontCharBoundingBox(n),
Self::PdfInlineForm(n)=>NodeDataRef::PdfInlineForm(n),
Self::PdfMatrix(n)=>NodeDataRef::PdfMatrix(n),
Self::PdfPath(n)=>NodeDataRef::PdfPath(n),
Self::PdfXobjForm(n)=>NodeDataRef::PdfXobjForm(n),
Self::Cropbox(n)=>NodeDataRef::Cropbox(n),
Self::Mediabox(n)=>NodeDataRef::Mediabox(n),
Self::PageLayout(n)=>NodeDataRef::PageLayout(n),
Self::PdfFigure(n)=>NodeDataRef::PdfFigure(n),
Self::PdfFont(n)=>NodeDataRef::PdfFont(n),
Self::PdfFormSubtype(n)=>NodeDataRef::PdfFormSubtype(n),
Self::PdfOriginalPath(n)=>NodeDataRef::PdfOriginalPath(n),
Self::PdfOriginalPathPrimitive(n)=>NodeDataRef::PdfOriginalPathPrimitive(n),
Self::PdfRectangle(n)=>NodeDataRef::PdfRectangle(n),
Self::PdfStyle(n)=>NodeDataRef::PdfStyle(n),
Self::VisualBbox(n)=>NodeDataRef::VisualBbox(n),
Self::PdfCharacter(n)=>NodeDataRef::PdfCharacter(n),
Self::PdfCurve(n)=>NodeDataRef::PdfCurve(n),
Self::PdfForm(n)=>NodeDataRef::PdfForm(n),
Self::PdfSameStyleUnicodeCharacters(n)=>NodeDataRef::PdfSameStyleUnicodeCharacters(n),
Self::PdfXobject(n)=>NodeDataRef::PdfXobject(n),
Self::PdfFormula(n)=>NodeDataRef::PdfFormula(n),
Self::PdfLine(n)=>NodeDataRef::PdfLine(n),
Self::PdfSameStyleCharacters(n)=>NodeDataRef::PdfSameStyleCharacters(n),
Self::PdfParagraphComposition(n)=>NodeDataRef::PdfParagraphComposition(n),
Self::PdfParagraph(n)=>NodeDataRef::PdfParagraph(n),
Self::Page(n)=>NodeDataRef::Page(n),
Self::Document(n)=>NodeDataRef::Document(n),
}}}
#[allow(unused_variables)] impl NodeDataRef<'_> {fn kind(&self)->Kind{match self{
Self::BaseOperations(_)=>Kind::BaseOperations,
Self::Box(_)=>Kind::Box,
Self::GraphicState(_)=>Kind::GraphicState,
Self::PdfAffineTransform(_)=>Kind::PdfAffineTransform,
Self::PdfFontCharBoundingBox(_)=>Kind::PdfFontCharBoundingBox,
Self::PdfInlineForm(_)=>Kind::PdfInlineForm,
Self::PdfMatrix(_)=>Kind::PdfMatrix,
Self::PdfPath(_)=>Kind::PdfPath,
Self::PdfXobjForm(_)=>Kind::PdfXobjForm,
Self::Cropbox(_)=>Kind::Cropbox,
Self::Mediabox(_)=>Kind::Mediabox,
Self::PageLayout(_)=>Kind::PageLayout,
Self::PdfFigure(_)=>Kind::PdfFigure,
Self::PdfFont(_)=>Kind::PdfFont,
Self::PdfFormSubtype(_)=>Kind::PdfFormSubtype,
Self::PdfOriginalPath(_)=>Kind::PdfOriginalPath,
Self::PdfOriginalPathPrimitive(_)=>Kind::PdfOriginalPathPrimitive,
Self::PdfRectangle(_)=>Kind::PdfRectangle,
Self::PdfStyle(_)=>Kind::PdfStyle,
Self::VisualBbox(_)=>Kind::VisualBbox,
Self::PdfCharacter(_)=>Kind::PdfCharacter,
Self::PdfCurve(_)=>Kind::PdfCurve,
Self::PdfForm(_)=>Kind::PdfForm,
Self::PdfSameStyleUnicodeCharacters(_)=>Kind::PdfSameStyleUnicodeCharacters,
Self::PdfXobject(_)=>Kind::PdfXobject,
Self::PdfFormula(_)=>Kind::PdfFormula,
Self::PdfLine(_)=>Kind::PdfLine,
Self::PdfSameStyleCharacters(_)=>Kind::PdfSameStyleCharacters,
Self::PdfParagraphComposition(_)=>Kind::PdfParagraphComposition,
Self::PdfParagraph(_)=>Kind::PdfParagraph,
Self::Page(_)=>Kind::Page,
Self::Document(_)=>Kind::Document,
}} fn edges(&self)->Vec<&Edge>{let mut out=vec![];match self{
Self::BaseOperations(n)=>{
},
Self::Box(n)=>{
},
Self::GraphicState(n)=>{
},
Self::PdfAffineTransform(n)=>{
},
Self::PdfFontCharBoundingBox(n)=>{
},
Self::PdfInlineForm(n)=>{
},
Self::PdfMatrix(n)=>{
},
Self::PdfPath(n)=>{
},
Self::PdfXobjForm(n)=>{
},
Self::Cropbox(n)=>{
out.extend(n.r#box.iter());
},
Self::Mediabox(n)=>{
out.extend(n.r#box.iter());
},
Self::PageLayout(n)=>{
out.extend(n.r#box.iter());
},
Self::PdfFigure(n)=>{
out.extend(n.r#box.iter());
},
Self::PdfFont(n)=>{
out.extend(n.pdf_font_char_bounding_box.iter());
},
Self::PdfFormSubtype(n)=>{
out.extend(n.pdf_inline_form.iter());
out.extend(n.pdf_xobj_form.iter());
},
Self::PdfOriginalPath(n)=>{
out.extend(n.pdf_path.iter());
},
Self::PdfOriginalPathPrimitive(n)=>{
},
Self::PdfRectangle(n)=>{
out.extend(n.r#box.iter());
out.extend(n.graphic_state.iter());
},
Self::PdfStyle(n)=>{
out.extend(n.graphic_state.iter());
},
Self::VisualBbox(n)=>{
out.extend(n.r#box.iter());
},
Self::PdfCharacter(n)=>{
out.extend(n.pdf_style.iter());
out.extend(n.r#box.iter());
out.extend(n.visual_bbox.iter());
},
Self::PdfCurve(n)=>{
out.extend(n.r#box.iter());
out.extend(n.graphic_state.iter());
out.extend(n.pdf_path.iter());
out.extend(n.pdf_original_path.iter());
out.extend(n.pdf_original_path_primitive.iter());
},
Self::PdfForm(n)=>{
out.extend(n.r#box.iter());
out.extend(n.graphic_state.iter());
out.extend(n.pdf_matrix.iter());
out.extend(n.pdf_affine_transform.iter());
out.extend(n.pdf_form_subtype.iter());
},
Self::PdfSameStyleUnicodeCharacters(n)=>{
out.extend(n.pdf_style.iter());
},
Self::PdfXobject(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_font.iter());
out.extend(n.base_operations.iter());
},
Self::PdfFormula(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_character.iter());
out.extend(n.pdf_curve.iter());
out.extend(n.pdf_form.iter());
},
Self::PdfLine(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_character.iter());
},
Self::PdfSameStyleCharacters(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_style.iter());
out.extend(n.pdf_character.iter());
},
Self::PdfParagraphComposition(n)=>{
out.extend(n.pdf_line.iter());
out.extend(n.pdf_formula.iter());
out.extend(n.pdf_same_style_characters.iter());
out.extend(n.pdf_character.iter());
out.extend(n.pdf_same_style_unicode_characters.iter());
},
Self::PdfParagraph(n)=>{
out.extend(n.r#box.iter());
out.extend(n.pdf_style.iter());
out.extend(n.pdf_paragraph_composition.iter());
},
Self::Page(n)=>{
out.extend(n.mediabox.iter());
out.extend(n.cropbox.iter());
out.extend(n.pdf_xobject.iter());
out.extend(n.page_layout.iter());
out.extend(n.pdf_rectangle.iter());
out.extend(n.pdf_font.iter());
out.extend(n.pdf_paragraph.iter());
out.extend(n.pdf_figure.iter());
out.extend(n.pdf_character.iter());
out.extend(n.pdf_curve.iter());
out.extend(n.pdf_form.iter());
out.extend(n.base_operations.iter());
},
Self::Document(n)=>{
out.extend(n.page.iter());
},
}out}
fn dump(&self,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();match self {
Self::BaseOperations(n)=>{
out.insert("value".into(),serde_json::to_value(&n.value).map_err(|_|AccessError::WrongType)?);
},
Self::Box(n)=>{
out.insert("x".into(),n.x.map(float_artifact).unwrap_or(Json::Null));
out.insert("y".into(),n.y.map(float_artifact).unwrap_or(Json::Null));
out.insert("x2".into(),n.x2.map(float_artifact).unwrap_or(Json::Null));
out.insert("y2".into(),n.y2.map(float_artifact).unwrap_or(Json::Null));
},
Self::GraphicState(n)=>{
out.insert("passthrough_per_char_instruction".into(),d.instructions(&n.passthrough_per_char_instruction,format!("{path}.passthrough_per_char_instruction"))?);
},
Self::PdfAffineTransform(n)=>{
out.insert("translation_x".into(),n.translation_x.map(float_artifact).unwrap_or(Json::Null));
out.insert("translation_y".into(),n.translation_y.map(float_artifact).unwrap_or(Json::Null));
out.insert("rotation".into(),n.rotation.map(float_artifact).unwrap_or(Json::Null));
out.insert("scale_x".into(),n.scale_x.map(float_artifact).unwrap_or(Json::Null));
out.insert("scale_y".into(),n.scale_y.map(float_artifact).unwrap_or(Json::Null));
out.insert("shear".into(),n.shear.map(float_artifact).unwrap_or(Json::Null));
},
Self::PdfFontCharBoundingBox(n)=>{
out.insert("x".into(),n.x.map(float_artifact).unwrap_or(Json::Null));
out.insert("y".into(),n.y.map(float_artifact).unwrap_or(Json::Null));
out.insert("x2".into(),n.x2.map(float_artifact).unwrap_or(Json::Null));
out.insert("y2".into(),n.y2.map(float_artifact).unwrap_or(Json::Null));
out.insert("char_id".into(),serde_json::to_value(&n.char_id).map_err(|_|AccessError::WrongType)?);
},
Self::PdfInlineForm(n)=>{
out.insert("form_data".into(),serde_json::to_value(&n.form_data).map_err(|_|AccessError::WrongType)?);
out.insert("image_parameters".into(),serde_json::to_value(&n.image_parameters).map_err(|_|AccessError::WrongType)?);
},
Self::PdfMatrix(n)=>{
out.insert("a".into(),n.a.map(float_artifact).unwrap_or(Json::Null));
out.insert("b".into(),n.b.map(float_artifact).unwrap_or(Json::Null));
out.insert("c".into(),n.c.map(float_artifact).unwrap_or(Json::Null));
out.insert("d".into(),n.d.map(float_artifact).unwrap_or(Json::Null));
out.insert("e".into(),n.e.map(float_artifact).unwrap_or(Json::Null));
out.insert("f".into(),n.f.map(float_artifact).unwrap_or(Json::Null));
},
Self::PdfPath(n)=>{
out.insert("x".into(),n.x().map(float_artifact).unwrap_or(Json::Null));
out.insert("y".into(),n.y().map(float_artifact).unwrap_or(Json::Null));
out.insert("op".into(),serde_json::to_value(&n.op).map_err(|_|AccessError::WrongType)?);
out.insert("has_xy".into(),serde_json::to_value(&n.has_xy()).map_err(|_|AccessError::WrongType)?);
},
Self::PdfXobjForm(n)=>{
out.insert("xref_id".into(),serde_json::to_value(&n.xref_id).map_err(|_|AccessError::WrongType)?);
out.insert("do_args".into(),serde_json::to_value(&n.do_args).map_err(|_|AccessError::WrongType)?);
},
Self::Cropbox(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
},
Self::Mediabox(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
},
Self::PageLayout(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("id".into(),serde_json::to_value(&n.id).map_err(|_|AccessError::WrongType)?);
out.insert("conf".into(),n.conf.map(float_artifact).unwrap_or(Json::Null));
out.insert("class_name".into(),serde_json::to_value(&n.class_name).map_err(|_|AccessError::WrongType)?);
},
Self::PdfFigure(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
},
Self::PdfFont(n)=>{
out.insert("pdf_font_char_bounding_box".into(),Json::Array(n.pdf_font_char_bounding_box.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_font_char_bounding_box[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("name".into(),serde_json::to_value(&n.name).map_err(|_|AccessError::WrongType)?);
out.insert("font_id".into(),serde_json::to_value(&n.font_id).map_err(|_|AccessError::WrongType)?);
out.insert("xref_id".into(),serde_json::to_value(&n.xref_id).map_err(|_|AccessError::WrongType)?);
out.insert("encoding_length".into(),serde_json::to_value(&n.encoding_length).map_err(|_|AccessError::WrongType)?);
out.insert("font_subtype".into(),serde_json::to_value(&n.font_subtype).map_err(|_|AccessError::WrongType)?);
out.insert("type3_font_matrix".into(),serde_json::to_value(&n.type3_font_matrix).map_err(|_|AccessError::WrongType)?);
out.insert("type3_font_bbox".into(),serde_json::to_value(&n.type3_font_bbox).map_err(|_|AccessError::WrongType)?);
out.insert("type3_em_height".into(),n.type3_em_height.map(float_artifact).unwrap_or(Json::Null));
out.insert("bold".into(),serde_json::to_value(&n.bold).map_err(|_|AccessError::WrongType)?);
out.insert("italic".into(),serde_json::to_value(&n.italic).map_err(|_|AccessError::WrongType)?);
out.insert("monospace".into(),serde_json::to_value(&n.monospace).map_err(|_|AccessError::WrongType)?);
out.insert("serif".into(),serde_json::to_value(&n.serif).map_err(|_|AccessError::WrongType)?);
out.insert("ascent".into(),n.ascent.map(float_artifact).unwrap_or(Json::Null));
out.insert("descent".into(),n.descent.map(float_artifact).unwrap_or(Json::Null));
},
Self::PdfFormSubtype(n)=>{
out.insert("pdf_inline_form".into(),n.pdf_inline_form.as_ref().map(|c|d.edge(c,format!("{path}.pdf_inline_form"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_xobj_form".into(),n.pdf_xobj_form.as_ref().map(|c|d.edge(c,format!("{path}.pdf_xobj_form"))).transpose()?.unwrap_or(Json::Null));
},
Self::PdfOriginalPath(n)=>{
out.insert("pdf_path".into(),n.pdf_path.as_ref().map(|c|d.edge(c,format!("{path}.pdf_path"))).transpose()?.unwrap_or(Json::Null));
},
Self::PdfOriginalPathPrimitive(n)=>{
out.insert("op".into(),serde_json::to_value(&n.op).map_err(|_|AccessError::WrongType)?);
out.insert("args".into(),serde_json::to_value(&n.args).map_err(|_|AccessError::WrongType)?);
},
Self::PdfRectangle(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("graphic_state".into(),n.graphic_state.as_ref().map(|c|d.edge(c,format!("{path}.graphic_state"))).transpose()?.unwrap_or(Json::Null));
out.insert("debug_info".into(),serde_json::to_value(&n.debug_info).map_err(|_|AccessError::WrongType)?);
out.insert("fill_background".into(),serde_json::to_value(&n.fill_background).map_err(|_|AccessError::WrongType)?);
out.insert("xobj_id".into(),serde_json::to_value(&n.xobj_id).map_err(|_|AccessError::WrongType)?);
out.insert("line_width".into(),n.line_width.map(float_artifact).unwrap_or(Json::Null));
out.insert("render_order".into(),serde_json::to_value(&n.render_order).map_err(|_|AccessError::WrongType)?);
},
Self::PdfStyle(n)=>{
out.insert("graphic_state".into(),n.graphic_state.as_ref().map(|c|d.edge(c,format!("{path}.graphic_state"))).transpose()?.unwrap_or(Json::Null));
out.insert("font_id".into(),serde_json::to_value(&n.font_id).map_err(|_|AccessError::WrongType)?);
out.insert("font_size".into(),n.font_size.map(float_artifact).unwrap_or(Json::Null));
},
Self::VisualBbox(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
},
Self::PdfCharacter(n)=>{
out.insert("pdf_style".into(),n.pdf_style.as_ref().map(|c|d.edge(c,format!("{path}.pdf_style"))).transpose()?.unwrap_or(Json::Null));
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("visual_bbox".into(),n.visual_bbox.as_ref().map(|c|d.edge(c,format!("{path}.visual_bbox"))).transpose()?.unwrap_or(Json::Null));
out.insert("vertical".into(),serde_json::to_value(&n.vertical).map_err(|_|AccessError::WrongType)?);
out.insert("scale".into(),n.scale.map(float_artifact).unwrap_or(Json::Null));
out.insert("visual_roundtrip_text_scale_x".into(),n.visual_roundtrip_text_scale_x.map(float_artifact).unwrap_or(Json::Null));
out.insert("visual_roundtrip_text_scale_y".into(),n.visual_roundtrip_text_scale_y.map(float_artifact).unwrap_or(Json::Null));
out.insert("visual_roundtrip_text_render_mode".into(),serde_json::to_value(&n.visual_roundtrip_text_render_mode).map_err(|_|AccessError::WrongType)?);
out.insert("pdf_character_id".into(),serde_json::to_value(&n.pdf_character_id).map_err(|_|AccessError::WrongType)?);
out.insert("char_unicode".into(),serde_json::to_value(&n.char_unicode).map_err(|_|AccessError::WrongType)?);
out.insert("advance".into(),n.advance.map(float_artifact).unwrap_or(Json::Null));
out.insert("xobj_id".into(),serde_json::to_value(&n.xobj_id).map_err(|_|AccessError::WrongType)?);
out.insert("debug_info".into(),serde_json::to_value(&n.debug_info).map_err(|_|AccessError::WrongType)?);
out.insert("formula_layout_id".into(),serde_json::to_value(&n.formula_layout_id).map_err(|_|AccessError::WrongType)?);
out.insert("render_order".into(),serde_json::to_value(&n.render_order).map_err(|_|AccessError::WrongType)?);
out.insert("sub_render_order".into(),serde_json::to_value(&n.sub_render_order).map_err(|_|AccessError::WrongType)?);
},
Self::PdfCurve(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("graphic_state".into(),n.graphic_state.as_ref().map(|c|d.edge(c,format!("{path}.graphic_state"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_path".into(),Json::Array(n.pdf_path.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_path[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_original_path".into(),Json::Array(n.pdf_original_path.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_original_path[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_original_path_primitive".into(),n.pdf_original_path_primitive.as_ref().map(|c|d.edge(c,format!("{path}.pdf_original_path_primitive"))).transpose()?.unwrap_or(Json::Null));
out.insert("debug_info".into(),serde_json::to_value(&n.debug_info).map_err(|_|AccessError::WrongType)?);
out.insert("fill_background".into(),serde_json::to_value(&n.fill_background).map_err(|_|AccessError::WrongType)?);
out.insert("stroke_path".into(),serde_json::to_value(&n.stroke_path).map_err(|_|AccessError::WrongType)?);
out.insert("evenodd".into(),serde_json::to_value(&n.evenodd).map_err(|_|AccessError::WrongType)?);
out.insert("passthrough_paint".into(),serde_json::to_value(&n.passthrough_paint).map_err(|_|AccessError::WrongType)?);
out.insert("xobj_id".into(),serde_json::to_value(&n.xobj_id).map_err(|_|AccessError::WrongType)?);
out.insert("render_order".into(),serde_json::to_value(&n.render_order).map_err(|_|AccessError::WrongType)?);
out.insert("ctm".into(),serde_json::to_value(&n.ctm).map_err(|_|AccessError::WrongType)?);
out.insert("relocation_transform".into(),serde_json::to_value(&n.relocation_transform).map_err(|_|AccessError::WrongType)?);
},
Self::PdfForm(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("graphic_state".into(),n.graphic_state.as_ref().map(|c|d.edge(c,format!("{path}.graphic_state"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_matrix".into(),n.pdf_matrix.as_ref().map(|c|d.edge(c,format!("{path}.pdf_matrix"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_affine_transform".into(),n.pdf_affine_transform.as_ref().map(|c|d.edge(c,format!("{path}.pdf_affine_transform"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_form_subtype".into(),n.pdf_form_subtype.as_ref().map(|c|d.edge(c,format!("{path}.pdf_form_subtype"))).transpose()?.unwrap_or(Json::Null));
out.insert("xobj_id".into(),serde_json::to_value(&n.xobj_id).map_err(|_|AccessError::WrongType)?);
out.insert("ctm".into(),serde_json::to_value(&n.ctm).map_err(|_|AccessError::WrongType)?);
out.insert("relocation_transform".into(),serde_json::to_value(&n.relocation_transform).map_err(|_|AccessError::WrongType)?);
out.insert("render_order".into(),serde_json::to_value(&n.render_order).map_err(|_|AccessError::WrongType)?);
out.insert("form_type".into(),serde_json::to_value(&n.form_type).map_err(|_|AccessError::WrongType)?);
},
Self::PdfSameStyleUnicodeCharacters(n)=>{
out.insert("pdf_style".into(),n.pdf_style.as_ref().map(|c|d.edge(c,format!("{path}.pdf_style"))).transpose()?.unwrap_or(Json::Null));
out.insert("unicode".into(),serde_json::to_value(&n.unicode).map_err(|_|AccessError::WrongType)?);
out.insert("debug_info".into(),serde_json::to_value(&n.debug_info).map_err(|_|AccessError::WrongType)?);
},
Self::PdfXobject(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_font".into(),Json::Array(n.pdf_font.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_font[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("base_operations".into(),n.base_operations.as_ref().map(|c|d.edge(c,format!("{path}.base_operations"))).transpose()?.unwrap_or(Json::Null));
out.insert("xobj_id".into(),serde_json::to_value(&n.xobj_id).map_err(|_|AccessError::WrongType)?);
out.insert("xref_id".into(),serde_json::to_value(&n.xref_id).map_err(|_|AccessError::WrongType)?);
},
Self::PdfFormula(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_character".into(),Json::Array(n.pdf_character.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_character[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_curve".into(),Json::Array(n.pdf_curve.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_curve[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_form".into(),Json::Array(n.pdf_form.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_form[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("x_offset".into(),n.x_offset.map(float_artifact).unwrap_or(Json::Null));
out.insert("y_offset".into(),n.y_offset.map(float_artifact).unwrap_or(Json::Null));
out.insert("x_advance".into(),n.x_advance.map(float_artifact).unwrap_or(Json::Null));
out.insert("line_id".into(),serde_json::to_value(&n.line_id).map_err(|_|AccessError::WrongType)?);
out.insert("is_corner_mark".into(),serde_json::to_value(&n.is_corner_mark).map_err(|_|AccessError::WrongType)?);
},
Self::PdfLine(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_character".into(),Json::Array(n.pdf_character.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_character[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("render_order".into(),serde_json::to_value(&n.render_order).map_err(|_|AccessError::WrongType)?);
},
Self::PdfSameStyleCharacters(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_style".into(),n.pdf_style.as_ref().map(|c|d.edge(c,format!("{path}.pdf_style"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_character".into(),Json::Array(n.pdf_character.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_character[{i}]"))).collect::<Result<Vec<_>,_>>()?));
},
Self::PdfParagraphComposition(n)=>{
out.insert("pdf_line".into(),n.pdf_line.as_ref().map(|c|d.edge(c,format!("{path}.pdf_line"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_formula".into(),n.pdf_formula.as_ref().map(|c|d.edge(c,format!("{path}.pdf_formula"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_same_style_characters".into(),n.pdf_same_style_characters.as_ref().map(|c|d.edge(c,format!("{path}.pdf_same_style_characters"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_character".into(),n.pdf_character.as_ref().map(|c|d.edge(c,format!("{path}.pdf_character"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_same_style_unicode_characters".into(),n.pdf_same_style_unicode_characters.as_ref().map(|c|d.edge(c,format!("{path}.pdf_same_style_unicode_characters"))).transpose()?.unwrap_or(Json::Null));
},
Self::PdfParagraph(n)=>{
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_style".into(),n.pdf_style.as_ref().map(|c|d.edge(c,format!("{path}.pdf_style"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_paragraph_composition".into(),Json::Array(n.pdf_paragraph_composition.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_paragraph_composition[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("xobj_id".into(),serde_json::to_value(&n.xobj_id).map_err(|_|AccessError::WrongType)?);
out.insert("unicode".into(),serde_json::to_value(&n.unicode).map_err(|_|AccessError::WrongType)?);
out.insert("scale".into(),n.scale.map(float_artifact).unwrap_or(Json::Null));
out.insert("optimal_scale".into(),n.optimal_scale.map(float_artifact).unwrap_or(Json::Null));
out.insert("vertical".into(),serde_json::to_value(&n.vertical).map_err(|_|AccessError::WrongType)?);
out.insert("first_line_indent".into(),serde_json::to_value(&n.first_line_indent).map_err(|_|AccessError::WrongType)?);
out.insert("debug_id".into(),serde_json::to_value(&n.debug_id).map_err(|_|AccessError::WrongType)?);
out.insert("layout_label".into(),serde_json::to_value(&n.layout_label).map_err(|_|AccessError::WrongType)?);
out.insert("layout_id".into(),serde_json::to_value(&n.layout_id).map_err(|_|AccessError::WrongType)?);
out.insert("render_order".into(),serde_json::to_value(&n.render_order).map_err(|_|AccessError::WrongType)?);
},
Self::Page(n)=>{
out.insert("mediabox".into(),n.mediabox.as_ref().map(|c|d.edge(c,format!("{path}.mediabox"))).transpose()?.unwrap_or(Json::Null));
out.insert("cropbox".into(),n.cropbox.as_ref().map(|c|d.edge(c,format!("{path}.cropbox"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_xobject".into(),Json::Array(n.pdf_xobject.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_xobject[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("page_layout".into(),Json::Array(n.page_layout.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.page_layout[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_rectangle".into(),Json::Array(n.pdf_rectangle.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_rectangle[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_font".into(),Json::Array(n.pdf_font.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_font[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_paragraph".into(),Json::Array(n.pdf_paragraph.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_paragraph[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_figure".into(),Json::Array(n.pdf_figure.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_figure[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_character".into(),Json::Array(n.pdf_character.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_character[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_curve".into(),Json::Array(n.pdf_curve.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_curve[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("pdf_form".into(),Json::Array(n.pdf_form.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.pdf_form[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("base_operations".into(),n.base_operations.as_ref().map(|c|d.edge(c,format!("{path}.base_operations"))).transpose()?.unwrap_or(Json::Null));
out.insert("page_number".into(),serde_json::to_value(&n.page_number).map_err(|_|AccessError::WrongType)?);
out.insert("unit".into(),serde_json::to_value(&n.unit).map_err(|_|AccessError::WrongType)?);
},
Self::Document(n)=>{
out.insert("page".into(),Json::Array(n.page.iter().enumerate().map(|(i,c)|d.edge(c,format!("{path}.page[{i}]"))).collect::<Result<Vec<_>,_>>()?));
out.insert("total_pages".into(),serde_json::to_value(&n.total_pages).map_err(|_|AccessError::WrongType)?);
},
}Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write<W:std::io::Write>(&self,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;match self {
Self::BaseOperations(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"value\":").map_err(|_|AccessError::WrongType)?;{let value=decode_base_operations(&n.value)?;serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::Box(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x\":").map_err(|_|AccessError::WrongType)?;{let value=n.x.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x2\":").map_err(|_|AccessError::WrongType)?;{let value=n.x2.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"y\":").map_err(|_|AccessError::WrongType)?;{let value=n.y.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"y2\":").map_err(|_|AccessError::WrongType)?;{let value=n.y2.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::GraphicState(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"passthrough_per_char_instruction\":").map_err(|_|AccessError::WrongType)?;{let value=d.instructions(&n.passthrough_per_char_instruction,format!("{path}.passthrough_per_char_instruction"))?;serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfAffineTransform(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"rotation\":").map_err(|_|AccessError::WrongType)?;{let value=n.rotation.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"scale_x\":").map_err(|_|AccessError::WrongType)?;{let value=n.scale_x.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"scale_y\":").map_err(|_|AccessError::WrongType)?;{let value=n.scale_y.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"shear\":").map_err(|_|AccessError::WrongType)?;{let value=n.shear.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"translation_x\":").map_err(|_|AccessError::WrongType)?;{let value=n.translation_x.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"translation_y\":").map_err(|_|AccessError::WrongType)?;{let value=n.translation_y.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfFontCharBoundingBox(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"char_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.char_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x\":").map_err(|_|AccessError::WrongType)?;{let value=n.x.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x2\":").map_err(|_|AccessError::WrongType)?;{let value=n.x2.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"y\":").map_err(|_|AccessError::WrongType)?;{let value=n.y.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"y2\":").map_err(|_|AccessError::WrongType)?;{let value=n.y2.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfInlineForm(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"form_data\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.form_data).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"image_parameters\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.image_parameters).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfMatrix(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"a\":").map_err(|_|AccessError::WrongType)?;{let value=n.a.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"b\":").map_err(|_|AccessError::WrongType)?;{let value=n.b.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"c\":").map_err(|_|AccessError::WrongType)?;{let value=n.c.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"d\":").map_err(|_|AccessError::WrongType)?;{let value=n.d.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"e\":").map_err(|_|AccessError::WrongType)?;{let value=n.e.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"f\":").map_err(|_|AccessError::WrongType)?;{let value=n.f.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfPath(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"has_xy\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.has_xy()).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"op\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.op).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x\":").map_err(|_|AccessError::WrongType)?;{let value=n.x().map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"y\":").map_err(|_|AccessError::WrongType)?;{let value=n.y().map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfXobjForm(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"do_args\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.do_args).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xref_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xref_id).map_err(|_|AccessError::WrongType)?;}
},
Self::Cropbox(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::Mediabox(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::PageLayout(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"class_name\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.class_name).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"conf\":").map_err(|_|AccessError::WrongType)?;{let value=n.conf.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.id).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfFigure(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::PdfFont(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"ascent\":").map_err(|_|AccessError::WrongType)?;{let value=n.ascent.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"bold\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.bold).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"descent\":").map_err(|_|AccessError::WrongType)?;{let value=n.descent.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"encoding_length\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.encoding_length).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"font_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.font_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"font_subtype\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.font_subtype).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"italic\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.italic).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"monospace\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.monospace).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"name\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.name).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_font_char_bounding_box\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_font_char_bounding_box.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_font_char_bounding_box[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"serif\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.serif).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"type3_em_height\":").map_err(|_|AccessError::WrongType)?;{let value=n.type3_em_height.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"type3_font_bbox\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.type3_font_bbox).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"type3_font_matrix\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.type3_font_matrix).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xref_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xref_id).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfFormSubtype(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_inline_form\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_inline_form{d.write_edge(e,&format!("{path}.pdf_inline_form"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_xobj_form\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_xobj_form{d.write_edge(e,&format!("{path}.pdf_xobj_form"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::PdfOriginalPath(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_path\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_path{d.write_edge(e,&format!("{path}.pdf_path"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::PdfOriginalPathPrimitive(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"args\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.args).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"op\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.op).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfRectangle(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"debug_info\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.debug_info).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"fill_background\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.fill_background).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"graphic_state\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.graphic_state{d.write_edge(e,&format!("{path}.graphic_state"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"line_width\":").map_err(|_|AccessError::WrongType)?;{let value=n.line_width.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"render_order\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.render_order).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xobj_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xobj_id).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfStyle(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"font_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.font_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"font_size\":").map_err(|_|AccessError::WrongType)?;{let value=n.font_size.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"graphic_state\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.graphic_state{d.write_edge(e,&format!("{path}.graphic_state"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::VisualBbox(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::PdfCharacter(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"advance\":").map_err(|_|AccessError::WrongType)?;{let value=n.advance.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"char_unicode\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.char_unicode).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"debug_info\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.debug_info).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"formula_layout_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.formula_layout_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_character_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.pdf_character_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_style\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_style{d.write_edge(e,&format!("{path}.pdf_style"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"render_order\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.render_order).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"scale\":").map_err(|_|AccessError::WrongType)?;{let value=n.scale.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"sub_render_order\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.sub_render_order).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"vertical\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.vertical).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"visual_bbox\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.visual_bbox{d.write_edge(e,&format!("{path}.visual_bbox"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"visual_roundtrip_text_render_mode\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.visual_roundtrip_text_render_mode).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"visual_roundtrip_text_scale_x\":").map_err(|_|AccessError::WrongType)?;{let value=n.visual_roundtrip_text_scale_x.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"visual_roundtrip_text_scale_y\":").map_err(|_|AccessError::WrongType)?;{let value=n.visual_roundtrip_text_scale_y.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xobj_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xobj_id).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfCurve(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"ctm\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.ctm).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"debug_info\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.debug_info).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"evenodd\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.evenodd).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"fill_background\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.fill_background).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"graphic_state\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.graphic_state{d.write_edge(e,&format!("{path}.graphic_state"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"passthrough_paint\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.passthrough_paint).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_original_path\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_original_path.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_original_path[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_original_path_primitive\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_original_path_primitive{d.write_edge(e,&format!("{path}.pdf_original_path_primitive"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_path\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_path.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_path[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"relocation_transform\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.relocation_transform).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"render_order\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.render_order).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"stroke_path\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.stroke_path).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xobj_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xobj_id).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfForm(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"ctm\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.ctm).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"form_type\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.form_type).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"graphic_state\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.graphic_state{d.write_edge(e,&format!("{path}.graphic_state"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_affine_transform\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_affine_transform{d.write_edge(e,&format!("{path}.pdf_affine_transform"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_form_subtype\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_form_subtype{d.write_edge(e,&format!("{path}.pdf_form_subtype"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_matrix\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_matrix{d.write_edge(e,&format!("{path}.pdf_matrix"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"relocation_transform\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.relocation_transform).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"render_order\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.render_order).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xobj_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xobj_id).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfSameStyleUnicodeCharacters(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"debug_info\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.debug_info).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_style\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_style{d.write_edge(e,&format!("{path}.pdf_style"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"unicode\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.unicode).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfXobject(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"base_operations\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.base_operations{d.write_edge(e,&format!("{path}.base_operations"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_font\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_font.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_font[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xobj_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xobj_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xref_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xref_id).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfFormula(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"is_corner_mark\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.is_corner_mark).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"line_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.line_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_character\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_character.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_character[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_curve\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_curve.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_curve[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_form\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_form.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_form[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x_advance\":").map_err(|_|AccessError::WrongType)?;{let value=n.x_advance.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x_offset\":").map_err(|_|AccessError::WrongType)?;{let value=n.x_offset.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"y_offset\":").map_err(|_|AccessError::WrongType)?;{let value=n.y_offset.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfLine(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_character\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_character.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_character[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"render_order\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.render_order).map_err(|_|AccessError::WrongType)?;}
},
Self::PdfSameStyleCharacters(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_character\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_character.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_character[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_style\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_style{d.write_edge(e,&format!("{path}.pdf_style"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::PdfParagraphComposition(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_character\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_character{d.write_edge(e,&format!("{path}.pdf_character"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_formula\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_formula{d.write_edge(e,&format!("{path}.pdf_formula"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_line\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_line{d.write_edge(e,&format!("{path}.pdf_line"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_same_style_characters\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_same_style_characters{d.write_edge(e,&format!("{path}.pdf_same_style_characters"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_same_style_unicode_characters\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_same_style_unicode_characters{d.write_edge(e,&format!("{path}.pdf_same_style_unicode_characters"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
},
Self::PdfParagraph(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"debug_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.debug_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"first_line_indent\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.first_line_indent).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"layout_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.layout_id).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"layout_label\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.layout_label).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"optimal_scale\":").map_err(|_|AccessError::WrongType)?;{let value=n.optimal_scale.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_paragraph_composition\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_paragraph_composition.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_paragraph_composition[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_style\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_style{d.write_edge(e,&format!("{path}.pdf_style"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"render_order\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.render_order).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"scale\":").map_err(|_|AccessError::WrongType)?;{let value=n.scale.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"unicode\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.unicode).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"vertical\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.vertical).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xobj_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xobj_id).map_err(|_|AccessError::WrongType)?;}
},
Self::Page(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"base_operations\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.base_operations{d.write_edge(e,&format!("{path}.base_operations"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"cropbox\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.cropbox{d.write_edge(e,&format!("{path}.cropbox"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"mediabox\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.mediabox{d.write_edge(e,&format!("{path}.mediabox"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"page_layout\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.page_layout.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.page_layout[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"page_number\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.page_number).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_character\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_character.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_character[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_curve\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_curve.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_curve[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_figure\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_figure.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_figure[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_font\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_font.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_font[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_form\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_form.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_form[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_paragraph\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_paragraph.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_paragraph[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_rectangle\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_rectangle.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_rectangle[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_xobject\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.pdf_xobject.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.pdf_xobject[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"unit\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.unit).map_err(|_|AccessError::WrongType)?;}
},
Self::Document(n)=>{
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"page\":").map_err(|_|AccessError::WrongType)?;{out.write_all(b"[").map_err(|_|AccessError::WrongType)?;for (i,e) in n.page.iter().enumerate(){if i>0{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}d.write_edge(e,&format!("{path}.page[{i}]"),out)?;}out.write_all(b"]").map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"total_pages\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.total_pages).map_err(|_|AccessError::WrongType)?;}
},
}out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover(&self,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{match self {
Self::BaseOperations(n)=>{
},
Self::Box(n)=>{
},
Self::GraphicState(n)=>{
d.instructions(&n.passthrough_per_char_instruction,format!("{path}.passthrough_per_char_instruction"))?;
},
Self::PdfAffineTransform(n)=>{
},
Self::PdfFontCharBoundingBox(n)=>{
},
Self::PdfInlineForm(n)=>{
},
Self::PdfMatrix(n)=>{
},
Self::PdfPath(n)=>{
},
Self::PdfXobjForm(n)=>{
},
Self::Cropbox(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
},
Self::Mediabox(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
},
Self::PageLayout(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
},
Self::PdfFigure(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
},
Self::PdfFont(n)=>{
for (i,e) in n.pdf_font_char_bounding_box.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_font_char_bounding_box[{i}]"))?;}
},
Self::PdfFormSubtype(n)=>{
if let Some(e)=&n.pdf_inline_form{d.discover_edge(e,&format!("{path}.pdf_inline_form"))?;}
if let Some(e)=&n.pdf_xobj_form{d.discover_edge(e,&format!("{path}.pdf_xobj_form"))?;}
},
Self::PdfOriginalPath(n)=>{
if let Some(e)=&n.pdf_path{d.discover_edge(e,&format!("{path}.pdf_path"))?;}
},
Self::PdfOriginalPathPrimitive(n)=>{
},
Self::PdfRectangle(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
if let Some(e)=&n.graphic_state{d.discover_edge(e,&format!("{path}.graphic_state"))?;}
},
Self::PdfStyle(n)=>{
if let Some(e)=&n.graphic_state{d.discover_edge(e,&format!("{path}.graphic_state"))?;}
},
Self::VisualBbox(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
},
Self::PdfCharacter(n)=>{
if let Some(e)=&n.pdf_style{d.discover_edge(e,&format!("{path}.pdf_style"))?;}
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
if let Some(e)=&n.visual_bbox{d.discover_edge(e,&format!("{path}.visual_bbox"))?;}
},
Self::PdfCurve(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
if let Some(e)=&n.graphic_state{d.discover_edge(e,&format!("{path}.graphic_state"))?;}
for (i,e) in n.pdf_path.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_path[{i}]"))?;}
for (i,e) in n.pdf_original_path.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_original_path[{i}]"))?;}
if let Some(e)=&n.pdf_original_path_primitive{d.discover_edge(e,&format!("{path}.pdf_original_path_primitive"))?;}
},
Self::PdfForm(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
if let Some(e)=&n.graphic_state{d.discover_edge(e,&format!("{path}.graphic_state"))?;}
if let Some(e)=&n.pdf_matrix{d.discover_edge(e,&format!("{path}.pdf_matrix"))?;}
if let Some(e)=&n.pdf_affine_transform{d.discover_edge(e,&format!("{path}.pdf_affine_transform"))?;}
if let Some(e)=&n.pdf_form_subtype{d.discover_edge(e,&format!("{path}.pdf_form_subtype"))?;}
},
Self::PdfSameStyleUnicodeCharacters(n)=>{
if let Some(e)=&n.pdf_style{d.discover_edge(e,&format!("{path}.pdf_style"))?;}
},
Self::PdfXobject(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
for (i,e) in n.pdf_font.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_font[{i}]"))?;}
if let Some(e)=&n.base_operations{d.discover_edge(e,&format!("{path}.base_operations"))?;}
},
Self::PdfFormula(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
for (i,e) in n.pdf_character.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_character[{i}]"))?;}
for (i,e) in n.pdf_curve.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_curve[{i}]"))?;}
for (i,e) in n.pdf_form.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_form[{i}]"))?;}
},
Self::PdfLine(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
for (i,e) in n.pdf_character.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_character[{i}]"))?;}
},
Self::PdfSameStyleCharacters(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
if let Some(e)=&n.pdf_style{d.discover_edge(e,&format!("{path}.pdf_style"))?;}
for (i,e) in n.pdf_character.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_character[{i}]"))?;}
},
Self::PdfParagraphComposition(n)=>{
if let Some(e)=&n.pdf_line{d.discover_edge(e,&format!("{path}.pdf_line"))?;}
if let Some(e)=&n.pdf_formula{d.discover_edge(e,&format!("{path}.pdf_formula"))?;}
if let Some(e)=&n.pdf_same_style_characters{d.discover_edge(e,&format!("{path}.pdf_same_style_characters"))?;}
if let Some(e)=&n.pdf_character{d.discover_edge(e,&format!("{path}.pdf_character"))?;}
if let Some(e)=&n.pdf_same_style_unicode_characters{d.discover_edge(e,&format!("{path}.pdf_same_style_unicode_characters"))?;}
},
Self::PdfParagraph(n)=>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
if let Some(e)=&n.pdf_style{d.discover_edge(e,&format!("{path}.pdf_style"))?;}
for (i,e) in n.pdf_paragraph_composition.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_paragraph_composition[{i}]"))?;}
},
Self::Page(n)=>{
if let Some(e)=&n.mediabox{d.discover_edge(e,&format!("{path}.mediabox"))?;}
if let Some(e)=&n.cropbox{d.discover_edge(e,&format!("{path}.cropbox"))?;}
for (i,e) in n.pdf_xobject.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_xobject[{i}]"))?;}
for (i,e) in n.page_layout.iter().enumerate(){d.discover_edge(e,&format!("{path}.page_layout[{i}]"))?;}
for (i,e) in n.pdf_rectangle.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_rectangle[{i}]"))?;}
for (i,e) in n.pdf_font.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_font[{i}]"))?;}
for (i,e) in n.pdf_paragraph.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_paragraph[{i}]"))?;}
for (i,e) in n.pdf_figure.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_figure[{i}]"))?;}
for (i,e) in n.pdf_character.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_character[{i}]"))?;}
for (i,e) in n.pdf_curve.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_curve[{i}]"))?;}
for (i,e) in n.pdf_form.iter().enumerate(){d.discover_edge(e,&format!("{path}.pdf_form[{i}]"))?;}
if let Some(e)=&n.base_operations{d.discover_edge(e,&format!("{path}.base_operations"))?;}
},
Self::Document(n)=>{
for (i,e) in n.page.iter().enumerate(){d.discover_edge(e,&format!("{path}.page[{i}]"))?;}
},
}Ok(())}
pub fn payload_utf8_bytes(&self)->usize {match self {
Self::BaseOperations(n)=>json_utf8_bytes(&serde_json::to_value(&n.value).unwrap_or(Json::Null)),
Self::Box(n)=>json_utf8_bytes(&serde_json::to_value(&n.x).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.y).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.x2).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.y2).unwrap_or(Json::Null)),
Self::GraphicState(n)=>instruction_utf8_bytes(&n.passthrough_per_char_instruction),
Self::PdfAffineTransform(n)=>json_utf8_bytes(&serde_json::to_value(&n.translation_x).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.translation_y).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.rotation).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.scale_x).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.scale_y).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.shear).unwrap_or(Json::Null)),
Self::PdfFontCharBoundingBox(n)=>json_utf8_bytes(&serde_json::to_value(&n.x).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.y).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.x2).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.y2).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.char_id).unwrap_or(Json::Null)),
Self::PdfInlineForm(n)=>json_utf8_bytes(&serde_json::to_value(&n.form_data).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.image_parameters).unwrap_or(Json::Null)),
Self::PdfMatrix(n)=>json_utf8_bytes(&serde_json::to_value(&n.a).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.b).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.c).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.d).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.e).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.f).unwrap_or(Json::Null)),
Self::PdfPath(n)=>json_utf8_bytes(&serde_json::to_value(&n.x()).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.y()).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.op).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.has_xy()).unwrap_or(Json::Null)),
Self::PdfXobjForm(n)=>json_utf8_bytes(&serde_json::to_value(&n.xref_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.do_args).unwrap_or(Json::Null)),
Self::Cropbox(n)=>0,
Self::Mediabox(n)=>0,
Self::PageLayout(n)=>json_utf8_bytes(&serde_json::to_value(&n.id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.conf).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.class_name).unwrap_or(Json::Null)),
Self::PdfFigure(n)=>0,
Self::PdfFont(n)=>json_utf8_bytes(&serde_json::to_value(&n.name).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.font_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.xref_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.encoding_length).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.font_subtype).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.type3_font_matrix).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.type3_font_bbox).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.type3_em_height).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.bold).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.italic).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.monospace).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.serif).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.ascent).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.descent).unwrap_or(Json::Null)),
Self::PdfFormSubtype(n)=>0,
Self::PdfOriginalPath(n)=>0,
Self::PdfOriginalPathPrimitive(n)=>json_utf8_bytes(&serde_json::to_value(&n.op).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.args).unwrap_or(Json::Null)),
Self::PdfRectangle(n)=>json_utf8_bytes(&serde_json::to_value(&n.debug_info).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.fill_background).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.xobj_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.line_width).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.render_order).unwrap_or(Json::Null)),
Self::PdfStyle(n)=>json_utf8_bytes(&serde_json::to_value(&n.font_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.font_size).unwrap_or(Json::Null)),
Self::VisualBbox(n)=>0,
Self::PdfCharacter(n)=>json_utf8_bytes(&serde_json::to_value(&n.vertical).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.scale).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.visual_roundtrip_text_scale_x).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.visual_roundtrip_text_scale_y).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.visual_roundtrip_text_render_mode).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.pdf_character_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.char_unicode).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.advance).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.xobj_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.debug_info).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.formula_layout_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.render_order).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.sub_render_order).unwrap_or(Json::Null)),
Self::PdfCurve(n)=>json_utf8_bytes(&serde_json::to_value(&n.debug_info).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.fill_background).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.stroke_path).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.evenodd).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.passthrough_paint).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.xobj_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.render_order).unwrap_or(Json::Null))+n.ctm.utf8_bytes()+n.relocation_transform.iter().map(PythonValue::utf8_bytes).sum::<usize>(),
Self::PdfForm(n)=>json_utf8_bytes(&serde_json::to_value(&n.xobj_id).unwrap_or(Json::Null))+n.ctm.iter().map(PythonValue::utf8_bytes).sum::<usize>()+n.relocation_transform.iter().map(PythonValue::utf8_bytes).sum::<usize>()+json_utf8_bytes(&serde_json::to_value(&n.render_order).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.form_type).unwrap_or(Json::Null)),
Self::PdfSameStyleUnicodeCharacters(n)=>json_utf8_bytes(&serde_json::to_value(&n.unicode).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.debug_info).unwrap_or(Json::Null)),
Self::PdfXobject(n)=>json_utf8_bytes(&serde_json::to_value(&n.xobj_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.xref_id).unwrap_or(Json::Null)),
Self::PdfFormula(n)=>json_utf8_bytes(&serde_json::to_value(&n.x_offset).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.y_offset).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.x_advance).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.line_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.is_corner_mark).unwrap_or(Json::Null)),
Self::PdfLine(n)=>json_utf8_bytes(&serde_json::to_value(&n.render_order).unwrap_or(Json::Null)),
Self::PdfSameStyleCharacters(n)=>0,
Self::PdfParagraphComposition(n)=>0,
Self::PdfParagraph(n)=>json_utf8_bytes(&serde_json::to_value(&n.xobj_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.unicode).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.scale).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.optimal_scale).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.vertical).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.first_line_indent).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.debug_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.layout_label).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.layout_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.render_order).unwrap_or(Json::Null)),
Self::Page(n)=>json_utf8_bytes(&serde_json::to_value(&n.page_number).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.unit).unwrap_or(Json::Null)),
Self::Document(n)=>json_utf8_bytes(&serde_json::to_value(&n.total_pages).unwrap_or(Json::Null)),
}}}
#[derive(Default)] struct TypedTables {
r#box:Vec<StoredNode<DataBox>>,
graphicstate:Vec<StoredNode<DataGraphicState>>,
pdffontcharboundingbox:Vec<StoredNode<DataPdfFontCharBoundingBox>>,
pagelayout:Vec<StoredNode<DataPageLayout>>,
pdffont:Vec<StoredNode<DataPdfFont>>,
pdfrectangle:Vec<StoredNode<DataPdfRectangle>>,
pdfstyle:Vec<StoredNode<DataPdfStyle>>,
visualbbox:Vec<StoredNode<DataVisualBbox>>,
pdfcharacter:Vec<StoredNode<DataPdfCharacter>>,
pdfcurve:Vec<StoredNode<DataPdfCurve>>,
pdfform:Vec<StoredNode<DataPdfForm>>,
pdfsamestyleunicodecharacters:Vec<StoredNode<DataPdfSameStyleUnicodeCharacters>>,
pdfxobject:Vec<StoredNode<DataPdfXobject>>,
pdfformula:Vec<StoredNode<DataPdfFormula>>,
pdfline:Vec<StoredNode<DataPdfLine>>,
pdfsamestylecharacters:Vec<StoredNode<DataPdfSameStyleCharacters>>,
pdfparagraphcomposition:Vec<StoredNode<DataPdfParagraphComposition>>,
pdfparagraph:Vec<StoredNode<DataPdfParagraph>>,
page:Vec<StoredNode<DataPage>>,
}
impl TypedTables {fn shrink_to_fit(&mut self){
self.r#box.shrink_to_fit();
self.graphicstate.shrink_to_fit();
self.pdffontcharboundingbox.shrink_to_fit();
self.pagelayout.shrink_to_fit();
self.pdffont.shrink_to_fit();
self.pdfrectangle.shrink_to_fit();
self.pdfstyle.shrink_to_fit();
self.visualbbox.shrink_to_fit();
self.pdfcharacter.shrink_to_fit();
self.pdfcurve.shrink_to_fit();
self.pdfform.shrink_to_fit();
self.pdfsamestyleunicodecharacters.shrink_to_fit();
self.pdfxobject.shrink_to_fit();
self.pdfformula.shrink_to_fit();
self.pdfline.shrink_to_fit();
self.pdfsamestylecharacters.shrink_to_fit();
self.pdfparagraphcomposition.shrink_to_fit();
self.pdfparagraph.shrink_to_fit();
self.page.shrink_to_fit();
} fn reserve_kind(&mut self,kind:Kind,count:usize){match kind{
Kind::Box=>self.r#box.reserve_exact(count),
Kind::GraphicState=>self.graphicstate.reserve_exact(count),
Kind::PdfFontCharBoundingBox=>self.pdffontcharboundingbox.reserve_exact(count),
Kind::PageLayout=>self.pagelayout.reserve_exact(count),
Kind::PdfFont=>self.pdffont.reserve_exact(count),
Kind::PdfRectangle=>self.pdfrectangle.reserve_exact(count),
Kind::PdfStyle=>self.pdfstyle.reserve_exact(count),
Kind::VisualBbox=>self.visualbbox.reserve_exact(count),
Kind::PdfCharacter=>self.pdfcharacter.reserve_exact(count),
Kind::PdfCurve=>self.pdfcurve.reserve_exact(count),
Kind::PdfForm=>self.pdfform.reserve_exact(count),
Kind::PdfSameStyleUnicodeCharacters=>self.pdfsamestyleunicodecharacters.reserve_exact(count),
Kind::PdfXobject=>self.pdfxobject.reserve_exact(count),
Kind::PdfFormula=>self.pdfformula.reserve_exact(count),
Kind::PdfLine=>self.pdfline.reserve_exact(count),
Kind::PdfSameStyleCharacters=>self.pdfsamestylecharacters.reserve_exact(count),
Kind::PdfParagraphComposition=>self.pdfparagraphcomposition.reserve_exact(count),
Kind::PdfParagraph=>self.pdfparagraph.reserve_exact(count),
Kind::Page=>self.page.reserve_exact(count),
_=>unreachable!(),}} fn push(&mut self,node:Node)->usize{let Node{identity,data}=node;match data{
NodeData::Box(data)=>{let i=self.r#box.len();self.r#box.push(StoredNode{identity,data});i},
NodeData::GraphicState(data)=>{let i=self.graphicstate.len();self.graphicstate.push(StoredNode{identity,data});i},
NodeData::PdfFontCharBoundingBox(data)=>{let i=self.pdffontcharboundingbox.len();self.pdffontcharboundingbox.push(StoredNode{identity,data});i},
NodeData::PageLayout(data)=>{let i=self.pagelayout.len();self.pagelayout.push(StoredNode{identity,data});i},
NodeData::PdfFont(data)=>{let i=self.pdffont.len();self.pdffont.push(StoredNode{identity,data});i},
NodeData::PdfRectangle(data)=>{let i=self.pdfrectangle.len();self.pdfrectangle.push(StoredNode{identity,data});i},
NodeData::PdfStyle(data)=>{let i=self.pdfstyle.len();self.pdfstyle.push(StoredNode{identity,data});i},
NodeData::VisualBbox(data)=>{let i=self.visualbbox.len();self.visualbbox.push(StoredNode{identity,data});i},
NodeData::PdfCharacter(data)=>{let i=self.pdfcharacter.len();self.pdfcharacter.push(StoredNode{identity,data});i},
NodeData::PdfCurve(data)=>{let i=self.pdfcurve.len();self.pdfcurve.push(StoredNode{identity,data});i},
NodeData::PdfForm(data)=>{let i=self.pdfform.len();self.pdfform.push(StoredNode{identity,data});i},
NodeData::PdfSameStyleUnicodeCharacters(data)=>{let i=self.pdfsamestyleunicodecharacters.len();self.pdfsamestyleunicodecharacters.push(StoredNode{identity,data});i},
NodeData::PdfXobject(data)=>{let i=self.pdfxobject.len();self.pdfxobject.push(StoredNode{identity,data});i},
NodeData::PdfFormula(data)=>{let i=self.pdfformula.len();self.pdfformula.push(StoredNode{identity,data});i},
NodeData::PdfLine(data)=>{let i=self.pdfline.len();self.pdfline.push(StoredNode{identity,data});i},
NodeData::PdfSameStyleCharacters(data)=>{let i=self.pdfsamestylecharacters.len();self.pdfsamestylecharacters.push(StoredNode{identity,data});i},
NodeData::PdfParagraphComposition(data)=>{let i=self.pdfparagraphcomposition.len();self.pdfparagraphcomposition.push(StoredNode{identity,data});i},
NodeData::PdfParagraph(data)=>{let i=self.pdfparagraph.len();self.pdfparagraph.push(StoredNode{identity,data});i},
NodeData::Page(data)=>{let i=self.page.len();self.page.push(StoredNode{identity,data});i},
_=>unreachable!("only mutable schema objects enter typed tables"),
}} fn node(&self,location:Location)->NodeRef<'_>{match location.kind{
Kind::Box=>{let n=&self.r#box[location.index];NodeRef{identity:n.identity,data:NodeDataRef::Box(&n.data)}},
Kind::GraphicState=>{let n=&self.graphicstate[location.index];NodeRef{identity:n.identity,data:NodeDataRef::GraphicState(&n.data)}},
Kind::PdfFontCharBoundingBox=>{let n=&self.pdffontcharboundingbox[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfFontCharBoundingBox(&n.data)}},
Kind::PageLayout=>{let n=&self.pagelayout[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PageLayout(&n.data)}},
Kind::PdfFont=>{let n=&self.pdffont[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfFont(&n.data)}},
Kind::PdfRectangle=>{let n=&self.pdfrectangle[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfRectangle(&n.data)}},
Kind::PdfStyle=>{let n=&self.pdfstyle[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfStyle(&n.data)}},
Kind::VisualBbox=>{let n=&self.visualbbox[location.index];NodeRef{identity:n.identity,data:NodeDataRef::VisualBbox(&n.data)}},
Kind::PdfCharacter=>{let n=&self.pdfcharacter[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfCharacter(&n.data)}},
Kind::PdfCurve=>{let n=&self.pdfcurve[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfCurve(&n.data)}},
Kind::PdfForm=>{let n=&self.pdfform[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfForm(&n.data)}},
Kind::PdfSameStyleUnicodeCharacters=>{let n=&self.pdfsamestyleunicodecharacters[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfSameStyleUnicodeCharacters(&n.data)}},
Kind::PdfXobject=>{let n=&self.pdfxobject[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfXobject(&n.data)}},
Kind::PdfFormula=>{let n=&self.pdfformula[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfFormula(&n.data)}},
Kind::PdfLine=>{let n=&self.pdfline[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfLine(&n.data)}},
Kind::PdfSameStyleCharacters=>{let n=&self.pdfsamestylecharacters[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfSameStyleCharacters(&n.data)}},
Kind::PdfParagraphComposition=>{let n=&self.pdfparagraphcomposition[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfParagraphComposition(&n.data)}},
Kind::PdfParagraph=>{let n=&self.pdfparagraph[location.index];NodeRef{identity:n.identity,data:NodeDataRef::PdfParagraph(&n.data)}},
Kind::Page=>{let n=&self.page[location.index];NodeRef{identity:n.identity,data:NodeDataRef::Page(&n.data)}},
_=>unreachable!("only table kinds have locations"),
}} fn drain_kind(&mut self,kind:Kind,keys:&[usize],visit:&mut impl FnMut(usize,Node)){match kind{
Kind::Box=>{for (node,key) in std::mem::take(&mut self.r#box).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::Box(node.data)});}},
Kind::GraphicState=>{for (node,key) in std::mem::take(&mut self.graphicstate).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::GraphicState(node.data)});}},
Kind::PdfFontCharBoundingBox=>{for (node,key) in std::mem::take(&mut self.pdffontcharboundingbox).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfFontCharBoundingBox(node.data)});}},
Kind::PageLayout=>{for (node,key) in std::mem::take(&mut self.pagelayout).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PageLayout(node.data)});}},
Kind::PdfFont=>{for (node,key) in std::mem::take(&mut self.pdffont).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfFont(node.data)});}},
Kind::PdfRectangle=>{for (node,key) in std::mem::take(&mut self.pdfrectangle).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfRectangle(node.data)});}},
Kind::PdfStyle=>{for (node,key) in std::mem::take(&mut self.pdfstyle).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfStyle(node.data)});}},
Kind::VisualBbox=>{for (node,key) in std::mem::take(&mut self.visualbbox).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::VisualBbox(node.data)});}},
Kind::PdfCharacter=>{for (node,key) in std::mem::take(&mut self.pdfcharacter).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfCharacter(node.data)});}},
Kind::PdfCurve=>{for (node,key) in std::mem::take(&mut self.pdfcurve).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfCurve(node.data)});}},
Kind::PdfForm=>{for (node,key) in std::mem::take(&mut self.pdfform).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfForm(node.data)});}},
Kind::PdfSameStyleUnicodeCharacters=>{for (node,key) in std::mem::take(&mut self.pdfsamestyleunicodecharacters).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfSameStyleUnicodeCharacters(node.data)});}},
Kind::PdfXobject=>{for (node,key) in std::mem::take(&mut self.pdfxobject).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfXobject(node.data)});}},
Kind::PdfFormula=>{for (node,key) in std::mem::take(&mut self.pdfformula).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfFormula(node.data)});}},
Kind::PdfLine=>{for (node,key) in std::mem::take(&mut self.pdfline).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfLine(node.data)});}},
Kind::PdfSameStyleCharacters=>{for (node,key) in std::mem::take(&mut self.pdfsamestylecharacters).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfSameStyleCharacters(node.data)});}},
Kind::PdfParagraphComposition=>{for (node,key) in std::mem::take(&mut self.pdfparagraphcomposition).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfParagraphComposition(node.data)});}},
Kind::PdfParagraph=>{for (node,key) in std::mem::take(&mut self.pdfparagraph).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::PdfParagraph(node.data)});}},
Kind::Page=>{for (node,key) in std::mem::take(&mut self.page).into_iter().zip(keys){visit(*key,Node{identity:node.identity,data:NodeData::Page(node.data)});}},
_=>unreachable!(),}
} fn reuse_kind(&mut self,source:&mut Self,kind:Kind,keys:&[usize],visit:&mut impl FnMut(usize,usize,&mut Node)){match kind{
Kind::Box=>{let mut values=std::mem::take(&mut source.r#box);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::Box(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::Box(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.r#box=values;},
Kind::GraphicState=>{let mut values=std::mem::take(&mut source.graphicstate);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::GraphicState(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::GraphicState(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.graphicstate=values;},
Kind::PdfFontCharBoundingBox=>{let mut values=std::mem::take(&mut source.pdffontcharboundingbox);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfFontCharBoundingBox(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfFontCharBoundingBox(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdffontcharboundingbox=values;},
Kind::PageLayout=>{let mut values=std::mem::take(&mut source.pagelayout);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PageLayout(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PageLayout(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pagelayout=values;},
Kind::PdfFont=>{let mut values=std::mem::take(&mut source.pdffont);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfFont(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfFont(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdffont=values;},
Kind::PdfRectangle=>{let mut values=std::mem::take(&mut source.pdfrectangle);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfRectangle(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfRectangle(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfrectangle=values;},
Kind::PdfStyle=>{let mut values=std::mem::take(&mut source.pdfstyle);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfStyle(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfStyle(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfstyle=values;},
Kind::VisualBbox=>{let mut values=std::mem::take(&mut source.visualbbox);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::VisualBbox(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::VisualBbox(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.visualbbox=values;},
Kind::PdfCharacter=>{let mut values=std::mem::take(&mut source.pdfcharacter);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfCharacter(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfCharacter(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfcharacter=values;},
Kind::PdfCurve=>{let mut values=std::mem::take(&mut source.pdfcurve);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfCurve(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfCurve(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfcurve=values;},
Kind::PdfForm=>{let mut values=std::mem::take(&mut source.pdfform);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfForm(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfForm(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfform=values;},
Kind::PdfSameStyleUnicodeCharacters=>{let mut values=std::mem::take(&mut source.pdfsamestyleunicodecharacters);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfSameStyleUnicodeCharacters(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfSameStyleUnicodeCharacters(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfsamestyleunicodecharacters=values;},
Kind::PdfXobject=>{let mut values=std::mem::take(&mut source.pdfxobject);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfXobject(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfXobject(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfxobject=values;},
Kind::PdfFormula=>{let mut values=std::mem::take(&mut source.pdfformula);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfFormula(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfFormula(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfformula=values;},
Kind::PdfLine=>{let mut values=std::mem::take(&mut source.pdfline);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfLine(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfLine(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfline=values;},
Kind::PdfSameStyleCharacters=>{let mut values=std::mem::take(&mut source.pdfsamestylecharacters);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfSameStyleCharacters(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfSameStyleCharacters(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfsamestylecharacters=values;},
Kind::PdfParagraphComposition=>{let mut values=std::mem::take(&mut source.pdfparagraphcomposition);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfParagraphComposition(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfParagraphComposition(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfparagraphcomposition=values;},
Kind::PdfParagraph=>{let mut values=std::mem::take(&mut source.pdfparagraph);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::PdfParagraph(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::PdfParagraph(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.pdfparagraph=values;},
Kind::Page=>{let mut values=std::mem::take(&mut source.page);for (index,(stored,key)) in values.iter_mut().zip(keys).enumerate(){let mut node=Node{identity:stored.identity,data:NodeData::Page(std::mem::take(&mut stored.data))};visit(*key,index,&mut node);let NodeData::Page(data)=node.data else{unreachable!()};stored.data=data;}values.shrink_to_fit();self.page=values;},
_=>unreachable!(),}
} #[cfg(feature="memory-profile")] fn capacity_bytes(&self)->usize{
self.r#box.capacity()*std::mem::size_of::<StoredNode<DataBox>>()+self.graphicstate.capacity()*std::mem::size_of::<StoredNode<DataGraphicState>>()+self.pdffontcharboundingbox.capacity()*std::mem::size_of::<StoredNode<DataPdfFontCharBoundingBox>>()+self.pagelayout.capacity()*std::mem::size_of::<StoredNode<DataPageLayout>>()+self.pdffont.capacity()*std::mem::size_of::<StoredNode<DataPdfFont>>()+self.pdfrectangle.capacity()*std::mem::size_of::<StoredNode<DataPdfRectangle>>()+self.pdfstyle.capacity()*std::mem::size_of::<StoredNode<DataPdfStyle>>()+self.visualbbox.capacity()*std::mem::size_of::<StoredNode<DataVisualBbox>>()+self.pdfcharacter.capacity()*std::mem::size_of::<StoredNode<DataPdfCharacter>>()+self.pdfcurve.capacity()*std::mem::size_of::<StoredNode<DataPdfCurve>>()+self.pdfform.capacity()*std::mem::size_of::<StoredNode<DataPdfForm>>()+self.pdfsamestyleunicodecharacters.capacity()*std::mem::size_of::<StoredNode<DataPdfSameStyleUnicodeCharacters>>()+self.pdfxobject.capacity()*std::mem::size_of::<StoredNode<DataPdfXobject>>()+self.pdfformula.capacity()*std::mem::size_of::<StoredNode<DataPdfFormula>>()+self.pdfline.capacity()*std::mem::size_of::<StoredNode<DataPdfLine>>()+self.pdfsamestylecharacters.capacity()*std::mem::size_of::<StoredNode<DataPdfSameStyleCharacters>>()+self.pdfparagraphcomposition.capacity()*std::mem::size_of::<StoredNode<DataPdfParagraphComposition>>()+self.pdfparagraph.capacity()*std::mem::size_of::<StoredNode<DataPdfParagraph>>()+self.page.capacity()*std::mem::size_of::<StoredNode<DataPage>>()
}}
impl Edge {fn table_edges(&self,visit:&mut impl FnMut(usize)){match self{
Self::Shared(key)=>visit(*key),
Self::Inline(n)=>{for e in n.data.edges(){e.table_edges(visit);}},
Self::ImmutableCropbox(n)=>{
if let Some(e)=&n.data.r#box{e.table_edges(visit);}
},
Self::ImmutableMediabox(n)=>{
if let Some(e)=&n.data.r#box{e.table_edges(visit);}
},
Self::ImmutablePdfFigure(n)=>{
if let Some(e)=&n.data.r#box{e.table_edges(visit);}
},
_=>{}}} fn relocate(&mut self,indices:&Relocation,memo:&mut HashMap<u64,Edge>){match self{
Self::Shared(key)=>{*key=indices.index(*key);debug_assert_ne!(*key,usize::MAX);},
Self::Inline(_)=>unreachable!("compaction follows attachment"),
Self::ImmutableCropbox(n)=>{let identity=n.identity;if let Some(edge)=memo.get(&identity){*self=edge.share();return;}
let mut data=NodeData::Cropbox(DataCropbox{
r#box:n.data.r#box.as_ref().map(Edge::share),
});for e in data.edges_mut(){e.relocate(indices,memo);}
let edge=Node{identity,data}.into_immutable_edge();memo.insert(identity,edge.share());*self=edge;},
Self::ImmutableMediabox(n)=>{let identity=n.identity;if let Some(edge)=memo.get(&identity){*self=edge.share();return;}
let mut data=NodeData::Mediabox(DataMediabox{
r#box:n.data.r#box.as_ref().map(Edge::share),
});for e in data.edges_mut(){e.relocate(indices,memo);}
let edge=Node{identity,data}.into_immutable_edge();memo.insert(identity,edge.share());*self=edge;},
Self::ImmutablePdfFigure(n)=>{let identity=n.identity;if let Some(edge)=memo.get(&identity){*self=edge.share();return;}
let mut data=NodeData::PdfFigure(DataPdfFigure{
r#box:n.data.r#box.as_ref().map(Edge::share),
});for e in data.edges_mut(){e.relocate(indices,memo);}
let edge=Node{identity,data}.into_immutable_edge();memo.insert(identity,edge.share());*self=edge;},
_=>{}}}}
#[allow(unused_variables)] fn dump_data_baseoperations(n:&DataBaseOperations,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("value".into(),serde_json::to_value(&n.value).map_err(|_|AccessError::WrongType)?);
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_baseoperations<W:std::io::Write>(n:&DataBaseOperations,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"value\":").map_err(|_|AccessError::WrongType)?;{let value=decode_base_operations(&n.value)?;serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_baseoperations(n:&DataBaseOperations,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
Ok(())}
#[allow(unused_variables)] fn payload_data_baseoperations(n:&DataBaseOperations,seen:&mut HashSet<u64>)->usize{json_utf8_bytes(&serde_json::to_value(&n.value).unwrap_or(Json::Null))}
#[allow(unused_variables)] fn dump_data_pdfaffinetransform(n:&DataPdfAffineTransform,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("translation_x".into(),n.translation_x.map(float_artifact).unwrap_or(Json::Null));
out.insert("translation_y".into(),n.translation_y.map(float_artifact).unwrap_or(Json::Null));
out.insert("rotation".into(),n.rotation.map(float_artifact).unwrap_or(Json::Null));
out.insert("scale_x".into(),n.scale_x.map(float_artifact).unwrap_or(Json::Null));
out.insert("scale_y".into(),n.scale_y.map(float_artifact).unwrap_or(Json::Null));
out.insert("shear".into(),n.shear.map(float_artifact).unwrap_or(Json::Null));
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdfaffinetransform<W:std::io::Write>(n:&DataPdfAffineTransform,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"rotation\":").map_err(|_|AccessError::WrongType)?;{let value=n.rotation.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"scale_x\":").map_err(|_|AccessError::WrongType)?;{let value=n.scale_x.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"scale_y\":").map_err(|_|AccessError::WrongType)?;{let value=n.scale_y.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"shear\":").map_err(|_|AccessError::WrongType)?;{let value=n.shear.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"translation_x\":").map_err(|_|AccessError::WrongType)?;{let value=n.translation_x.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"translation_y\":").map_err(|_|AccessError::WrongType)?;{let value=n.translation_y.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdfaffinetransform(n:&DataPdfAffineTransform,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
Ok(())}
#[allow(unused_variables)] fn payload_data_pdfaffinetransform(n:&DataPdfAffineTransform,seen:&mut HashSet<u64>)->usize{json_utf8_bytes(&serde_json::to_value(&n.translation_x).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.translation_y).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.rotation).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.scale_x).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.scale_y).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.shear).unwrap_or(Json::Null))}
#[allow(unused_variables)] fn dump_data_pdfinlineform(n:&DataPdfInlineForm,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("form_data".into(),serde_json::to_value(&n.form_data).map_err(|_|AccessError::WrongType)?);
out.insert("image_parameters".into(),serde_json::to_value(&n.image_parameters).map_err(|_|AccessError::WrongType)?);
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdfinlineform<W:std::io::Write>(n:&DataPdfInlineForm,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"form_data\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.form_data).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"image_parameters\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.image_parameters).map_err(|_|AccessError::WrongType)?;}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdfinlineform(n:&DataPdfInlineForm,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
Ok(())}
#[allow(unused_variables)] fn payload_data_pdfinlineform(n:&DataPdfInlineForm,seen:&mut HashSet<u64>)->usize{json_utf8_bytes(&serde_json::to_value(&n.form_data).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.image_parameters).unwrap_or(Json::Null))}
#[allow(unused_variables)] fn dump_data_pdfmatrix(n:&DataPdfMatrix,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("a".into(),n.a.map(float_artifact).unwrap_or(Json::Null));
out.insert("b".into(),n.b.map(float_artifact).unwrap_or(Json::Null));
out.insert("c".into(),n.c.map(float_artifact).unwrap_or(Json::Null));
out.insert("d".into(),n.d.map(float_artifact).unwrap_or(Json::Null));
out.insert("e".into(),n.e.map(float_artifact).unwrap_or(Json::Null));
out.insert("f".into(),n.f.map(float_artifact).unwrap_or(Json::Null));
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdfmatrix<W:std::io::Write>(n:&DataPdfMatrix,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"a\":").map_err(|_|AccessError::WrongType)?;{let value=n.a.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"b\":").map_err(|_|AccessError::WrongType)?;{let value=n.b.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"c\":").map_err(|_|AccessError::WrongType)?;{let value=n.c.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"d\":").map_err(|_|AccessError::WrongType)?;{let value=n.d.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"e\":").map_err(|_|AccessError::WrongType)?;{let value=n.e.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"f\":").map_err(|_|AccessError::WrongType)?;{let value=n.f.map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdfmatrix(n:&DataPdfMatrix,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
Ok(())}
#[allow(unused_variables)] fn payload_data_pdfmatrix(n:&DataPdfMatrix,seen:&mut HashSet<u64>)->usize{json_utf8_bytes(&serde_json::to_value(&n.a).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.b).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.c).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.d).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.e).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.f).unwrap_or(Json::Null))}
#[allow(unused_variables)] fn dump_data_pdfpath(n:&DataPdfPath,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("x".into(),n.x().map(float_artifact).unwrap_or(Json::Null));
out.insert("y".into(),n.y().map(float_artifact).unwrap_or(Json::Null));
out.insert("op".into(),serde_json::to_value(&n.op).map_err(|_|AccessError::WrongType)?);
out.insert("has_xy".into(),serde_json::to_value(&n.has_xy()).map_err(|_|AccessError::WrongType)?);
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdfpath<W:std::io::Write>(n:&DataPdfPath,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"has_xy\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.has_xy()).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"op\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.op).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"x\":").map_err(|_|AccessError::WrongType)?;{let value=n.x().map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"y\":").map_err(|_|AccessError::WrongType)?;{let value=n.y().map(float_artifact).unwrap_or(Json::Null);serde_json::to_writer(&mut *out,&value).map_err(|_|AccessError::WrongType)?;}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdfpath(n:&DataPdfPath,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
Ok(())}
#[allow(unused_variables)] fn payload_data_pdfpath(n:&DataPdfPath,seen:&mut HashSet<u64>)->usize{json_utf8_bytes(&serde_json::to_value(&n.x()).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.y()).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.op).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.has_xy()).unwrap_or(Json::Null))}
#[allow(unused_variables)] fn dump_data_pdfxobjform(n:&DataPdfXobjForm,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("xref_id".into(),serde_json::to_value(&n.xref_id).map_err(|_|AccessError::WrongType)?);
out.insert("do_args".into(),serde_json::to_value(&n.do_args).map_err(|_|AccessError::WrongType)?);
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdfxobjform<W:std::io::Write>(n:&DataPdfXobjForm,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"do_args\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.do_args).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"xref_id\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.xref_id).map_err(|_|AccessError::WrongType)?;}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdfxobjform(n:&DataPdfXobjForm,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
Ok(())}
#[allow(unused_variables)] fn payload_data_pdfxobjform(n:&DataPdfXobjForm,seen:&mut HashSet<u64>)->usize{json_utf8_bytes(&serde_json::to_value(&n.xref_id).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.do_args).unwrap_or(Json::Null))}
#[allow(unused_variables)] fn dump_data_cropbox(n:&DataCropbox,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_cropbox<W:std::io::Write>(n:&DataCropbox,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_cropbox(n:&DataCropbox,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
Ok(())}
#[allow(unused_variables)] fn payload_data_cropbox(n:&DataCropbox,seen:&mut HashSet<u64>)->usize{n.r#box.iter().map(|edge|edge.payload_utf8_bytes(seen)).sum::<usize>()}
#[allow(unused_variables)] fn dump_data_mediabox(n:&DataMediabox,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_mediabox<W:std::io::Write>(n:&DataMediabox,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_mediabox(n:&DataMediabox,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
Ok(())}
#[allow(unused_variables)] fn payload_data_mediabox(n:&DataMediabox,seen:&mut HashSet<u64>)->usize{n.r#box.iter().map(|edge|edge.payload_utf8_bytes(seen)).sum::<usize>()}
#[allow(unused_variables)] fn dump_data_pdffigure(n:&DataPdfFigure,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("box".into(),n.r#box.as_ref().map(|c|d.edge(c,format!("{path}.box"))).transpose()?.unwrap_or(Json::Null));
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdffigure<W:std::io::Write>(n:&DataPdfFigure,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"box\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.r#box{d.write_edge(e,&format!("{path}.box"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdffigure(n:&DataPdfFigure,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
if let Some(e)=&n.r#box{d.discover_edge(e,&format!("{path}.box"))?;}
Ok(())}
#[allow(unused_variables)] fn payload_data_pdffigure(n:&DataPdfFigure,seen:&mut HashSet<u64>)->usize{n.r#box.iter().map(|edge|edge.payload_utf8_bytes(seen)).sum::<usize>()}
#[allow(unused_variables)] fn dump_data_pdfformsubtype(n:&DataPdfFormSubtype,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("pdf_inline_form".into(),n.pdf_inline_form.as_ref().map(|c|d.edge(c,format!("{path}.pdf_inline_form"))).transpose()?.unwrap_or(Json::Null));
out.insert("pdf_xobj_form".into(),n.pdf_xobj_form.as_ref().map(|c|d.edge(c,format!("{path}.pdf_xobj_form"))).transpose()?.unwrap_or(Json::Null));
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdfformsubtype<W:std::io::Write>(n:&DataPdfFormSubtype,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_inline_form\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_inline_form{d.write_edge(e,&format!("{path}.pdf_inline_form"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_xobj_form\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_xobj_form{d.write_edge(e,&format!("{path}.pdf_xobj_form"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdfformsubtype(n:&DataPdfFormSubtype,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
if let Some(e)=&n.pdf_inline_form{d.discover_edge(e,&format!("{path}.pdf_inline_form"))?;}
if let Some(e)=&n.pdf_xobj_form{d.discover_edge(e,&format!("{path}.pdf_xobj_form"))?;}
Ok(())}
#[allow(unused_variables)] fn payload_data_pdfformsubtype(n:&DataPdfFormSubtype,seen:&mut HashSet<u64>)->usize{n.pdf_inline_form.iter().map(|edge|edge.payload_utf8_bytes(seen)).sum::<usize>()+n.pdf_xobj_form.iter().map(|edge|edge.payload_utf8_bytes(seen)).sum::<usize>()}
#[allow(unused_variables)] fn dump_data_pdforiginalpath(n:&DataPdfOriginalPath,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("pdf_path".into(),n.pdf_path.as_ref().map(|c|d.edge(c,format!("{path}.pdf_path"))).transpose()?.unwrap_or(Json::Null));
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdforiginalpath<W:std::io::Write>(n:&DataPdfOriginalPath,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"pdf_path\":").map_err(|_|AccessError::WrongType)?;{if let Some(e)=&n.pdf_path{d.write_edge(e,&format!("{path}.pdf_path"),out)?;}else{out.write_all(b"null").map_err(|_|AccessError::WrongType)?;}}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdforiginalpath(n:&DataPdfOriginalPath,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
if let Some(e)=&n.pdf_path{d.discover_edge(e,&format!("{path}.pdf_path"))?;}
Ok(())}
#[allow(unused_variables)] fn payload_data_pdforiginalpath(n:&DataPdfOriginalPath,seen:&mut HashSet<u64>)->usize{n.pdf_path.iter().map(|edge|edge.payload_utf8_bytes(seen)).sum::<usize>()}
#[allow(unused_variables)] fn dump_data_pdforiginalpathprimitive(n:&DataPdfOriginalPathPrimitive,d:&mut DumpContext<'_>,path:&str)->Result<Json,AccessError>{let mut out=serde_json::Map::new();
out.insert("op".into(),serde_json::to_value(&n.op).map_err(|_|AccessError::WrongType)?);
out.insert("args".into(),serde_json::to_value(&n.args).map_err(|_|AccessError::WrongType)?);
Ok(Json::Object(out))}
#[allow(unused_braces,unused_assignments,unused_variables)] fn write_data_pdforiginalpathprimitive<W:std::io::Write>(n:&DataPdfOriginalPathPrimitive,d:&mut DumpContext<'_>,path:&str,out:&mut W)->Result<(),AccessError>{out.write_all(b"{").map_err(|_|AccessError::WrongType)?;let mut first=true;
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"args\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.args).map_err(|_|AccessError::WrongType)?;}
if !first{out.write_all(b",").map_err(|_|AccessError::WrongType)?;}first=false;out.write_all(b"\"op\":").map_err(|_|AccessError::WrongType)?;{serde_json::to_writer(&mut *out,&n.op).map_err(|_|AccessError::WrongType)?;}
out.write_all(b"}").map_err(|_|AccessError::WrongType)?;Ok(())}
#[allow(unused_variables)] fn discover_data_pdforiginalpathprimitive(n:&DataPdfOriginalPathPrimitive,d:&mut DumpContext<'_>,path:&str)->Result<(),AccessError>{
Ok(())}
#[allow(unused_variables)] fn payload_data_pdforiginalpathprimitive(n:&DataPdfOriginalPathPrimitive,seen:&mut HashSet<u64>)->usize{json_utf8_bytes(&serde_json::to_value(&n.op).unwrap_or(Json::Null))+json_utf8_bytes(&serde_json::to_value(&n.args).unwrap_or(Json::Null))}
impl Edge {pub fn payload_utf8_bytes(&self,seen:&mut HashSet<u64>)->usize{match self{
Self::Inline(_)|Self::Shared(_)=>0,
Self::ImmutableBaseOperations(node)=>if !seen.insert(node.identity){0}else{payload_data_baseoperations(&node.data,seen)},
Self::ImmutablePdfAffineTransform(node)=>if !seen.insert(node.identity){0}else{payload_data_pdfaffinetransform(&node.data,seen)},
Self::ImmutablePdfInlineForm(node)=>if !seen.insert(node.identity){0}else{payload_data_pdfinlineform(&node.data,seen)},
Self::ImmutablePdfMatrix(node)=>if !seen.insert(node.identity){0}else{payload_data_pdfmatrix(&node.data,seen)},
Self::ImmutablePdfPath(node)=>if !seen.insert(node.identity){0}else{payload_data_pdfpath(&node.data,seen)},
Self::ImmutablePdfXobjForm(node)=>if !seen.insert(node.identity){0}else{payload_data_pdfxobjform(&node.data,seen)},
Self::ImmutableCropbox(node)=>if !seen.insert(node.identity){0}else{payload_data_cropbox(&node.data,seen)},
Self::ImmutableMediabox(node)=>if !seen.insert(node.identity){0}else{payload_data_mediabox(&node.data,seen)},
Self::ImmutablePdfFigure(node)=>if !seen.insert(node.identity){0}else{payload_data_pdffigure(&node.data,seen)},
Self::ImmutablePdfFormSubtype(node)=>if !seen.insert(node.identity){0}else{payload_data_pdfformsubtype(&node.data,seen)},
Self::ImmutablePdfOriginalPath(node)=>if !seen.insert(node.identity){0}else{payload_data_pdforiginalpath(&node.data,seen)},
Self::ImmutablePdfOriginalPathPrimitive(node)=>if !seen.insert(node.identity){0}else{payload_data_pdforiginalpathprimitive(&node.data,seen)},
_=>unreachable!("table types are always shared"),
}}}
fn share_edge(edge:&Edge)->Edge{match edge{
Edge::Inline(_)=>unreachable!("only attached values may be shared"),
Edge::Shared(index)=>Edge::Shared(*index),
Edge::ImmutableBaseOperations(node)=>Edge::ImmutableBaseOperations(Arc::clone(node)),
Edge::ImmutableBox(node)=>Edge::ImmutableBox(Arc::clone(node)),
Edge::ImmutableGraphicState(node)=>Edge::ImmutableGraphicState(Arc::clone(node)),
Edge::ImmutablePdfAffineTransform(node)=>Edge::ImmutablePdfAffineTransform(Arc::clone(node)),
Edge::ImmutablePdfFontCharBoundingBox(node)=>Edge::ImmutablePdfFontCharBoundingBox(Arc::clone(node)),
Edge::ImmutablePdfInlineForm(node)=>Edge::ImmutablePdfInlineForm(Arc::clone(node)),
Edge::ImmutablePdfMatrix(node)=>Edge::ImmutablePdfMatrix(Arc::clone(node)),
Edge::ImmutablePdfPath(node)=>Edge::ImmutablePdfPath(Arc::clone(node)),
Edge::ImmutablePdfXobjForm(node)=>Edge::ImmutablePdfXobjForm(Arc::clone(node)),
Edge::ImmutableCropbox(node)=>Edge::ImmutableCropbox(Arc::clone(node)),
Edge::ImmutableMediabox(node)=>Edge::ImmutableMediabox(Arc::clone(node)),
Edge::ImmutablePageLayout(node)=>Edge::ImmutablePageLayout(Arc::clone(node)),
Edge::ImmutablePdfFigure(node)=>Edge::ImmutablePdfFigure(Arc::clone(node)),
Edge::ImmutablePdfFont(node)=>Edge::ImmutablePdfFont(Arc::clone(node)),
Edge::ImmutablePdfFormSubtype(node)=>Edge::ImmutablePdfFormSubtype(Arc::clone(node)),
Edge::ImmutablePdfOriginalPath(node)=>Edge::ImmutablePdfOriginalPath(Arc::clone(node)),
Edge::ImmutablePdfOriginalPathPrimitive(node)=>Edge::ImmutablePdfOriginalPathPrimitive(Arc::clone(node)),
Edge::ImmutablePdfRectangle(node)=>Edge::ImmutablePdfRectangle(Arc::clone(node)),
Edge::ImmutablePdfStyle(node)=>Edge::ImmutablePdfStyle(Arc::clone(node)),
Edge::ImmutableVisualBbox(node)=>Edge::ImmutableVisualBbox(Arc::clone(node)),
Edge::ImmutablePdfCharacter(node)=>Edge::ImmutablePdfCharacter(Arc::clone(node)),
Edge::ImmutablePdfCurve(node)=>Edge::ImmutablePdfCurve(Arc::clone(node)),
Edge::ImmutablePdfForm(node)=>Edge::ImmutablePdfForm(Arc::clone(node)),
Edge::ImmutablePdfSameStyleUnicodeCharacters(node)=>Edge::ImmutablePdfSameStyleUnicodeCharacters(Arc::clone(node)),
Edge::ImmutablePdfXobject(node)=>Edge::ImmutablePdfXobject(Arc::clone(node)),
Edge::ImmutablePdfFormula(node)=>Edge::ImmutablePdfFormula(Arc::clone(node)),
Edge::ImmutablePdfLine(node)=>Edge::ImmutablePdfLine(Arc::clone(node)),
Edge::ImmutablePdfSameStyleCharacters(node)=>Edge::ImmutablePdfSameStyleCharacters(Arc::clone(node)),
Edge::ImmutablePdfParagraphComposition(node)=>Edge::ImmutablePdfParagraphComposition(Arc::clone(node)),
Edge::ImmutablePdfParagraph(node)=>Edge::ImmutablePdfParagraph(Arc::clone(node)),
Edge::ImmutablePage(node)=>Edge::ImmutablePage(Arc::clone(node)),
Edge::ImmutableDocument(node)=>Edge::ImmutableDocument(Arc::clone(node)),
}}
fn dump_immutable_edge(d:&mut DumpContext<'_>,edge:&Edge,path:&str)->Result<Json,AccessError>{match edge{
Edge::ImmutableBaseOperations(node)=>{d.alias(Kind::BaseOperations,node.identity,path);dump_data_baseoperations(&node.data,d,path)}
Edge::ImmutablePdfAffineTransform(node)=>{d.alias(Kind::PdfAffineTransform,node.identity,path);dump_data_pdfaffinetransform(&node.data,d,path)}
Edge::ImmutablePdfInlineForm(node)=>{d.alias(Kind::PdfInlineForm,node.identity,path);dump_data_pdfinlineform(&node.data,d,path)}
Edge::ImmutablePdfMatrix(node)=>{d.alias(Kind::PdfMatrix,node.identity,path);dump_data_pdfmatrix(&node.data,d,path)}
Edge::ImmutablePdfPath(node)=>{d.alias(Kind::PdfPath,node.identity,path);dump_data_pdfpath(&node.data,d,path)}
Edge::ImmutablePdfXobjForm(node)=>{d.alias(Kind::PdfXobjForm,node.identity,path);dump_data_pdfxobjform(&node.data,d,path)}
Edge::ImmutableCropbox(node)=>{d.alias(Kind::Cropbox,node.identity,path);dump_data_cropbox(&node.data,d,path)}
Edge::ImmutableMediabox(node)=>{d.alias(Kind::Mediabox,node.identity,path);dump_data_mediabox(&node.data,d,path)}
Edge::ImmutablePdfFigure(node)=>{d.alias(Kind::PdfFigure,node.identity,path);dump_data_pdffigure(&node.data,d,path)}
Edge::ImmutablePdfFormSubtype(node)=>{d.alias(Kind::PdfFormSubtype,node.identity,path);dump_data_pdfformsubtype(&node.data,d,path)}
Edge::ImmutablePdfOriginalPath(node)=>{d.alias(Kind::PdfOriginalPath,node.identity,path);dump_data_pdforiginalpath(&node.data,d,path)}
Edge::ImmutablePdfOriginalPathPrimitive(node)=>{d.alias(Kind::PdfOriginalPathPrimitive,node.identity,path);dump_data_pdforiginalpathprimitive(&node.data,d,path)}
_=>unreachable!("table types are always shared"),
}}
fn write_immutable_edge<W:std::io::Write>(d:&mut DumpContext<'_>,edge:&Edge,path:&str,out:&mut W)->Result<(),AccessError>{match edge{
Edge::ImmutableBaseOperations(node)=>{d.alias(Kind::BaseOperations,node.identity,path);write_data_baseoperations(&node.data,d,path,out)}
Edge::ImmutablePdfAffineTransform(node)=>{d.alias(Kind::PdfAffineTransform,node.identity,path);write_data_pdfaffinetransform(&node.data,d,path,out)}
Edge::ImmutablePdfInlineForm(node)=>{d.alias(Kind::PdfInlineForm,node.identity,path);write_data_pdfinlineform(&node.data,d,path,out)}
Edge::ImmutablePdfMatrix(node)=>{d.alias(Kind::PdfMatrix,node.identity,path);write_data_pdfmatrix(&node.data,d,path,out)}
Edge::ImmutablePdfPath(node)=>{d.alias(Kind::PdfPath,node.identity,path);write_data_pdfpath(&node.data,d,path,out)}
Edge::ImmutablePdfXobjForm(node)=>{d.alias(Kind::PdfXobjForm,node.identity,path);write_data_pdfxobjform(&node.data,d,path,out)}
Edge::ImmutableCropbox(node)=>{d.alias(Kind::Cropbox,node.identity,path);write_data_cropbox(&node.data,d,path,out)}
Edge::ImmutableMediabox(node)=>{d.alias(Kind::Mediabox,node.identity,path);write_data_mediabox(&node.data,d,path,out)}
Edge::ImmutablePdfFigure(node)=>{d.alias(Kind::PdfFigure,node.identity,path);write_data_pdffigure(&node.data,d,path,out)}
Edge::ImmutablePdfFormSubtype(node)=>{d.alias(Kind::PdfFormSubtype,node.identity,path);write_data_pdfformsubtype(&node.data,d,path,out)}
Edge::ImmutablePdfOriginalPath(node)=>{d.alias(Kind::PdfOriginalPath,node.identity,path);write_data_pdforiginalpath(&node.data,d,path,out)}
Edge::ImmutablePdfOriginalPathPrimitive(node)=>{d.alias(Kind::PdfOriginalPathPrimitive,node.identity,path);write_data_pdforiginalpathprimitive(&node.data,d,path,out)}
_=>unreachable!("table types are always shared"),
}}
fn discover_immutable_edge(d:&mut DumpContext<'_>,edge:&Edge,path:&str)->Result<(),AccessError>{match edge{
Edge::ImmutableBaseOperations(node)=>{d.alias(Kind::BaseOperations,node.identity,path);discover_data_baseoperations(&node.data,d,path)}
Edge::ImmutablePdfAffineTransform(node)=>{d.alias(Kind::PdfAffineTransform,node.identity,path);discover_data_pdfaffinetransform(&node.data,d,path)}
Edge::ImmutablePdfInlineForm(node)=>{d.alias(Kind::PdfInlineForm,node.identity,path);discover_data_pdfinlineform(&node.data,d,path)}
Edge::ImmutablePdfMatrix(node)=>{d.alias(Kind::PdfMatrix,node.identity,path);discover_data_pdfmatrix(&node.data,d,path)}
Edge::ImmutablePdfPath(node)=>{d.alias(Kind::PdfPath,node.identity,path);discover_data_pdfpath(&node.data,d,path)}
Edge::ImmutablePdfXobjForm(node)=>{d.alias(Kind::PdfXobjForm,node.identity,path);discover_data_pdfxobjform(&node.data,d,path)}
Edge::ImmutableCropbox(node)=>{d.alias(Kind::Cropbox,node.identity,path);discover_data_cropbox(&node.data,d,path)}
Edge::ImmutableMediabox(node)=>{d.alias(Kind::Mediabox,node.identity,path);discover_data_mediabox(&node.data,d,path)}
Edge::ImmutablePdfFigure(node)=>{d.alias(Kind::PdfFigure,node.identity,path);discover_data_pdffigure(&node.data,d,path)}
Edge::ImmutablePdfFormSubtype(node)=>{d.alias(Kind::PdfFormSubtype,node.identity,path);discover_data_pdfformsubtype(&node.data,d,path)}
Edge::ImmutablePdfOriginalPath(node)=>{d.alias(Kind::PdfOriginalPath,node.identity,path);discover_data_pdforiginalpath(&node.data,d,path)}
Edge::ImmutablePdfOriginalPathPrimitive(node)=>{d.alias(Kind::PdfOriginalPathPrimitive,node.identity,path);discover_data_pdforiginalpathprimitive(&node.data,d,path)}
_=>unreachable!("table types are always shared"),
}}
impl Node {fn into_immutable_edge(self)->Edge{let Self{identity,data}=self;match data{
NodeData::BaseOperations(data)=>Edge::ImmutableBaseOperations(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfAffineTransform(data)=>Edge::ImmutablePdfAffineTransform(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfInlineForm(data)=>Edge::ImmutablePdfInlineForm(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfMatrix(data)=>Edge::ImmutablePdfMatrix(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfPath(data)=>Edge::ImmutablePdfPath(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfXobjForm(data)=>Edge::ImmutablePdfXobjForm(Arc::new(ImmutableNode{identity,data})),
NodeData::Cropbox(data)=>Edge::ImmutableCropbox(Arc::new(ImmutableNode{identity,data})),
NodeData::Mediabox(data)=>Edge::ImmutableMediabox(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfFigure(data)=>Edge::ImmutablePdfFigure(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfFormSubtype(data)=>Edge::ImmutablePdfFormSubtype(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfOriginalPath(data)=>Edge::ImmutablePdfOriginalPath(Arc::new(ImmutableNode{identity,data})),
NodeData::PdfOriginalPathPrimitive(data)=>Edge::ImmutablePdfOriginalPathPrimitive(Arc::new(ImmutableNode{identity,data})),
_=>unreachable!("table and root nodes stay in NodeData"),
}}}
pub enum WriterView {}
impl Permission for WriterView {const WRITE:[bool;CLASS_COUNT]=[false,false,false,false,false,false,false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,false,false,false,true,false,false,false,false,false,true,false];}
impl Writes<types::Page> for WriterView {}
impl Writes<types::PdfFont> for WriterView {}
impl Writes<types::PdfXobject> for WriterView {}
impl TreeView<'_,WriterView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum LayoutParserView {}
impl Permission for LayoutParserView {const WRITE:[bool;CLASS_COUNT]=[false,true,true,false,false,false,false,false,false,false,false,true,false,false,false,false,false,false,true,false,false,false,false,false,false,false,false,false,false,true,true,false];}
impl Writes<types::Box> for LayoutParserView {}
impl Writes<types::GraphicState> for LayoutParserView {}
impl Writes<types::Page> for LayoutParserView {}
impl Writes<types::PageLayout> for LayoutParserView {}
impl Writes<types::PdfParagraph> for LayoutParserView {}
impl Writes<types::PdfStyle> for LayoutParserView {}
impl TreeView<'_,LayoutParserView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum ParagraphFinderView {}
impl Permission for ParagraphFinderView {const WRITE:[bool;CLASS_COUNT]=[false,true,true,false,false,false,false,false,false,false,false,true,false,false,false,false,false,false,true,true,true,false,false,false,false,true,true,true,true,true,true,false];}
impl Writes<types::Box> for ParagraphFinderView {}
impl Writes<types::GraphicState> for ParagraphFinderView {}
impl Writes<types::Page> for ParagraphFinderView {}
impl Writes<types::PageLayout> for ParagraphFinderView {}
impl Writes<types::PdfCharacter> for ParagraphFinderView {}
impl Writes<types::PdfFormula> for ParagraphFinderView {}
impl Writes<types::PdfLine> for ParagraphFinderView {}
impl Writes<types::PdfParagraph> for ParagraphFinderView {}
impl Writes<types::PdfParagraphComposition> for ParagraphFinderView {}
impl Writes<types::PdfSameStyleCharacters> for ParagraphFinderView {}
impl Writes<types::PdfStyle> for ParagraphFinderView {}
impl Writes<types::VisualBbox> for ParagraphFinderView {}
impl TreeView<'_,ParagraphFinderView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum StylesAndFormulasView {}
impl Permission for StylesAndFormulasView {const WRITE:[bool;CLASS_COUNT]=[false,false,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true,false,true,true,true,false,false,true,true,false,true,true,true,false];}
impl Writes<types::GraphicState> for StylesAndFormulasView {}
impl Writes<types::Page> for StylesAndFormulasView {}
impl Writes<types::PdfCharacter> for StylesAndFormulasView {}
impl Writes<types::PdfCurve> for StylesAndFormulasView {}
impl Writes<types::PdfForm> for StylesAndFormulasView {}
impl Writes<types::PdfFormula> for StylesAndFormulasView {}
impl Writes<types::PdfLine> for StylesAndFormulasView {}
impl Writes<types::PdfParagraph> for StylesAndFormulasView {}
impl Writes<types::PdfParagraphComposition> for StylesAndFormulasView {}
impl Writes<types::PdfStyle> for StylesAndFormulasView {}
impl TreeView<'_,StylesAndFormulasView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum AddDebugInformationView {}
impl Permission for AddDebugInformationView {const WRITE:[bool;CLASS_COUNT]=[false,true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,false,false,false,true,true,false];}
impl Writes<types::Box> for AddDebugInformationView {}
impl Writes<types::GraphicState> for AddDebugInformationView {}
impl Writes<types::Page> for AddDebugInformationView {}
impl Writes<types::PdfParagraph> for AddDebugInformationView {}
impl Writes<types::PdfStyle> for AddDebugInformationView {}
impl TreeView<'_,AddDebugInformationView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum TranslationView {}
impl Permission for TranslationView {const WRITE:[bool;CLASS_COUNT]=[false,true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true,false,false,false,false,true,false,true,false,true,true,true,true,false];}
impl Writes<types::Box> for TranslationView {}
impl Writes<types::GraphicState> for TranslationView {}
impl Writes<types::Page> for TranslationView {}
impl Writes<types::PdfFormula> for TranslationView {}
impl Writes<types::PdfParagraph> for TranslationView {}
impl Writes<types::PdfParagraphComposition> for TranslationView {}
impl Writes<types::PdfSameStyleCharacters> for TranslationView {}
impl Writes<types::PdfSameStyleUnicodeCharacters> for TranslationView {}
impl Writes<types::PdfStyle> for TranslationView {}
impl TreeView<'_,TranslationView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum ScanDetectionView {}
impl Permission for ScanDetectionView {const WRITE:[bool;CLASS_COUNT]=[false,false,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true,false,true,false,false,false,false,false,false,false,false,false,true,false];}
impl Writes<types::GraphicState> for ScanDetectionView {}
impl Writes<types::Page> for ScanDetectionView {}
impl Writes<types::PdfCharacter> for ScanDetectionView {}
impl Writes<types::PdfStyle> for ScanDetectionView {}
impl TreeView<'_,ScanDetectionView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum TypesettingView {}
impl Permission for TypesettingView {const WRITE:[bool;CLASS_COUNT]=[true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true,true];}
impl Writes<types::BaseOperations> for TypesettingView {}
impl Writes<types::Box> for TypesettingView {}
impl Writes<types::Cropbox> for TypesettingView {}
impl Writes<types::Document> for TypesettingView {}
impl Writes<types::GraphicState> for TypesettingView {}
impl Writes<types::Mediabox> for TypesettingView {}
impl Writes<types::Page> for TypesettingView {}
impl Writes<types::PageLayout> for TypesettingView {}
impl Writes<types::PdfAffineTransform> for TypesettingView {}
impl Writes<types::PdfCharacter> for TypesettingView {}
impl Writes<types::PdfCurve> for TypesettingView {}
impl Writes<types::PdfFigure> for TypesettingView {}
impl Writes<types::PdfFont> for TypesettingView {}
impl Writes<types::PdfFontCharBoundingBox> for TypesettingView {}
impl Writes<types::PdfForm> for TypesettingView {}
impl Writes<types::PdfFormSubtype> for TypesettingView {}
impl Writes<types::PdfFormula> for TypesettingView {}
impl Writes<types::PdfInlineForm> for TypesettingView {}
impl Writes<types::PdfLine> for TypesettingView {}
impl Writes<types::PdfMatrix> for TypesettingView {}
impl Writes<types::PdfOriginalPath> for TypesettingView {}
impl Writes<types::PdfOriginalPathPrimitive> for TypesettingView {}
impl Writes<types::PdfParagraph> for TypesettingView {}
impl Writes<types::PdfParagraphComposition> for TypesettingView {}
impl Writes<types::PdfPath> for TypesettingView {}
impl Writes<types::PdfRectangle> for TypesettingView {}
impl Writes<types::PdfSameStyleCharacters> for TypesettingView {}
impl Writes<types::PdfSameStyleUnicodeCharacters> for TypesettingView {}
impl Writes<types::PdfStyle> for TypesettingView {}
impl Writes<types::PdfXobjForm> for TypesettingView {}
impl Writes<types::PdfXobject> for TypesettingView {}
impl Writes<types::VisualBbox> for TypesettingView {}
impl TreeView<'_,TypesettingView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
pub enum ReadOnlyView {}
impl Permission for ReadOnlyView {const WRITE:[bool;CLASS_COUNT]=[false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false];}
pub enum GeometryView {}
impl Permission for GeometryView {const WRITE:[bool;CLASS_COUNT]=[false,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false];}
impl Writes<types::Box> for GeometryView {}
impl TreeView<'_,GeometryView> {pub fn as_readonlyview(&mut self)->TreeView<'_,ReadOnlyView>{TreeView{doc:self.doc.reborrow(),_permission:PhantomData}}}
