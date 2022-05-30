use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;

use crate::{Decimal, ForStatement, Location};

mod constructor;
mod display;
pub mod for_statement;
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
    ForStatement(Box<ForStatement>),
}

/// - `{%=`: Destroy all whitespace
/// - `{%-`: Destroy all blank lines
/// - `{% `: Destroy whitespace, and the first newline encountered
/// - `{%_`: Destroy whitespace
#[derive(Debug)]
pub enum SpaceDestroyer {
    /// Destroy all whitespace
    Everything,
    /// Destroy all blank lines
    NewlineAll,
    /// Destroy whitespace, and the first newline encountered
    NewlineOne,
    /// Destroy whitespace
    Nothing,
}

impl SahaNode {}
