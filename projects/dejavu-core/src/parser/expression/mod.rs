use super::*;

impl ParserContext {
    pub fn parse_expression(&mut self, e: ExpressionNode) -> DjvNode {
        self.parse_value(e.head)
    }
}
