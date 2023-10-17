use super::*;
use dejavu_parser::dejavu::{AtomicNode, BooleanNode, TermNode};

impl<'i> From<&'i TemplateIfNode> for DejavuBranches {
    fn from(value: &TemplateIfNode) -> Self {
        let mut out = DejavuBranches::new(value.if_else_if.len());
        let cond = value.conditions();
        let start = value.rights();
        let end = value.lefts();

        for (index, node) in value.bodies().iter().enumerate() {
            let s = unsafe { take_control_r(start.get_unchecked(index), true) };
            let e = unsafe { take_control_l(end.get_unchecked(index), true) };
            match cond.get(index) {
                Some(cond) => {
                    let mut body = take_elements(node);
                    body.trim_text(s, e);
                    out += DejavuConditional { condition: DejavuExpression::from(*cond), body: body.clone() }
                }
                None => {
                    let mut body = take_elements(node);
                    body.trim_text(s, e);
                    out.default = body.clone()
                }
            }
        }

        out
    }
}

impl<'i> From<&'i ExpressionNode> for DejavuExpression {
    fn from(value: &ExpressionNode) -> Self {
        let base = Self::from(&value.term);
        base
    }
}

impl<'i> From<&'i TermNode> for DejavuExpression {
    fn from(value: &TermNode) -> Self {
        Self::from(&value.atomic)
    }
}

impl<'i> From<&'i AtomicNode> for DejavuExpression {
    fn from(value: &AtomicNode) -> Self {
        match value {
            AtomicNode::Atomic0 => Self::Null,
            AtomicNode::Boolean(v) => Self::from(v),
            AtomicNode::Identifier(_) => Self::Null,
            AtomicNode::Number(_) => Self::Null,
        }
    }
}

impl<'i> From<&'i BooleanNode> for DejavuExpression {
    fn from(value: &BooleanNode) -> Self {
        match value {
            BooleanNode::Boolean0 => Self::Boolean(true),
            BooleanNode::Boolean1 => Self::Boolean(false),
        }
    }
}
