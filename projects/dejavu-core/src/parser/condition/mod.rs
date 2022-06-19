use dejavu_parser::{SlotFor, SlotIf};

use crate::{
    parser::ParserContext,
    value::{for_statement::ForStatement, DjvNode, SpaceDestroyer},
};

impl ParserContext {
    pub fn parse_if_slot(&mut self, s: SlotIf, outter: &mut Vec<DjvNode>) {
        let mut out = vec![];
        out.push(self.right_destroyer(&s.start.right, true));
        out.extend(self.parse_statements(s.body));
        out.push(self.left_destroyer(&s.end.left, true));
        todo!("if statement")
        // let stmt = IfStatement { body: SpaceDestroyer::clear(out) };
        // SahaNode::from(stmt).with_file(&self.file)
    }
}

impl ParserContext {
    pub fn parse_for_slot(&mut self, s: SlotFor, outter: &mut Vec<DjvNode>) -> DjvNode {
        let l = self.left_destroyer(&s.start.left, true);
        let r = self.right_destroyer(&s.end.right, true);
        outter.push(l);
        let mut inner = vec![];
        inner.push(self.right_destroyer(&s.start.right, true));
        inner.extend(self.parse_statements(s.body));
        inner.push(self.left_destroyer(&s.end.left, true));
        let stmt = ForStatement { body: SpaceDestroyer::clear(inner) };
        outter.push(r);
        DjvNode::from(stmt).with_file(&self.file)
    }
}
