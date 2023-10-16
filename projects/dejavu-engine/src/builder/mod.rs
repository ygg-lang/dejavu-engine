use askama::Template;
use dejavu_ir::hir::{DejavuRoot, DejavuStatement};
use std::str::FromStr;

#[derive(Debug, Template)]
#[template(path = "out.rs.jinja", escape = "none")]
pub struct DejavuBuilder {
    statements: Vec<DejavuStatement>,
}

impl DejavuBuilder {
    pub fn new(s: &str) -> Self {
        Self { statements: DejavuRoot::from_str(s).unwrap().statements }
    }
}
