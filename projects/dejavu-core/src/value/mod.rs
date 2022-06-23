use diagnostic_quick::{QError, QResult};
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
    str::FromStr,
};

use crate::BinaryExpression;
use diagnostic_quick::FileID;
use serde::{Deserialize, Serialize};

use crate::value::for_statement::ForStatement;

mod constructor;
mod convert;
mod display;
pub mod expression;
pub mod for_statement;
pub mod ser;
mod whitespace;
mod write_rust;
pub mod atomic;

#[derive(Serialize, Deserialize)]
pub struct DjvNode {
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
    Integer(i128),
    Decimal(f64),
    Identifier(Vec<String>),
    Vector(Vec<DjvNode>),
    Statements(Vec<DjvNode>),
    LeftDestroyer(SpaceDestroyer),
    RightDestroyer(SpaceDestroyer),
    ForStatement(Box<ForStatement>),
    Binary(Box<BinaryExpression>),
}

/// Space destroyer, destroy space by need
///
/// - `=`: Destroy all whitespace
/// - `~`: Destroy all blank lines
/// - `-`: Destroy whitespace, and the first newline encountered
/// - `_`: Destroy whitespace, stop at first newline encountered
/// - `!`: Destroy nothing
#[derive(Debug, Serialize, Deserialize)]
pub enum SpaceDestroyer {
    /// Destroy all whitespace
    Everything,
    /// Destroy all blank lines
    NewlineAll,
    /// Destroy whitespace and the first newline encountered
    NewlineOne,
    /// Destroy all whitespace, stop at first newline encountered
    WhiteSpaceOnly,
    /// Destroy nothing
    Nothing,
}

impl DjvNode {}
