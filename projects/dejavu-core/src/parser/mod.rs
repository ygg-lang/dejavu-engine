mod conditional;
mod looping;
mod utils;

use crate::{
    hir::{DejavuBranches, DejavuConditional, DejavuExpression, DejavuRoot, DejavuStatement, DejavuTextTrim},
    parser::utils::{take_control_l, take_control_r, take_elements},
};
use alloc::string::String;
use core::{mem::take, str::FromStr};
use dejavu_parser::{
    dejavu::{DejavuRule, ExpressionNode, RootNode, TemplateIfNode, TextElementNode},
    YggdrasilError,
};

impl From<RootNode> for DejavuRoot {
    fn from(value: RootNode) -> Self {
        take_elements(&value.element)
    }
}

impl FromStr for DejavuRoot {
    type Err = YggdrasilError<DejavuRule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
