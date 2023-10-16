use alloc::{string::String, vec::Vec};
use core::ops::Range;

mod text;

pub use self::text::{DejavuText, DejavuTextTrim};

#[derive(Debug)]
pub struct DejavuRoot {
    pub statements: Vec<DejavuStatement>,
}

impl Default for DejavuRoot {
    fn default() -> Self {
        Self { statements: Vec::new() }
    }
}

#[derive(Debug)]
pub enum DejavuStatement {
    Text(DejavuText),
    If(DejavuIfElse),
}

#[derive(Debug)]
pub struct DejavuIfElse {
    pub then: DejavuBranch,
    pub else_if: Vec<DejavuBranch>,
    pub default: Option<DejavuBranch>,
}

#[derive(Debug)]
pub struct DejavuBranch {
    pub condition: DejavuExpression,
    pub body: Vec<DejavuStatement>,
}

#[derive(Debug)]
pub struct DejavuExpression {}

pub trait CodeGenerator {}
