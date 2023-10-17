use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter, Write},
    ops::{AddAssign, Range},
};

use indentation::{display_wrap, DisplayIndent, IndentFormatter};

pub use self::{
    conditional::{DejavuBranches, DejavuConditional},
    expr::DejavuExpression,
    text::{DejavuText, DejavuTextTrim},
};

mod conditional;
mod expr;
mod text;

#[derive(Clone, Debug)]
pub struct DejavuRoot {
    pub statements: Vec<DejavuStatement>,
}

#[derive(Clone)]
pub enum DejavuStatement {
    Text(DejavuText),
    Branches(DejavuBranches),
}

impl Debug for DejavuStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            DejavuStatement::Text(v) => Debug::fmt(v, f),
            DejavuStatement::Branches(v) => Debug::fmt(v, f),
        }
    }
}

display_wrap![DejavuStatement, DejavuText, DejavuBranches, DejavuConditional];
impl DisplayIndent for DejavuStatement {
    fn fmt_indent<W: Write>(&self, f: IndentFormatter<W>) -> core::fmt::Result {
        match self {
            DejavuStatement::Text(v) => v.fmt_indent(f),
            DejavuStatement::Branches(v) => v.fmt_indent(f),
        }
    }
}

impl Default for DejavuRoot {
    fn default() -> Self {
        Self { statements: Vec::new() }
    }
}

impl<T> AddAssign<T> for DejavuRoot
where
    T: Into<DejavuStatement>,
{
    fn add_assign(&mut self, rhs: T) {
        self.statements.push(rhs.into())
    }
}

impl DejavuRoot {
    pub fn new(capacity: usize) -> Self {
        Self { statements: Vec::with_capacity(capacity) }
    }
    /// `%> ... <%`
    pub fn trim_text(&mut self, head: DejavuTextTrim, tail: DejavuTextTrim) {
        if let Some(DejavuStatement::Text(v)) = self.statements.first_mut() {
            v.trim_head(head)
        }
        if let Some(DejavuStatement::Text(v)) = self.statements.last_mut() {
            v.trim_tail(tail)
        }
    }
}

pub trait CodeGenerator {}
