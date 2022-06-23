use crate::{BinaryExpression, IfStatement, Namespace};
use diagnostic_quick::{FileID, QError, QResult};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
    str::FromStr,
};

use crate::value::for_statement::ForStatement;

pub mod atomic;
mod constructor;
mod convert;
mod display;
pub mod expression;
pub mod for_statement;
pub mod ser;
mod whitespace;
mod write_rust;

#[derive(Serialize, Deserialize)]
pub struct DjvNode {
    pub kind: DjvKind,
    pub span: Range<usize>,
    pub file: FileID,
}

#[repr(u8)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DjvKind {
    Null = 0,
    Boolean(bool),
    Text(String),
    Integer(i128),
    Decimal(f64),
    Namespace(Namespace),
    Vector(Vec<DjvNode>),
    Statements(Vec<DjvNode>),
    LeftDestroyer(SpaceDestroyer),
    RightDestroyer(SpaceDestroyer),
    IfStatement(Box<IfStatement>),
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
