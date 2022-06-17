use dejavu_parser::UnicodeText;
use super::*;

impl UnicodeText {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        SahaNode::text(self.string).with_range(&self.position).with_file(&ctx.file)
    }
}

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
        let sci = self.string.replace("**", "e");
        let result = if sci.contains('e') { Decimal::from_scientific(&sci) } else { Decimal::from_str_exact(&sci) };
        let o = match result {
            Ok(o) => o,
            Err(e) => {
                let error = QError::from(e).with_range(&self.position).with_file(&ctx.file);
                ctx.errors.push(error);
                Decimal::zero()
            }
        };
        SahaNode::number(o).with_range(&self.position).with_file(&ctx.file)
    }
}
