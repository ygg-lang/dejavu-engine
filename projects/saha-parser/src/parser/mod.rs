use std::{ops::Range, str::FromStr};

use peginator::PegParser;

use saha_types::{
    Decimal, Failure, FileID, ForStatement, SahaError, SahaNode, SahaValue, SpaceDestroyer, Success, Validation, Zero,
};

use crate::parser::saha::{
    CommentL, CommentR, IdentifierNode, NumberNode, SahaStatement, SahaStatementNodes, SlotExpressionNode, SlotFor, SlotL,
    SlotR, SpecialNode, ValueNode,
};

use self::saha::SahaParser;

mod value;
mod whitespace;

#[allow(unused, non_camel_case_types)]
mod saha;

#[derive(Default)]
pub struct ParserContext {
    file: FileID,
    errors: Vec<SahaError>,
}

impl ParserContext {
    pub fn id(&self, s: String, r: Range<usize>) -> SahaNode {
        SahaNode::identifier(s).with_range(r).with_file(&self.file)
    }
    pub fn text(&self, s: String, r: Range<usize>) -> SahaNode {
        SahaNode::text(s).with_range(r).with_file(&self.file)
    }
    pub fn null(&self, r: Range<usize>) -> SahaNode {
        SahaNode::null().with_range(r).with_file(&self.file)
    }
    pub fn bool(&self, v: bool, r: Range<usize>) -> SahaNode {
        SahaNode::boolean(v).with_range(r).with_file(&self.file)
    }
}

pub fn parse(input: &str) -> Validation<Vec<SahaNode>> {
    let mut ctx = ParserContext::default();
    let lf = input.replace("\r\n", "\n");
    match SahaParser::parse(&lf) {
        Ok(s) => {
            let value = s.visit(&mut ctx);
            Success { value, diagnostics: ctx.errors }
        }
        Err(e) => Failure { fatal: SahaError::from(e), diagnostics: vec![] },
    }
}

impl SahaParser {
    pub fn visit(self, ctx: &mut ParserContext) -> Vec<SahaNode> {
        SpaceDestroyer::clear(self.parsed.visit(ctx))
    }
}

impl SahaStatementNodes {
    pub fn visit(self, ctx: &mut ParserContext) -> Vec<SahaNode> {
        let mut out = vec![];
        for statement in self.statements {
            match statement {
                SahaStatement::UnicodeText(s) => out.push(ctx.text(s.string, s.position)),
                SahaStatement::SlotFor(s) => {
                    let l = ctx.left_destroyer(&s.start.left, true);
                    let r = ctx.right_destroyer(&s.end.right, true);
                    out.push(l);
                    out.push(ctx.for_statement(s));
                    out.push(r);
                }
                SahaStatement::SlotExpressionNode(SlotExpressionNode { left, value, right }) => {
                    out.push(ctx.left_destroyer(&left, false));
                    out.push(value.visit(ctx));
                    out.push(ctx.right_destroyer(&right, false));
                }
                SahaStatement::Comment(s) => {
                    out.push(ctx.left_destroyer(&s.left, false));
                    out.push(ctx.right_destroyer(&s.right, false));
                }
                SahaStatement::SlotIf(_) => todo!(),
            }
        }
        // Don't break white space, prevent redundant breaks
        out
    }
}

impl ParserContext {
    pub fn for_statement(&mut self, s: SlotFor) -> SahaNode {
        let mut out = vec![];
        out.push(self.right_destroyer(&s.start.right, true));
        out.extend(s.body.visit(self));
        out.push(self.left_destroyer(&s.end.left, true));
        let stmt = ForStatement { body: SpaceDestroyer::clear(out) };
        SahaNode::from(stmt).with_file(&self.file)
    }
}

impl SlotExpressionNode {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        self.value.visit(ctx)
    }
}

impl ValueNode {
    pub fn visit(self, ctx: &mut ParserContext) -> SahaNode {
        match self {
            ValueNode::IdentifierNode(v) => v.visit(ctx),
            ValueNode::SpecialNode(v) => v.visit(ctx),
            ValueNode::NumberNode(v) => v.visit(ctx),
        }
    }
}
