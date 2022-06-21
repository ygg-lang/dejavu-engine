use std::str::FromStr;

use diagnostic_quick::QResult;
use pratt::{Affix, PrattParser};

use dejavu_parser::{ExpressionNodeInfix, TermNode};

use crate::{BinaryExpression, BinaryOperator};

use super::*;

impl ParserContext {
    pub fn parse_expression(&mut self, e: ExpressionNode) -> DjvNode {
        let mut ts = vec![];
        ts.push(TokenStream::Primary(self.parse_term(e.head)));
        for ExpressionNodeInfix { op, value } in e.infix {
            ts.push(TokenStream::Infix(op));
            ts.push(TokenStream::Primary(self.parse_term(value)));
        }
        match ExprParser.parse(&mut ts.into_iter()) {
            Ok(o) => o,
            Err(error) => {
                println!("Error: {:?}", error);
                DjvNode::from("").with_range(&e.position).with_file(&self.file)
            }
        }
    }
    pub(super) fn parse_term(&mut self, value: TermNode) -> DjvNode {
        match value.term {
            ValueNode::IdentifierNode(v) => self.parse_identifier(v),
            ValueNode::NumberNode(v) => self.parse_integer(v),
            ValueNode::BooleanNode(v) => self.parse_boolean(v),
        }
    }
}

#[derive(Debug)]
pub enum TokenStream {
    Infix(String),
    Primary(DjvNode),
    Group(Vec<TokenStream>),
}

struct ExprParser;

impl<I> PrattParser<I> for ExprParser
where
    I: Iterator<Item = TokenStream>,
{
    type Error = QError;
    type Input = TokenStream;
    type Output = DjvNode;

    fn query(&mut self, tree: &TokenStream) -> QResult<Affix> {
        let affix = match tree {
            TokenStream::Group(_) => Affix::Nilfix,
            TokenStream::Primary(_) => Affix::Nilfix,
            TokenStream::Infix(s) => {
                let op = BinaryOperator::from_str(s)?;
                Affix::Infix(op.as_precedence(), op.as_associativity())
            }
        };
        Ok(affix)
    }

    fn primary(&mut self, tree: TokenStream) -> QResult<DjvNode> {
        let expr = match tree {
            TokenStream::Primary(node) => node,
            TokenStream::Group(group) => self.parse(&mut group.into_iter()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
    }

    fn infix(&mut self, lhs: DjvNode, tree: TokenStream, rhs: DjvNode) -> QResult<DjvNode> {
        let operator = match tree {
            TokenStream::Infix(infix) => match infix.as_str() {
                "+" => BinaryOperator::Addition,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        Ok(BinaryExpression { operator, left: lhs, right: rhs }.into())
    }

    fn prefix(&mut self, _tree: TokenStream, _rhs: DjvNode) -> QResult<DjvNode> {
        unreachable!()
    }

    fn postfix(&mut self, _lhs: DjvNode, _tree: TokenStream) -> QResult<DjvNode> {
        unreachable!()
    }
}
