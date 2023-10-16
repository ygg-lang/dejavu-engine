use crate::{
    dejavu::{RootNode, TemplateIfNode, TextElementNode},
    utils::take_control_r,
};
use dejavu_ir::hir::{DejavuBranches, DejavuRoot, DejavuStatement, DejavuTextTrim};
use crate::utils::take_elements;

mod conditional;

impl RootNode {
    pub fn as_hir(&self) -> DejavuRoot {
        DejavuRoot {
            statements: take_elements(&self.element, DejavuTextTrim::Nothing),
        }

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
