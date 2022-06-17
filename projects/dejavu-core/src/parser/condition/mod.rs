use dejavu_parser::SlotIf;

use crate::{parser::ParserContext, value::SahaNode};

impl ParserContext {
    pub fn if_statement(&mut self, s: SlotIf) -> SahaNode {
        let mut out = vec![];
        out.push(self.right_destroyer(&s.start.right, true));
        out.extend(s.body.visit(self));
        out.push(self.left_destroyer(&s.end.left, true));
        todo!("if statement")
        // let stmt = IfStatement { body: SpaceDestroyer::clear(out) };
        // SahaNode::from(stmt).with_file(&self.file)
    }
}
