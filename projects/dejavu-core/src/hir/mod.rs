use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    ops::{AddAssign, Range},
};

mod conditional;
mod expr;
mod text;

pub use self::{
    conditional::{DejavuBranches, DejavuConditional},
    expr::DejavuExpression,
    text::{DejavuText, DejavuTextTrim},
};

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

impl Display for DejavuStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            DejavuStatement::Text(v) => Display::fmt(v, f),
            DejavuStatement::Branches(v) => Display::fmt(v, f),
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
