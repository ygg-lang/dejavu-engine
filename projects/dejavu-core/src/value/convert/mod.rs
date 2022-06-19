use crate::value::{ASTKind, DjvNode};

impl From<u8> for DjvNode {
    fn from(value: u8) -> Self {
        DjvNode { kind: ASTKind::Integer(value as i128), span: Default::default(), file: Default::default() }
    }
}

impl From<f64> for DjvNode {
    fn from(value: f64) -> Self {
        DjvNode { kind: ASTKind::Decimal(value), span: Default::default(), file: Default::default() }
    }
}
