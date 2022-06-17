use dejavu_parser::{BooleanNode, NumberNode, SpecialNode, UnicodeText};

use super::*;

impl ParserContext {
    fn parse_value(&mut self, value: ValueNode) -> SahaNode {
        match value {
            ValueNode::IdentifierNode(v) => {}
            ValueNode::NumberNode(v) => {self.parse_integer(v)}
            ValueNode::BooleanNode(v) => self.parse_boolean(v),
        }
    }
    fn parse_boolean(&mut self, bool: BooleanNode) -> SahaNode {
        let value = match bool.string.as_str() {
            "true" => true,
            _ => false,
        };
        SahaNode::boolean(value).with_range(&bool.position).with_file(&self.file)
    }
    fn parse_decimal(&mut self, number: NumberNode) -> SahaNode {
        let node = match SahaNode::decimal(number.string.as_str()) {
            Ok(o) => o,
            Err(e) => {
                self.errors.push(e.with_file(&self.file).with_range(&number.position));
                SahaNode::from(0.0)
            }
        };
        node.with_range(&number.position).with_file(&self.file)
    }
    fn parse_integer(&mut self, number: NumberNode) -> SahaNode {
        let node = match SahaNode::integer(number.string.as_str()) {
            Ok(o) => o,
            Err(e) => {
                self.errors.push(e.with_file(&self.file).with_range(&number.position));
                SahaNode::from(0)
            }
        };
        node.with_range(&number.position).with_file(&self.file)
    }
}

impl UnicodeText {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        SahaNode::text(self.string).with_range(&self.position).with_file(&ctx.file)
    }
    pub fn id(&self, s: String, r: Range<usize>) -> SahaNode {
        SahaNode::identifier(s).with_range(&r).with_file(&self.file)
    }
    pub fn null(&self, r: Range<usize>) -> SahaNode {
        SahaNode::null().with_range(&r).with_file(&self.file)
    }
}

impl IdentifierNode {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        ctx.id(self.string, self.position)
    }
}
