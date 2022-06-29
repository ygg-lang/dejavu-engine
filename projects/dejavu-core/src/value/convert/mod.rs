use crate::value::{DjvKind, DjvNode};

impl From<bool> for DjvNode {
    fn from(value: bool) -> Self {
        DjvNode { kind: DjvKind::Boolean(value), span: Default::default(), file: Default::default() }
    }
}

impl From<u8> for DjvNode {
    fn from(value: u8) -> Self {
        DjvNode { kind: DjvKind::Integer(value as i128), span: Default::default(), file: Default::default() }
    }
}

impl From<f64> for DjvNode {
    fn from(value: f64) -> Self {
        DjvNode { kind: DjvKind::Decimal(value), span: Default::default(), file: Default::default() }
    }
}

impl From<&str> for DjvNode {
    fn from(value: &str) -> Self {
        DjvNode { kind: DjvKind::Text(value.to_string()), span: Default::default(), file: Default::default() }
    }
}
