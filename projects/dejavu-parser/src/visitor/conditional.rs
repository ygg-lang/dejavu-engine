use super::*;
use crate::{
    dejavu::{ExpressionNode, IfElseNode},
    utils::take_control_l,
};
use dejavu_ir::hir::{DejavuConditional, DejavuExpression};
use std::mem::take;

impl TemplateIfNode {
    /// ```dejavu
    /// <% if %>
    ///    text
    /// <% else if %>
    ///    text
    /// <% else %>
    ///    text
    /// <% end %>
    /// ```
    pub(crate) fn as_hir(&self) -> DejavuStatement {
        let mut out = DejavuBranches::default();
        let mut left = take_control_r(&self.if_begin.template_r, true);
        let mut right = DejavuTextTrim::Nothing;
        let mut cond = self.if_begin.expression.as_hir();
        let mut body = Default::default();

        for i in &self.if_else_if {
            right = take_control_l(&i.template_l, true);
            body = take_elements(&self.if_begin.element);
            body.trim_text(left, right);
            out += DejavuConditional { condition: take(&mut cond), body: take(&mut body.statements) };
            cond = i.expression.as_hir();
            left = take_control_r(&i.template_r, true);
        }

        match &self.if_else {
            Some(otherwise) => {
                right = take_control_l(&otherwise.template_l, true);
                body = take_elements(&otherwise.element);
                body.trim_text(left, right);
                out += DejavuConditional { condition: take(&mut cond), body: take(&mut body.statements) };
            }
            None => {}
        }
        DejavuStatement::Branches(out)
    }
}

impl ExpressionNode {
    pub fn as_hir(&self) -> DejavuExpression {
        DejavuExpression {}
    }
}
