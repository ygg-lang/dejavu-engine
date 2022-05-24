use std::ops::Range;

use peginator::PegParser;

use saha_types::{FileID, SahaError, SahaNode, SahaResult};

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
                    let mut inner = s.inner.visit(ctx);
                    destroy_left(&mut out, &s.start.left);
                    destroy_right(&mut inner, &s.start.right);
                    destroy_left(&mut inner, &s.end.left);
                    destroy_right(&mut out, &s.end.right);
                }
                SahaStatement::SlotExpressionNode(slot) => {
                    destroy_left(&mut out, &slot.left);
                    out.push(slot.visit(ctx));
                    // destroy_right(&mut out, slot.right);
                }
            }
        }
        out
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

pub fn destroy_left(list: &mut [SahaNode], mode: &SlotL) -> Option<()> {
    let last = list.last_mut()?.mut_text()?;
    match mode.trim {
        Some('=') => *last = last.trim_end().to_string(),
        Some('-') => {}
        Some('_') => {}
        _ => {}
    }

    None
}

pub fn destroy_right(list: &mut [SahaNode], mode: &SlotR) -> Option<()> {
    let first = list.first_mut()?.mut_text()?;
    match mode.trim {
        Some('=') => *first = first.trim_start().to_string(),
        Some('-') => {}
        Some('_') => {}
        _ => {}
    }
    None
}
