use crate::Location;

pub struct SahaNode {
    value: SahaValue,
    span: Location,
}

pub enum SahaValue {
    Null,
    Boolean(bool),
    Number(Number),
    Vector,
}

pub struct Number {}
