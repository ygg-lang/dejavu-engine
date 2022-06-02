use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;
use serde::{Deserialize, Serialize};

use crate::{Decimal, ForStatement, Location};

mod constructor;
mod display;
pub mod for_statement;
mod whitespace;

#[derive(Debug, Serialize, Deserialize)]
pub struct SahaNode {
    pub kind: SahaValue,
    pub span: Location,
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SahaValue {
    Null = 0,
    Boolean(bool),
    Text(String),
    Number(Decimal),
    Identifier(String),
    Vector(Vec<SahaValue>),
    Statements(Vec<SahaValue>),
    LeftDestroyer(SpaceDestroyer),
    RightDestroyer(SpaceDestroyer),
    ForStatement(Box<ForStatement>),
}

/// - `=`: Destroy all whitespace
/// - `-`: Destroy all blank lines
/// - `_`: Destroy whitespace, and the first newline encountered
/// - `!`: Destroy nothing
#[derive(Debug, Serialize, Deserialize)]
pub enum SpaceDestroyer {
    /// Destroy all whitespace
    Everything,
    /// Destroy all blank lines
    NewlineAll,
    /// Destroy whitespace, and the first newline encountered
    NewlineOne,
    /// Destroy nothing
    Nothing,
}

impl SahaNode {}
