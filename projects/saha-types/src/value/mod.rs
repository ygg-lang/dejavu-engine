use crate::Location;

pub struct SahaNode {
    pub kind: SahaValue,
    pub span: Location,
}

pub enum SahaValue {
    Null,
    Boolean(bool),
    Number(Number),
    Vector,
}

pub struct Number {}
