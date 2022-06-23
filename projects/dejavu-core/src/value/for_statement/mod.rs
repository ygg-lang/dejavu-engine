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
    pub branches: Vec<IfConditional>,
    pub backpack: Vec<DjvNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfConditional {
    pub condition: DjvNode,
    pub body: Vec<DjvNode>,
}

impl IfStatement {
    pub fn new(branches: Vec<IfConditional>, backpack: Vec<DjvNode>, span: &Range<usize>, file: &FileID) -> DjvNode {
        let value = Self { branches, backpack };
        DjvNode { kind: DjvKind::IfStatement(Box::new(value)), span: span.clone(), file: file.clone() }
    }
}
