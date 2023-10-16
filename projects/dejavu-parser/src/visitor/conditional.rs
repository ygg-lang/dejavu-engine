use super::*;
use crate::utils::take_control_l;
use dejavu_ir::hir::DejavuConditional;

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
        let mut trim = (DejavuTextTrim::Nothing, DejavuTextTrim::Nothing);

        let mut body = take_elements(&self.if_begin.element);
        body.trim_text(trim.0, trim.1);

        out += DejavuConditional { condition: Default::default(), body };

        DejavuStatement::Branches(out)
    }
}
