use crate::Identifier;
use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForStatement {
    pub pattern: DjvPattern,
    pub iterable: DjvNode,
    pub body: Vec<DjvNode>,
    pub backpack: Vec<DjvNode>,
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

impl ForStatement {
    /// Creates a new for statement from the given `pattern` and `iterable` object.
    ///
    /// The rust compiler checks the validity of iterators
    pub fn new(pattern: DjvNode, iterable: DjvNode) -> Self {
        Self { pattern, iterable, body: vec![], backpack: vec![] }
    }
    /// Set the body of the for statement, activate all [`SpaceDestroyer`].
    pub fn set_body(mut self, body: Vec<DjvNode>) {
        self.body = SpaceDestroyer::clear(body)
    }
    /// Set the else backpack of the for statement, activate all [`SpaceDestroyer`].
    pub fn set_backpack(mut self, backpack: Vec<DjvNode>) {
        self.backpack = SpaceDestroyer::clear(backpack)
    }
    pub fn as_node(self, span: &Range<usize>, file: &FileID) -> DjvNode {
        DjvNode { kind: DjvKind::ForStatement(Box::new(self)), span: span.clone(), file: file.clone() }
    }
}

impl IfStatement {
    pub fn new(branches: Vec<IfConditional>, backpack: Vec<DjvNode>, span: &Range<usize>, file: &FileID) -> DjvNode {
        DjvNode { kind: DjvKind::IfStatement(Box::new(Self { branches, backpack })), span: span.clone(), file: file.clone() }
    }
}
