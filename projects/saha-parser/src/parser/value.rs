use super::*;
use crate::parser::saha::{IdentifierNode, NumberNode};
use saha_types::Decimal;
use std::str::FromStr;

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
        match Decimal::from_str(&self.string) {
            Ok(_) => {}
            Err(_) => {}
        }
        SahaNode::null()
    }
}
