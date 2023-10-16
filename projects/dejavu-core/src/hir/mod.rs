use alloc::{string::String, vec::Vec};
use core::ops::Range;

pub struct DejavuRoot {
    pub statements: Vec<DejavuStatement>,
}

pub enum DejavuStatement {
    Text(DejavuText),
}

pub struct DejavuText {
    text: String,
    range: Range<usize>,
}

pub trait CodeGenerator {}
