use dejavu_ir::hir::{DejavuRoot, DejavuSequence, DejavuStatement};
use std::str::FromStr;

pub struct DejavuBuilder {
    statements: DejavuRoot,
}
impl DejavuBuilder {
    pub fn new(s: &str) -> Self {
        Self { statements: DejavuRoot::from_str(s).unwrap() }
    }
}
