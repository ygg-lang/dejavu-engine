use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForStatement {
    pub body: Vec<SahaNode>,
}

impl From<ForStatement> for SahaNode {
    fn from(value: ForStatement) -> Self {
        SahaNode { kind: SahaValue::ForStatement(Box::new(value)), span: Default::default() }
    }
}
