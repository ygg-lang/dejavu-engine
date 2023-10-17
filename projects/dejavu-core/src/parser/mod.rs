use core::str::FromStr;

use dejavu_parser::{
    dejavu::{
        DejavuRule, ElementNode, ExpressionNode, RootNode, SpaceControlNode, TemplateIfNode, TemplateLNode, TemplateRNode,
        TextElementNode,
    },
    YggdrasilError, YggdrasilNode,
};

use crate::{
    hir::{
        DejavuBranches, DejavuConditional, DejavuExpression, DejavuRoot, DejavuSequence, DejavuStatement, DejavuText,
        DejavuTextTrim,
    },
    parser::utils::{take_control_l, take_control_r, take_elements},
};

mod conditional;
mod looping;
mod utils;

impl FromStr for DejavuRoot {
    type Err = YggdrasilError<DejavuRule>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = RootNode::from_str(s)?;
        Ok(Self { body: take_elements(&value.element) })
    }
}
