use crate::dejavu::{ElementNode, SpaceControlNode, TextElementNode};
use dejavu_ir::hir::{DejavuRoot, DejavuStatement, DejavuText, DejavuTextTrim};

pub fn take_elements(s: &[ElementNode]) -> DejavuRoot {
    let mut out = DejavuRoot::default();
    for x in s {
        match x {
            ElementNode::TemplateExport(_) => {}
            ElementNode::TemplateIf(v) => {
                // out.trim_text(take_control_l(v.if_begin.template_l.space_control.clone(), true));
                // last_trim = take_control_r(v.if_end.template_r.space_control.clone(), true);
                out += v.as_hir();
            }
            ElementNode::TextMany(v) => out += take_text(&v.text_element),
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

impl SpaceControlNode {
    fn as_hir(&self) -> DejavuTextTrim {
        match self {
            SpaceControlNode::SpaceControl0 => DejavuTextTrim::Nothing,
            SpaceControlNode::SpaceControl1 => DejavuTextTrim::UntilLineBreak,
            SpaceControlNode::SpaceControl2 => DejavuTextTrim::RecentLineBreak,
            SpaceControlNode::SpaceControl3 => DejavuTextTrim::FurthestLineBreak,
            SpaceControlNode::SpaceControl4 => DejavuTextTrim::AllLineBreaks,
        }
    }
}

pub fn take_control_l(o: Option<SpaceControlNode>, statement: bool) -> DejavuTextTrim {
    match o {
        Some(SpaceControlNode::SpaceControl0) => DejavuTextTrim::Nothing,
        Some(SpaceControlNode::SpaceControl1) => DejavuTextTrim::UntilLineBreak,
        Some(SpaceControlNode::SpaceControl2) => DejavuTextTrim::RecentLineBreak,
        Some(SpaceControlNode::SpaceControl3) => DejavuTextTrim::FurthestLineBreak,
        Some(SpaceControlNode::SpaceControl4) => DejavuTextTrim::AllLineBreaks,
        None if statement => DejavuTextTrim::RecentLineBreak,
        None => DejavuTextTrim::Nothing,
    }
}

pub fn take_control_r(o: Option<SpaceControlNode>, statement: bool) -> DejavuTextTrim {
    match o {
        Some(SpaceControlNode::SpaceControl0) => DejavuTextTrim::Nothing,
        Some(SpaceControlNode::SpaceControl1) => DejavuTextTrim::UntilLineBreak,
        Some(SpaceControlNode::SpaceControl2) => DejavuTextTrim::RecentLineBreak,
        Some(SpaceControlNode::SpaceControl3) => DejavuTextTrim::FurthestLineBreak,
        Some(SpaceControlNode::SpaceControl4) => DejavuTextTrim::AllLineBreaks,
        None if statement => DejavuTextTrim::UntilLineBreak,
        None => DejavuTextTrim::Nothing,
    }
}
