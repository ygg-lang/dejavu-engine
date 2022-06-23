use dejavu_parser::{SlotFor, SlotIf};

use crate::{parser::ParserContext, DjvNode, ForStatement, IfStatement, SpaceDestroyer};

impl ParserContext {
    pub fn parse_if_slot(&mut self, s: SlotIf, outter: &mut Vec<DjvNode>) {
        let mut body = vec![];
        outter.push(self.left_destroyer(&s.start.left, true));
        body.push(self.right_destroyer(&s.start.right, true));
        body.extend(self.parse_statements(s.body));
        body.push(self.left_destroyer(&s.end.left, true));

        outter.push(IfStatement::new(body, &s.position, &self.file));

        outter.push(self.right_destroyer(&s.end.right, true));
    }
}

impl ParserContext {
    pub fn parse_for_slot(&mut self, s: SlotFor, outter: &mut Vec<DjvNode>) {
        let l = self.left_destroyer(&s.start.left, true);
        let r = self.right_destroyer(&s.end.right, true);
        outter.push(l);
        let mut inner = vec![];
        inner.push(self.right_destroyer(&s.start.right, true));
        inner.extend(self.parse_statements(s.body));
        inner.push(self.left_destroyer(&s.end.left, true));
        // let stmt = ForStatement { body: SpaceDestroyer::clear(inner) };
        outter.push(r);
    }
}
