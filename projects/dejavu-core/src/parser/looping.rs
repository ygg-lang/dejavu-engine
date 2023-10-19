use crate::{
    hir::{DejavuExpression, DejavuLoop, DejavuPattern},
    parser::utils::{take_control_l, take_control_r, take_elements},
};
use dejavu_parser::dejavu::{BarePatternNode, ForElseNode, PatternNode, TemplateForNode};

impl<'i> From<&'i TemplateForNode> for DejavuLoop {
    fn from(value: &TemplateForNode) -> Self {
        let pattern = DejavuPattern::from(&value.for_begin.pattern);
        let iterator = DejavuExpression::from(&value.for_begin.iterator);
        let mut body = take_elements(&value.for_begin.element);
        let s = take_control_r(&value.for_begin.template_r, true);
        let e = match &value.for_else {
            Some(s) => take_control_l(&s.template_l, true),
            None => take_control_l(&value.for_end.template_l, true),
        };
        body.trim_text(s, e);
        DejavuLoop { pattern, iterator, condition: None, body, otherwise: None }
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
        Self::Bare(vec![])
    }
}
