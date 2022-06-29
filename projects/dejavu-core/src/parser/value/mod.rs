use dejavu_parser::{BooleanNode, DecimalNode, IntegerNode, NamespaceNode, StringItem, StringNode};

use crate::{
    value::atomic::{DjvString, StringKind},
    Identifier, Namespace,
};

use super::*;

impl ParserContext {
    pub(super) fn parse_value(&mut self, value: ValueNode) -> DjvNode {
        match value {
            ValueNode::BooleanNode(v) => self.parse_boolean(v),
            ValueNode::DecimalNode(v) => self.parse_decimal(v),
            ValueNode::IntegerNode(v) => self.parse_integer(v),
            ValueNode::StringNode(v) => self.parse_string(v),
            ValueNode::NamespaceNode(v) => self.parse_namespace(v),
        }
    }
    pub fn parse_boolean(&mut self, node: BooleanNode) -> DjvNode {
        let value = match node.string.as_str() {
            "true" => DjvNode::boolean(true),
            _ => DjvNode::boolean(false),
        };
        value.with_range(&node.position).with_file(&self.file)
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
        let names = namespace.path.into_iter().map(|s| self.parse_identifier(s)).collect();
        Namespace::new(names, &namespace.position, &self.file)
    }
    pub(super) fn parse_identifier(&mut self, identifier: IdentifierNode) -> Identifier {
        Identifier { name: identifier.string, span: identifier.position }
    }
    pub(super) fn parse_text(&mut self, text: UnicodeText) -> DjvNode {
        DjvNode::text(text.string).with_range(&text.position).with_file(&self.file)
    }
}

impl ParserContext {
    pub fn parse_string(&mut self, string: StringNode) -> DjvNode {
        let mut chars = String::new();
        for item in string.body {
            self.parse_string_item(item, &mut chars);
        }
        let quote = match string.sq {
            None => StringKind::DoubleQuote,
            Some(_) => StringKind::SingleQuote,
        };
        DjvString { kind: quote, value: chars }.to_node(&string.position, &self.file)
    }
    fn parse_string_item(&mut self, item: StringItem, s: &mut String) {
        match item {
            StringItem::EscapeOther(c) => match c {
                't' => s.push('\t'),
                'r' => s.push('\r'),
                'n' => s.push('\n'),
                _ => s.push(c),
            },
            StringItem::EscapeUnicode(_) => {
                todo!()
            }
            StringItem::char(c) => {
                s.push(c);
            }
        }
    }
}
