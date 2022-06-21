use pratt::{Associativity, Precedence};

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

impl BinaryOperator {
    pub fn as_precedence(&self) -> Precedence {
        let o = match self {
            BinaryOperator::Addition => 10,
            BinaryOperator::Subtraction => 10,
            BinaryOperator::Multiplication => 20,
            BinaryOperator::Division => 20,
            BinaryOperator::Power => 20,
            BinaryOperator::Assign => 5,
        };
        Precedence(o)
    }
    pub fn as_associativity(&self) -> Associativity {
        match self {
            BinaryOperator::Addition => Associativity::Left,
            BinaryOperator::Subtraction => Associativity::Left,
            BinaryOperator::Multiplication => Associativity::Left,
            BinaryOperator::Division => Associativity::Left,
            BinaryOperator::Power => Associativity::Right,
            BinaryOperator::Assign => Associativity::Neither,
        }
    }
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

impl FromStr for BinaryOperator {
    type Err = QError;

    fn from_str(s: &str) -> QResult<Self> {
        let o = match s {
            "+" => Self::Addition,
            _ => Err(QError::syntax_error(format!("Unknown binary operator: {}", s)))?,
        };
        Ok(o)
    }
}
