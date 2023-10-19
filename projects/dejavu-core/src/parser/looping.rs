use crate::hir::DejavuIdentifier;

use super::*;

impl<'i> From<&'i TemplateForNode> for DejavuLoop {
    fn from(value: &TemplateForNode) -> Self {
        let pattern = DejavuPattern::from(&value.for_begin.pattern);
        let iterator = DejavuExpression::from(&value.for_begin.iterator);
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

impl<'i> From<&'i PatternNode> for DejavuPattern {
    fn from(node: &'i PatternNode) -> Self {
        match node {
            PatternNode::BarePattern(v) => DejavuPattern::from(v),
        }
    }
}

impl<'i> From<&'i BarePatternNode> for DejavuPattern {
    fn from(node: &'i BarePatternNode) -> Self {
        Self::Bare(node.identifier.iter().map(DejavuIdentifier::from).collect())
    }
}
