use std::ops::Range;

use peginator::PegParser;

use saha_types::{FileID, ForStatement, SahaError, SahaNode, SahaResult, SahaValue, SpaceDestroyer};

use crate::parser::saha::{
    CommentL, SahaStatement, SahaStatementNodes, SlotExpressionNode, SlotFor, SlotL, SlotR, SpecialNode, ValueNode,
};

use self::saha::SahaParser;

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

pub fn parse() -> SahaResult {
    let mut ctx = ParserContext::default();
    let out = SahaParser::parse(&include_str!("test.saha").replace("\r\n", "\n"))?;
    let v = out.visit(&mut ctx);
    println!("{:#?}", v);
    Ok(())
}

impl SahaParser {
    pub fn visit(self, ctx: &mut ParserContext) -> Vec<SahaNode> {
        self.parsed.visit(ctx)
    }
}

impl SahaStatementNodes {
    pub fn visit(self, ctx: &mut ParserContext) -> Vec<SahaNode> {
        let mut out = vec![];
        for statement in self.statements {
            match statement {
                SahaStatement::UnicodeText(s) => out.push(ctx.text(s, Range::default())),
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
                    out.push(ctx.left_destroyer(&s.left.trim, false));
                    out.push(value.visit(ctx));
                    out.push(ctx.right_destroyer(&right, false));
                }
            }
        }
        SpaceDestroyer::clear(out)
    }
}

impl ParserContext {
    pub fn for_statement(&mut self, s: SlotFor) -> SahaNode {
        let mut out = vec![];
        out.push(self.right_destroyer(&s.start.right, true));
        out.extend(s.inner.visit(self));
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
            ValueNode::IdentifierNode(v) => ctx.id(v.string, v.position),
            ValueNode::SpecialNode(v) => v.visit(ctx),
        }
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

impl ParserContext {
    pub fn left_destroyer(&self, mode: impl Into<SlotL>, statement: bool) -> SahaNode {
        SahaNode { kind: SahaValue::LeftDestroyer(SpaceDestroyer::new(mode.into().trim, statement)), span: Default::default() }
    }
    pub fn right_destroyer(&self, mode: impl Into<SlotR>, statement: bool) -> SahaNode {
        SahaNode { kind: SahaValue::RightDestroyer(SpaceDestroyer::new(mode.into().trim, statement)), span: Default::default() }
    }
}

impl From<&SlotL> for SlotL {
    fn from(value: &SlotL) -> Self {
        value.into()
    }
}

impl From<&CommentL> for SlotL {
    fn from(value: &CommentL) -> Self {
        SlotL { trim: value.trim }
    }
}

impl From<&SlotR> for SlotR {
    fn from(value: &SlotR) -> Self {
        value.into()
    }
}
