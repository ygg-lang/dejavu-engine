use saha_types::SahaNode;

use crate::parser::{saha::SlotIf, ParserContext};

impl SlotIf {
    pub fn visit(self, ctx: &mut ParserContext, outter: &mut Vec<SahaNode>) {
        println!("{:#?}", self);
    }
}
