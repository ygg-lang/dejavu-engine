use crate::dejavu::{ElementNode, RootNode, SpaceControlNode, TextElementsNode};
use dejavu_ir::hir::{DejavuRoot, DejavuStatement, DejavuText, DejavuTextTrim};
use std::mem::take;

impl RootNode {
    pub fn as_hir(&self) -> DejavuRoot {
        let mut out = DejavuRoot::default();
        let mut last_trim = DejavuTextTrim::Nothing;
        for x in &self.element {
            match x {
                ElementNode::TemplateExport(_) => {}
                ElementNode::TemplateIf(v) => {
                    out.trim_last_text(take_control_l(v.if_begin.template_l.space_control.clone(), true));
                    last_trim = take_control_r(v.if_end.template_r.space_control.clone(), true)
                }
                ElementNode::TextMany(v) => out.statements.push(DejavuStatement::Text(many_text(&v.text_elements, last_trim))),
            }
        }
        out
    }
}

impl TextElementsNode {
    pub fn pure_space(&self) -> bool {
        matches!(self, Self::TextSpace { .. })
    }
    pub fn write_buffer(&self, w: &mut String) {
        match self {
            TextElementsNode::TemplateE(_) => w.push_str("<%"),
            TextElementsNode::TextSpace(s) => w.push_str(&s.text),
            TextElementsNode::TextWord(s) => w.push_str(&s.text),
        }
    }
}

fn many_text(texts: &[TextElementsNode], last_trim: DejavuTextTrim) -> DejavuText {
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
    out.trim_head(last_trim);
    out
}

fn take_control_l(o: Option<SpaceControlNode>, statement: bool) -> DejavuTextTrim {
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

fn take_control_r(o: Option<SpaceControlNode>, statement: bool) -> DejavuTextTrim {
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
