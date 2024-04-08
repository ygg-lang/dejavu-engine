use crate::hir::DejavuIdentifier;
use dejavu_parser::dejavu::ForBlockNode;

use super::*;

impl<'i> From<ForBlockNode<'i>> for DejavuLoop {
    fn from(value: ForBlockNode<'i>) -> Self {
        let pattern = DejavuPattern::from(value.template_for().pattern());
        let iterator = DejavuExpression::from(&value.template_for().iterator);
        let condition = value.for_begin.condition.as_ref().map(DejavuExpression::from);
        let mut body = take_elements(&value.for_begin.element);
        let s = take_control_r(&value.for_begin.template_r, true);
        let e = match &value.for_else {
            Some(s) => take_control_l(&s.template_l, true),
            None => take_control_l(&value.for_end.template_l, true),
        };
        body.trim_text(s, e);
        let otherwise = match &value.for_else {
            Some(other) => {
                let mut body = take_elements(&other.element);
                let s = take_control_r(&other.template_r, true);
                let e = take_control_l(&value.for_end.template_l, true);
                body.trim_text(s, e);
                Some(body)
            }
            None => None,
        };
        DejavuLoop { pattern, iterator, condition, body, otherwise }
    }
}

impl<'i> From<PatternNode<'i>> for DejavuPattern {
    fn from(node: PatternNode<'i>) -> Self {
        match node {
            PatternNode::BarePattern(v) => DejavuPattern::from(v),
            PatternNode::Identifier(_) => {}
        }
    }
}

impl<'i> From<BarePatternNode<'i>> for DejavuPattern {
    fn from(node: BarePatternNode<'i>) -> Self {
        Self::Bare(node.identifier.iter().map(DejavuIdentifier::from).collect())
    }
}
