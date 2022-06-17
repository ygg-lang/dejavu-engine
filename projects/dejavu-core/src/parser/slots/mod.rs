use super::*;


impl SahaParser {
    pub fn visit(self, ctx: &mut ParserContext) -> Vec<SahaNode> {
        SpaceDestroyer::clear(self.parsed.visit(ctx))
    }
}

impl SahaStatementNodes {
    pub fn visit(self, ctx: &mut ParserContext) -> Vec<SahaNode> {
        let mut out = vec![];
        for statement in self.statements {
            match statement {
                SahaStatement::UnicodeText(s) => out.push(s.visit(ctx)),
                SahaStatement::SlotFor(s) => {
                    let l = ctx.left_destroyer(&s.start.left, true);
                    let r = ctx.right_destroyer(&s.end.right, true);
                    out.push(l);
                    out.push(ctx.for_statement(s));
                    out.push(r);
                }
                SahaStatement::Comment(s) => {
                    out.push(ctx.left_destroyer(&s.left, false));
                    out.push(ctx.right_destroyer(&s.right, false));
                }
                SahaStatement::SlotExpressionNode(s) => s.visit(ctx, &mut out),
                SahaStatement::SlotIf(s) => s.visit(ctx, &mut out),
            }
        }
        // Don't break white space, prevent redundant breaks
        out
    }
}

impl ParserContext {
    pub fn for_statement(&mut self, s: SlotFor) -> SahaNode {
        let mut out = vec![];
        out.push(self.right_destroyer(&s.start.right, true));
        out.extend(s.body.visit(self));
        out.push(self.left_destroyer(&s.end.left, true));
        let stmt = ForStatement { body: SpaceDestroyer::clear(out) };
        SahaNode::from(stmt).with_file(&self.file)
    }
}

impl SlotExpressionNode {
    pub fn visit(self, ctx: &mut ParserContext, out: &mut Vec<SahaNode>) {
        let l = ctx.left_destroyer(&self.left, false);
        let r = ctx.right_destroyer(&self.right, false);
        out.push(l);
        // out.push(self.value.visit(ctx));
        out.push(r);
    }
}

