use diagnostic_quick::FileID;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use serde::{Deserialize, Serialize};

use crate::value::for_statement::ForStatement;

mod constructor;
mod display;
pub mod for_statement;
pub mod ser;
mod whitespace;

#[derive(Serialize, Deserialize)]
pub struct SahaNode {
    pub kind: ASTKind,
    pub span: Range<usize>,
    pub file: FileID,
}

#[repr(u8)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ASTKind {
    Null = 0,
    Boolean(bool),
    Text(String),
    Number(Decimal),
    Identifier(String),
    Vector(Vec<SahaNode>),
    Statements(Vec<SahaNode>),
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
