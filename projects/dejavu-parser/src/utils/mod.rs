use crate::dejavu::{ElementNode, ExpressionNode, IfBlockNode, TemplateLNode, TemplateRNode, TextElementNode};
use yggdrasil_rt::YggdrasilNode;

impl<'i> TextElementNode<'i> {
    pub fn pure_space(&self) -> bool {
        matches!(self, Self::TextSpace { .. })
    }
    pub fn write_buffer(&self, w: &mut String) {
        match self {
            TextElementNode::TextSpace(s) => w.push_str(s.get_str()),
            TextElementNode::TextWord(s) => w.push_str(s.get_str()),
        }
    }
}

/// ```dejavu
/// <% if %>
///    then
/// <% else if %>
///    text
/// <% else %>
///    text
/// <% end %>
/// ```
impl<'i> IfBlockNode<'i> {
    pub fn rights(&self) -> Vec<TemplateRNode> {
        let mut out = Vec::with_capacity(self.template_else_if().len() + 1);
        out.push(self.template_if().template_r());
        for term in &self.template_else_if() {
            out.push(term.template_r())
        }
        match &self.template_else() {
            Some(s) => out.push(s.template_r()),
            None => {}
        }
        out
    }
    pub fn lefts(&self) -> Vec<TemplateLNode> {
        let mut out = Vec::with_capacity(self.template_else_if().len() + 1);
        for term in &self.template_else_if() {
            out.push(term.template_l())
        }
        match &self.template_else() {
            Some(s) => out.push(s.template_l()),
            None => {}
        }
        out.push(self.template_end().template_l());
        out
    }
    pub fn conditions(&self) -> Vec<ExpressionNode> {
        let mut out = Vec::with_capacity(self.template_else_if().len() + 1);
        out.push(self.template_if().expression());
        for term in self.template_else_if() {
            out.push(term.expression())
        }
        out
    }
    pub fn bodies(&self) -> Vec<Vec<ElementNode>> {
        let mut out = Vec::with_capacity(self.template_else_if().len() + 1);
        out.push(self.template_if().element());
        for term in &self.template_else_if() {
            out.push(term.element())
        }
        match &self.template_else() {
            Some(s) => out.push(s.element()),
            None => {}
        }
        out
    }
}
