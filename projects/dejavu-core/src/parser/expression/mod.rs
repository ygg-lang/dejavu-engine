use pratt::{Affix, Associativity, NoError, PrattParser, Precedence, Result};

use dejavu_parser::{ExpressionNodeInfix, TermNode};

use crate::{BinaryExpression, BinaryOperator};

use super::*;

impl ParserContext {
    pub fn parse_expression(&mut self, e: ExpressionNode) -> DjvNode {
        let mut ts = vec![];
        ts.push(TokenStream::Primary(e.head));
        for ExpressionNodeInfix { op, value } in e.infix {
            ts.push(TokenStream::Infix(op));
            ts.push(TokenStream::Primary(value));
        }
        ExprParser.parse(&mut ts).unwrap();
        DjvNode::null()
    }
    pub(super) fn parse_term(&mut self, value: TermNode) -> DjvNode {
        match value {
            ValueNode::IdentifierNode(v) => self.parse_identifier(v),
            ValueNode::NumberNode(v) => self.parse_integer(v),
            ValueNode::BooleanNode(v) => self.parse_boolean(v),
        }
    }
}

#[derive(Debug)]
pub enum TokenStream {
    Infix(String),
    Primary(TermNode),
    Group(Vec<TokenStream>),
}

struct ExprParser;

impl<I> PrattParser<I> for ExprParser
where
    I: Iterator<Item = TokenStream>,
{
    type Error = NoError;
    type Input = TokenStream;
    type Output = DjvNode;

    fn query(&mut self, tree: &TokenStream) -> Result<Affix> {
        let affix = match tree {
            TokenStream::Group(_) => Affix::Nilfix,
            TokenStream::Primary(_) => Affix::Nilfix,
            TokenStream::Infix(s) => match s.as_str() {
                "=" => Affix::Infix(Precedence(2), Associativity::Neither),
                "==" => Affix::Infix(Precedence(2), Associativity::Neither),
                "+" => Affix::Infix(Precedence(3), Associativity::Left),
                "-" => Affix::Infix(Precedence(3), Associativity::Left),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        Ok(affix)
    }

    fn primary(&mut self, tree: TokenStream) -> Result<DjvNode> {
        let expr = match tree {
            TokenStream::Primary(num) => DjvNode::Atom(num),
            TokenStream::Group(group) => self.parse(&mut group.into_iter()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
    }

    fn infix(&mut self, lhs: DjvNode, tree: TokenStream, rhs: DjvNode) -> Result<DjvNode> {
        let operator = match tree {
            TokenStream::Infix(infix) => match infix.as_str() {
                "+" => BinaryOperator::Addition,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        Ok(BinaryExpression { operator, left: lhs, right: rhs }.into())
    }

    fn prefix(&mut self, _tree: TokenStream, _rhs: DjvNode) -> Result<DjvNode> {
        unreachable!()
    }

    fn postfix(&mut self, _lhs: DjvNode, _tree: TokenStream) -> Result<DjvNode> {
        unreachable!()
    }
}
