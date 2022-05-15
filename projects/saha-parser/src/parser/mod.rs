use peginator::PegParser;

use crate::parser::saha::{SahaStatements, SlotExpressionNode};
use saha_types::{SahaError, SahaResult};

use self::saha::SahaParser;

#[allow(unused, non_camel_case_types)]
mod saha;

#[derive(Default)]
pub struct ParserContext {
    errors: Vec<SahaError>,
}

#[test]
fn test() -> SahaResult {
    let mut ctx = ParserContext::default();
    let out = SahaParser::parse(include_str!("test.saha"))?;
    out.visit(&mut ctx);
    Ok(())
}

impl SahaParser {
    pub fn visit(self, ctx: &mut ParserContext) {
        for statement in self.statements {
            match statement {
                SahaStatements::EndNode(v) => {
                    println!("{:#?}", v)
                }
                SahaStatements::SlotExpressionNode(v) => v.visit(ctx),
            }
        }
    }
}

impl SlotExpressionNode {
    pub fn visit(self, ctx: &mut ParserContext) {
        println!("{:?}", self.left.trim);
        println!("{:#?}", self)
    }
}
