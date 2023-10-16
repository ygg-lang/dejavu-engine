use crate::dejavu::TextElementNode;

impl TextElementNode {
    pub fn pure_space(&self) -> bool {
        matches!(self, Self::TextSpace { .. })
    }
    pub fn write_buffer(&self, w: &mut String) {
        match self {
            TextElementNode::TemplateE(_) => w.push_str("<%"),
            TextElementNode::TextSpace(s) => w.push_str(&s.text),
            TextElementNode::TextWord(s) => w.push_str(&s.text),
        }
    }
}
