use dejavu_parser::{DecimalNode, IntegerNode, NamespaceNode};

use super::*;

impl ParserContext {
    pub(super) fn parse_value(&mut self, value: ValueNode) -> DjvNode {
        match value {
            ValueNode::DecimalNode(v) => self.parse_decimal(v),
            ValueNode::IntegerNode(v) => self.parse_integer(v),
            ValueNode::NamespaceNode(v) => self.parse_namespace(v),
        }
    }
    pub(super) fn parse_decimal(&mut self, number: DecimalNode) -> DjvNode {
        let node = match DjvNode::decimal(number.string.as_str()) {
            Ok(o) => o,
            Err(e) => {
                self.errors.push(e.with_file(&self.file).with_range(&number.position));
                DjvNode::from(0.0)
            }
        };
        node.with_range(&number.position).with_file(&self.file)
    }
    pub(super) fn parse_integer(&mut self, number: IntegerNode) -> DjvNode {
        let node = match DjvNode::integer(number.string.as_str()) {
            Ok(o) => o,
            Err(e) => {
                self.errors.push(e.with_file(&self.file).with_range(&number.position));
                DjvNode::from(0)
            }
        };
        node.with_range(&number.position).with_file(&self.file)
    }
    pub(super) fn parse_namespace(&mut self, namespace: NamespaceNode) -> DjvNode {
        if namespace.path.len() == 1 {
            return unsafe { self.parse_maybe_special(&namespace.path.get_unchecked(0).string) };
        }
        let names = namespace.path.into_iter().map(|s| s.string).collect();
        DjvNode::identifier(names).with_range(&namespace.position).with_file(&self.file)
    }
    pub(super) fn parse_maybe_special(&mut self, identifier: &str) -> DjvNode {
        match identifier {
            "true" => DjvNode::from(true),
            "false" => DjvNode::from(false),
            _ => DjvNode::identifier(vec![identifier.to_string()]),
        }
    }

    pub(super) fn parse_text(&mut self, text: UnicodeText) -> DjvNode {
        DjvNode::text(text.string).with_range(&text.position).with_file(&self.file)
    }
}
