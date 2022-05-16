use peginator::PegParser;

use saha_types::{SahaError, SahaResult};

use crate::parser::saha::{SahaStatements, SlotExpressionNode, SlotL, SlotR};

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
                SahaStatements::UnicodeSpace(s) => {
                    println!("{}", s)
                }
                SahaStatements::UnicodeText(s) => {
                    println!("{}", s)
                }
                SahaStatements::SlotExpressionNode(v) => v.visit(ctx),
            }
        }
    }
}

impl SlotExpressionNode {
    pub fn visit(self, ctx: &mut ParserContext) {
        println!("{:#?}", self)
    }
}

impl SlotL {
    fn as_str(&self) -> &str {
        match self.trim {
            Some('_') => s,
            Some('-') => s,
            Some('=') => "",
            _ => s,
        }
    }
}

impl SlotR {
    fn as_str(&self) -> &str {
        let s = match &self.space {
            None => return "",
            Some(s) => s.as_str(),
        };
        match self.trim {
            Some('_') => s,
            Some('-') => s,
            Some('=') => "",
            _ => s,
        }
    }
}
