use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForStatement {
    pub body: Vec<DjvNode>,
}

impl From<ForStatement> for DjvNode {
    fn from(value: ForStatement) -> Self {
        DjvNode { kind: DjvKind::ForStatement(Box::new(value)), span: Default::default(), file: Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
    pub body: Vec<DjvNode>,
}

impl IfStatement {
    pub fn new(body: Vec<DjvNode>, span: &Range<usize>, file: &FileID) -> DjvNode {
        let value = Self { body };
        DjvNode { kind: DjvKind::IfStatement(Box::new(value)), span: span.clone(), file: file.clone() }
    }
}
