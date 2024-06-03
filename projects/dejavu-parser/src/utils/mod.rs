use crate::dejavu::{ElementNode, ExpressionNode, TemplateIfNode, TemplateLNode, TemplateRNode, TextElementNode};

impl TextElementNode {
    pub fn pure_space(&self) -> bool {
        matches!(self, Self::TextSpace { .. })
    }
    pub fn write_buffer(&self, w: &mut String) {
        match self {
            TextElementNode::Escape(_) => w.push_str("{%"),
            TextElementNode::TextSpace(s) => w.push_str(&s.text),
            TextElementNode::TextWord(s) => w.push_str(&s.text),
        }
    }
}

/// ```dejavu
/// {% if %}
///    then
/// {% else if %}
///    text
/// {% else %}
///    text
/// {% end %}
/// ```
impl TemplateIfNode {
    pub fn rights(&self) -> Vec<&TemplateRNode> {
        let mut out = Vec::with_capacity(self.if_else_if.len() + 1);
        out.push(&self.if_begin.template_r);
        for term in &self.if_else_if {
            out.push(&term.template_r)
        }
        match &self.if_else {
            Some(s) => out.push(&s.template_r),
            None => {}
        }
        out
    }
    pub fn lefts(&self) -> Vec<&TemplateLNode> {
        let mut out = Vec::with_capacity(self.if_else_if.len() + 1);
        for term in &self.if_else_if {
            out.push(&term.template_l)
        }
        match &self.if_else {
            Some(s) => out.push(&s.template_l),
            None => {}
        }
        out.push(&self.if_end.template_l);
        out
    }
    pub fn conditions(&self) -> Vec<&ExpressionNode> {
        let mut out = Vec::with_capacity(self.if_else_if.len() + 1);
        out.push(&self.if_begin.expression);
        for term in &self.if_else_if {
            out.push(&term.expression)
        }
        out
    }
    pub fn bodies(&self) -> Vec<&[ElementNode]> {
        let mut out = Vec::with_capacity(self.if_else_if.len() + 1);
        out.push(self.if_begin.element.as_slice());
        for term in &self.if_else_if {
            out.push(term.element.as_slice())
        }
        match &self.if_else {
            Some(s) => out.push(s.element.as_slice()),
            None => {}
        }
        out
    }
}
