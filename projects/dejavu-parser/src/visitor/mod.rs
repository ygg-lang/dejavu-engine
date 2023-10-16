use crate::dejavu::{ElementNode, RootNode, TextElementsNode};
use dejavu_ir::hir::{DejavuRoot, DejavuStatement, DejavuText};

impl RootNode {
    pub fn as_hir(&self) -> DejavuRoot {
        let mut out = DejavuRoot::default();
        for x in &self.element {
            match x {
                ElementNode::TemplateExport(_) => {}
                ElementNode::TemplateIf(_) => {}
                ElementNode::TextMany(v) => out.statements.push(DejavuStatement::Text(many_text(&v.text_elements))),
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
            TextElementsNode::TextSpace(s) => w.push_str(&s._text),
            TextElementsNode::TextWord(s) => w.push_str(&s._text),
        }
    }
}

fn many_text(texts: &[TextElementsNode]) -> DejavuText {
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
    out
}
