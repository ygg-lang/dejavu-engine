use askama::Template;
use dejavu_ir::hir::{DejavuRoot, DejavuStatement};

#[derive(Template)]
#[template(path = "out.rs.jinja", escape = "none")]
pub struct Builder {
    root: Vec<DejavuStatement>,
}

impl Builder {
    pub fn new(s: &str) -> Self {
        DejavuRoot::from()
    }
}
