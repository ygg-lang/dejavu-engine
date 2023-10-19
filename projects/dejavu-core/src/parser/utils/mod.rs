use super::*;

pub fn take_elements(s: &[ElementNode]) -> DejavuSequence {
    let mut out = DejavuSequence::default();
    for x in s {
        match x {
            ElementNode::Export(_) => {}
            ElementNode::For(_) => {}
            ElementNode::If(v) => {
                // out.trim_text(take_control_l(v.if_begin.template_l.space_control.clone(), true));
                // last_trim = take_control_r(v.if_end.template_r.space_control.clone(), true);
                out += DejavuStatement::Branches(v.into());
            }
            ElementNode::Text(v) => out += take_text(&v.text_element),
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
            if tail.pure_space() {
                //
                tail.write_buffer(&mut out.tail)
            }
            else {
                tail.write_buffer(&mut out.body)
            }
            let start = head.get_range().unwrap_or_default().start;
            let end = tail.get_range().unwrap_or_default().end;
            out.range = start..end;
            DejavuStatement::Text(out)
        }
        _ => panic!("many_text: {}", texts.len()),
    }
}

impl From<SpaceControlNode> for DejavuTextTrim {
    fn from(value: SpaceControlNode) -> Self {
        match value {
            SpaceControlNode::Nothing => DejavuTextTrim::Nothing,
            SpaceControlNode::Break0 => DejavuTextTrim::UntilLineBreak,
            SpaceControlNode::Break1 => DejavuTextTrim::RecentLineBreak,
            SpaceControlNode::Delete0 => DejavuTextTrim::FurthestLineBreak,
            SpaceControlNode::Delete1 => DejavuTextTrim::FurthestLineBreak,
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
