use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;

use voml_collection::Number;

use crate::Location;

mod constructor;

#[derive(Debug)]
pub struct SahaNode {
    pub kind: SahaValue,
    pub span: Location,
}

impl Display for SahaNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

#[derive(Debug)]
pub enum SahaValue {
    Null,
    Boolean(bool),
    Text(Box<String>),
    Number(Box<Number>),
    Identifier(Box<Identifier>),
    Vector,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

impl SahaNode {}
