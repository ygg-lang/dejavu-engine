use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;

use crate::{Decimal, Location};

mod constructor;
mod display;
mod whitespace;

#[derive(Debug)]
pub struct SahaNode {
    pub kind: SahaValue,
    pub span: Location,
}

#[repr(u8)]
#[derive(Debug)]
pub enum SahaValue {
    Null = 0,
    Boolean(bool),
    Text(String),
    Number(Box<Decimal>),
    Identifier(String),
    Vector,
    Statements(Vec<SahaValue>),
    LeftDestroyer(SpaceDestroyer),
    RightDestroyer(SpaceDestroyer),
}

/// - `{%=`: Destroy all whitespace
/// - `{%-`: Destroy all blank lines
/// - `{% `: Destroy whitespace, and the first newline encountered
/// - `{%_`: Destroy whitespace
#[derive(Debug)]
pub enum SpaceDestroyer {
    Everything,
    NewlineAll,
    NewlineOne,
    Nothing,
}

impl SahaNode {}

