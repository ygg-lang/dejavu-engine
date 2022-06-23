use pratt::{Associativity, Precedence};

use super::*;

mod unary;
mod binary;

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub binary: BinaryOperator,
    pub lhs: DjvNode,
    pub rhs: DjvNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub lhs: DjvNode,
    pub rhs: DjvNode,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,      // !
    Negative, // -
    Try,      // ?
}



impl From<BinaryExpression> for DjvNode {
    fn from(value: BinaryExpression) -> Self {
        DjvNode { kind: DjvKind::Binary(Box::new(value)), span: Default::default(), file: Default::default() }
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
