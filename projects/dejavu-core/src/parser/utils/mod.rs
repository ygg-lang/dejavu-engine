use super::*;
use crate::hir::{DejavuStatement, DejavuText, DejavuTextTrim};
use dejavu_parser::dejavu::{ElementNode, SpaceControlNode, TemplateLNode, TemplateRNode};

pub fn take_elements(s: &[ElementNode]) -> DejavuRoot {
    let mut out = DejavuRoot::default();
    for x in s {
        match x {
            ElementNode::TemplateExport(_) => {}
            ElementNode::TextMany(v) => out += take_text(&v.text_element),
            ElementNode::TemplateIf(v) => {
                // out.trim_text(take_control_l(v.if_begin.template_l.space_control.clone(), true));
                // last_trim = take_control_r(v.if_end.template_r.space_control.clone(), true);
                out += DejavuStatement::Branches(v.into());
            }
            ElementNode::TemplateFor(_) => {}
        }
    }
    out
}

pub fn take_text(texts: &[TextElementNode]) -> DejavuStatement {
    let mut out = DejavuText::default();
    match texts {
        [head, middle @ .., tail] => {
            if head.pure_space() {
                head.write_buffer(&mut out.head)
            }
            else {
                head.write_buffer(&mut out.body)
            }
            for term in middle {
                term.write_buffer(&mut out.body)
            }
            if tail.pure_space() { tail.write_buffer(&mut out.body) } else { tail.write_buffer(&mut out.body) }
        }
        _ => panic!("many_text: {}", texts.len()),
    }
    DejavuStatement::Text(out)
}

impl From<SpaceControlNode> for DejavuTextTrim {
    fn from(value: SpaceControlNode) -> Self {
        match value {
            SpaceControlNode::SpaceControl0 => DejavuTextTrim::Nothing,
            SpaceControlNode::SpaceControl1 => DejavuTextTrim::UntilLineBreak,
            SpaceControlNode::SpaceControl2 => DejavuTextTrim::RecentLineBreak,
            SpaceControlNode::SpaceControl3 => DejavuTextTrim::FurthestLineBreak,
            SpaceControlNode::SpaceControl4 => DejavuTextTrim::AllLineBreaks,
        }
    }
}

pub fn take_control_l(o: &TemplateLNode, statement: bool) -> DejavuTextTrim {
    match &o.space_control {
        Some(s) => s.clone().into(),
        None if statement => DejavuTextTrim::RecentLineBreak,
        None => DejavuTextTrim::Nothing,
    }
}

pub fn take_control_r(o: &TemplateRNode, statement: bool) -> DejavuTextTrim {
    match &o.space_control {
        Some(s) => s.clone().into(),
        None if statement => DejavuTextTrim::UntilLineBreak,
        None => DejavuTextTrim::Nothing,
    }
}
