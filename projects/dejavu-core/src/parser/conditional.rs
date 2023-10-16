use super::*;

impl<'i> From<&'i TemplateIfNode> for DejavuBranches {
    /// ```dejavu
    /// <% if %>
    ///    text
    /// <% else if %>
    ///    text
    /// <% else %>
    ///    text
    /// <% end %>
    /// ```
    fn from(value: &TemplateIfNode) -> Self {
        let mut out = DejavuBranches::default();
        let mut left = take_control_r(&value.if_begin.template_r, true);
        let mut right = DejavuTextTrim::Nothing;
        let mut cond = DejavuExpression::from(&value.if_begin.expression);
        let mut body = Default::default();

        for i in &value.if_else_if {
            right = take_control_l(&i.template_l, true);
            body = take_elements(&value.if_begin.element);
            body.trim_text(left, right);
            out += DejavuConditional { condition: take(&mut cond), body: take(&mut body.statements) };
            cond = (&i.expression).into();
            left = take_control_r(&i.template_r, true);
        }

        match &value.if_else {
            Some(otherwise) => {
                right = take_control_l(&otherwise.template_l, true);
                body = take_elements(&otherwise.element);
                body.trim_text(left, right);
                out += DejavuConditional { condition: take(&mut cond), body: take(&mut body.statements) };
            }
            None => {}
        }
        out
    }
}

impl<'i> From<&'i ExpressionNode> for DejavuExpression {
    fn from(value: &ExpressionNode) -> Self {
        Self {}
    }
}
