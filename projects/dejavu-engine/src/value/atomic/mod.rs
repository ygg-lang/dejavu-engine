use super::*;

mod display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub path: Vec<Identifier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DjvPattern {
    pub symbols: Vec<Identifier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DjvString {
    pub kind: StringKind,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StringKind {
    SingleQuote,
    DoubleQuote,
}
impl Namespace {
    #[inline]
    pub fn new(path: Vec<Identifier>, span: &Range<usize>, file: &FileID) -> DjvNode {
        DjvNode { kind: DjvKind::Namespace(Namespace { path }), span: span.clone(), file: file.clone() }
    }
}

impl DjvString {
    pub fn to_node(self, span: &Range<usize>, file: &FileID) -> DjvNode {
        DjvNode { kind: DjvKind::String(Box::new(self)), span: span.clone(), file: file.clone() }
    }
}
