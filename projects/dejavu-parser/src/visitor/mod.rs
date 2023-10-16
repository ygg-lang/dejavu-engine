use crate::{
    dejavu::{RootNode, TemplateIfNode, TextElementNode},
    utils::{take_control_r, take_elements},
};
use dejavu_ir::hir::{DejavuBranches, DejavuRoot, DejavuStatement, DejavuTextTrim};

mod conditional;

impl RootNode {
    pub fn as_hir(&self) -> DejavuRoot {
        take_elements(&self.element)
    }
}

impl TextElementNode {
    pub fn pure_space(&self) -> bool {
        matches!(self, Self::TextSpace { .. })
    }
    pub fn write_buffer(&self, w: &mut String) {
        match self {
            TextElementNode::TemplateE(_) => w.push_str("<%"),
            TextElementNode::TextSpace(s) => w.push_str(&s.text),
            TextElementNode::TextWord(s) => w.push_str(&s.text),
        }
    }
}
