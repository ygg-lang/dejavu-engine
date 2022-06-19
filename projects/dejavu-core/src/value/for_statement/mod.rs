use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForStatement {
    pub body: Vec<DjvNode>,
}

impl From<ForStatement> for DjvNode {
    fn from(value: ForStatement) -> Self {
        DjvNode { kind: ASTKind::ForStatement(Box::new(value)), span: Default::default(), file: Default::default() }
    }
}
