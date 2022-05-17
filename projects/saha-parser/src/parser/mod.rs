use std::ops::Range;

use peginator::PegParser;

use saha_types::{FileID, Location, SahaError, SahaNode, SahaResult, SahaValue};

use crate::parser::saha::{SahaStatement, SahaStatementNodes, SlotExpressionNode, SlotL, SlotR, SpecialNode, ValueNode};

use self::saha::SahaParser;

#[allow(unused, non_camel_case_types)]
mod saha;

#[derive(Default)]
pub struct ParserContext {
    file: FileID,
    errors: Vec<SahaError>,
}

impl ParserContext {
    pub fn text(&self, s: String, r: Range<usize>) -> SahaNode {
        SahaNode { kind: SahaValue::Text(s), span: Location { file: self.file.clone(), start: r.start, end: r.end } }
    }
    pub fn null(&self, r: Range<usize>) -> SahaNode {
        SahaNode { kind: SahaValue::Null, span: Location { file: self.file.clone(), start: r.start, end: r.end } }
    }
    pub fn bool(&self, v: bool, r: Range<usize>) -> SahaNode {
        SahaNode { kind: SahaValue::Boolean(v), span: Location { file: self.file.clone(), start: r.start, end: r.end } }
    }
}

pub fn parse() -> SahaResult {
    let mut ctx = ParserContext::default();
    let out = SahaParser::parse(include_str!("test.saha"))?;
    out.visit(&mut ctx);
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
                    destroy_left(&mut out, s.start.left);
                    let v = s.inner.visit(ctx);
                    println!("{:#?}", v);
                    println!("{:#?}", s.end);
                }
                SahaStatement::SlotExpressionNode(v) => v.visit(ctx),
            }
        }
        out
    }
}

impl SlotExpressionNode {
    pub fn visit(self, ctx: &mut ParserContext) {
        let out = self.value.visit(ctx);
        println!("{:#?}", out)
    }
}

impl ValueNode {
    pub fn visit(self, ctx: &mut ParserContext) {
        match self {
            ValueNode::IdentifierNode(_) => {}
            ValueNode::SpecialNode(v) => {
                println!("{:#?}", v.visit(ctx))
            }
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

pub fn destroy_left(list: &mut Vec<SahaNode>, mode: SlotL) -> Option<()> {
    let last = list.last_mut()?;

    None
}

pub fn destroy_right(list: &mut Vec<SahaNode>, mode: SlotR) -> Option<()> {
    let first = list.first()?;

    None
}
