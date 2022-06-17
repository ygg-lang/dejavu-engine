use crate::value::{ASTKind, SahaNode};

impl From<u8> for SahaNode {
    fn from(value: u8) -> Self {
        SahaNode { kind: ASTKind::Integer(value as i128), span: Default::default(), file: Default::default() }
    }
}

impl From<f64> for SahaNode {
    fn from(value: f64) -> Self {
        SahaNode { kind: ASTKind::Decimal(value), span: Default::default(), file: Default::default() }
    }
}
