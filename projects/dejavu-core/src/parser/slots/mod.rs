use super::*;

impl ParserContext {
    #[inline]
    pub(super) fn parse_root(&mut self, root: SahaParser) -> Vec<DjvNode> {
        SpaceDestroyer::clear(self.parse_statements(root.parsed))
    }
    pub(super) fn parse_statements(&mut self, nodes: SahaStatementNodes) -> Vec<DjvNode> {
        let mut out = vec![];
        for statement in nodes.statements {
            match statement {
                SahaStatement::UnicodeText(s) => out.push(self.parse_text(s)),
                SahaStatement::SlotFor(s) => {
                    self.parse_for_slot(s, &mut out);
                }
                SahaStatement::Comment(s) => {
                    out.push(self.left_destroyer(&s.left, false));
                    out.push(self.right_destroyer(&s.right, false));
                }
                SahaStatement::SlotExpressionNode(s) => self.parse_slot(s, &mut out),
                SahaStatement::SlotIf(s) => self.parse_if_slot(s, &mut out),
            }
        }
        // Don't break white space, prevent redundant breaks
        out
    }
    pub(super) fn parse_slot(&mut self, slot: SlotExpressionNode, out: &mut Vec<DjvNode>) {
        let l = self.left_destroyer(&slot.left, true);
        let r = self.right_destroyer(&slot.right, true);
        out.push(l);
        out.push(self.parse_expression(slot.e));
        out.push(r);
    }
}
