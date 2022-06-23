use dejavu_parser::{SlotFor, SlotIf};

use crate::{parser::ParserContext, value::for_statement::IfConditional, DjvNode, IfStatement, SpaceDestroyer};

impl ParserContext {
    pub fn parse_if_slot(&mut self, s: SlotIf, outter: &mut Vec<DjvNode>) {
        let mut body = vec![];
        outter.push(self.left_destroyer(&s.start.left, true));
        body.push(self.right_destroyer(&s.start.right, true));
        body.extend(self.parse_statements(s.body));
        let condition = self.parse_expression(s.start.cond);
        match s.r#else {
            // if _ { } else { }
            Some(else_statement) if s.else_if.is_empty() => {
                let mut backpack = vec![];
                body.push(self.left_destroyer(else_statement.left, true));
                backpack.push(self.right_destroyer(else_statement.right, true));
                backpack.extend(self.parse_statements(else_statement.body));
                backpack.push(self.left_destroyer(&s.end.left, true));
                outter.push(IfStatement::new(
                    vec![IfConditional { condition, body: SpaceDestroyer::clear(body) }],
                    SpaceDestroyer::clear(backpack),
                    &s.position,
                    &self.file,
                ));
            }
            // if _ { } else if _ { } else { }
            Some(else_statement) => {
                let _ = else_statement.body;
                todo!()
            }
            // if _ { }
            None if s.else_if.is_empty() => {
                body.push(self.left_destroyer(&s.end.left, true));
                outter.push(IfStatement::new(
                    vec![IfConditional { condition, body: SpaceDestroyer::clear(body) }],
                    vec![],
                    &s.position,
                    &self.file,
                ));
            }
            // if _ {} else if _ { }0
            None => {
                todo!()
            }
        }
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
