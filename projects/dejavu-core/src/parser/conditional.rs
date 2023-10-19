use dejavu_parser::dejavu::{AtomicNode, BooleanNode, IdentifierNode, TermNode};

use crate::hir::DejavuIdentifier;

use super::*;

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
            AtomicNode::Boolean(BooleanNode::True) => Self::Boolean(true),
            AtomicNode::Boolean(BooleanNode::False) => Self::Boolean(false),
            AtomicNode::Identifier(v) => Self::Identifier(DejavuIdentifier::from(v)),
            AtomicNode::Number(_) => Self::Null,
        }
    }
}

impl<'i> From<&'i IdentifierNode> for DejavuIdentifier {
    fn from(value: &IdentifierNode) -> Self {
        Self { text: "".to_string(), range: value.get_range().unwrap_or_default() }
    }
}
