use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: DjvNode,
    pub right: DjvNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BinaryOperator {
    /// `+`
    Addition,
    /// `-`
    Subtraction,
    /// `*`
    Multiplication,
    /// `/`
    Division,
    /// `^`
    Power,
    /// `=`
    Assign,
}

#[derive(Debug)]
pub enum UnOp {
    Not, // !
    Neg, // -
    Try, // ?
}

impl From<BinaryExpression> for DjvNode {
    fn from(value: BinaryExpression) -> Self {
        DjvNode { kind: ASTKind::Binary(Box::new(value)), span: Default::default(), file: Default::default() }
    }
}
