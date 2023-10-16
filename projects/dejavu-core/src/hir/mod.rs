use alloc::{string::String, vec::Vec};
use core::ops::{AddAssign, Range};

mod conditional;
mod text;

pub use self::{
    conditional::{DejavuBranches, DejavuConditional},
    text::{DejavuText, DejavuTextTrim},
};

#[derive(Debug)]
pub struct DejavuRoot {
    pub statements: Vec<DejavuStatement>,
}

#[derive(Debug)]
pub enum DejavuStatement {
    Text(DejavuText),
    Branches(DejavuBranches),
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

#[derive(Debug, Default)]
pub struct DejavuExpression {}

pub trait CodeGenerator {}
