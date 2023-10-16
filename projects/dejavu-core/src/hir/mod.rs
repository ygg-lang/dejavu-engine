use alloc::{string::String, vec::Vec};
use core::ops::Range;


pub struct DejavuRoot {
    pub statements: Vec<DejavuStatement>,
}

impl Default for DejavuRoot {
    fn default() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

pub enum DejavuStatement {
    Text(DejavuText),
}

#[derive(Default)]
pub struct DejavuText {
    pub head: String,
    pub body: String,
    pub tail: String,
    pub range: Range<usize>,
}

pub trait CodeGenerator {}
