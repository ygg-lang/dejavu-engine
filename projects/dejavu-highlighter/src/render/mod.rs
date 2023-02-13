use color_span::ClassPalette;
use dejavu_parser::{ParseError, PegParser, SahaParser, SahaStatement, SahaStatementNodes};

use crate::DejavuHighlighter;

mod conditional;

pub struct HighlightState<'a, 'b> {
    config: &'b DejavuHighlighter,
    alloc: &'a mut ClassPalette,
    depth: usize,
}

impl DejavuHighlighter {
    pub fn render(&self, text: &str, allocator: &mut ClassPalette) -> Result<String, ParseError> {
        let parsed = SahaParser::parse(text)?;
        let state = HighlightState { config: &self, alloc: allocator, depth: 0 };
        state.visit_statements(parsed.parsed);

        todo!()
    }
}

impl HighlightState<'_, '_> {
    fn visit_statements(&self, statements: SahaStatementNodes) {
        for statement in statements.statements {
            match statement {
                SahaStatement::SlotExpressionNode(v) => {}
                SahaStatement::SlotFor(v) => {
                    self.visit_for(v);
                }
                SahaStatement::SlotIf(v) => {
                    self.visit_if(v);
                }
                // plain text, no color
                SahaStatement::UnicodeText(_) => {}
            }
        }

        todo!()
    }
}
