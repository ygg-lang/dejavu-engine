use super::*;

impl ParserContext {
    pub(super) fn parse_value(&mut self, value: ValueNode) -> DjvNode {
        match value {
            ValueNode::IdentifierNode(v) => self.parse_identifier(v),
            ValueNode::NumberNode(v) => self.parse_integer(v),
            ValueNode::BooleanNode(v) => self.parse_boolean(v),
        }
    }
    pub(super) fn parse_boolean(&mut self, bool: BooleanNode) -> DjvNode {
        let value = matches!(bool.string.as_str(), "true");
        DjvNode::boolean(value).with_range(&bool.position).with_file(&self.file)
    }
    pub(super) fn parse_decimal(&mut self, number: NumberNode) -> DjvNode {
        let node = match DjvNode::decimal(number.string.as_str()) {
            Ok(o) => o,
            Err(e) => {
                self.errors.push(e.with_file(&self.file).with_range(&number.position));
                DjvNode::from(0.0)
            }
        };
        node.with_range(&number.position).with_file(&self.file)
    }
    pub(super) fn parse_integer(&mut self, number: NumberNode) -> DjvNode {
        let node = match DjvNode::integer(number.string.as_str()) {
            Ok(o) => o,
            Err(e) => {
                self.errors.push(e.with_file(&self.file).with_range(&number.position));
                DjvNode::from(0)
            }
        };
        node.with_range(&number.position).with_file(&self.file)
    }
    pub(super) fn parse_identifier(&mut self, identifier: IdentifierNode) -> DjvNode {
        DjvNode::identifier(identifier.string).with_range(&identifier.position).with_file(&self.file)
    }
    pub(super) fn parse_text(&mut self, text: UnicodeText) -> DjvNode {
        DjvNode::text(text.string).with_range(&text.position).with_file(&self.file)
    }
}
