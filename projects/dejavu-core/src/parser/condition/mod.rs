use crate::{
    parser::{saha::SlotIf, ParserContext},
    value::SahaNode,
};

impl SlotIf {
    pub fn visit(self, ctx: &mut ParserContext, outter: &mut Vec<SahaNode>) {
        println!("{:#?}", self);
    }
}
