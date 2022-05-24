use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;

use crate::{Decimal, Location};

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

#[repr(u8)]
#[derive(Debug)]
pub enum SahaValue {
    Null = 0,
    Boolean(bool),
    Text(Box<String>),
    Number(Box<Decimal>),
    Identifier(Box<String>),
    Vector,
}
// - `{%=`: Destroy all whitespace on the left
// - `{%-`: Destroy all blank lines on the left
// - `{% `: Destroy left whitespace, and the first newline encountered
// - `{%_`: destroy left whitespace, but exclude newlines
pub enum WhiteSpaceDestroyer {}

impl SahaNode {}
