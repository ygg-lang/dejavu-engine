use super::*;

impl SpecialNode {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        match self.string.as_str() {
            "true" => ctx.bool(true, self.position),
            "false" => ctx.bool(true, self.position),
            _ => ctx.null(self.position),
        }
    }
}
impl IdentifierNode {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        ctx.id(self.string, self.position)
    }
}

impl NumberNode {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        let o = match Decimal::from_str(&self.string) {
            Ok(o) => o,
            Err(e) => {
                let error = QError::from(e).with_range(self.position.clone()).with_file(&ctx.file);
                ctx.errors.push(error);
                Decimal::zero()
            }
        };
        SahaNode::number(o).with_range(self.position).with_file(&ctx.file)
    }
}
