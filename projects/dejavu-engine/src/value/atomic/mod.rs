mod display;

use super::*;

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

impl Namespace {
    #[inline]
    pub fn new(path: Vec<Identifier>, span: &Range<usize>, file: &FileID) -> DjvNode {
        DjvNode { kind: DjvKind::Namespace(Namespace { path }), span: span.clone(), file: file.clone() }
    }
}


