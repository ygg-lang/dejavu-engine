use color_span::ClassPalette;
use dejavu_parser::{ParseError, PegParser, SahaParser, SahaStatement, SahaStatementNodes};

use crate::DejavuHighlighter;

mod conditional;

pub struct HighlightState<'p> {
    pub alloc: &'p mut ClassPalette,
    pub depth: usize,
}

impl DejavuHighlighter {
    pub fn render<'i>(&'p mut self, text: &'i str, allocator: &mut ClassPalette) -> Result<String, ParseError> {
        let parsed = SahaParser::parse(text)?;
        let state = HighlightState { alloc: self.palette, depth: 0 };
        state.visit_statements(parsed.parsed);

        todo!()
    }
}

impl HighlightState<'_> {
    fn visit_statements(&self, statements: SahaStatementNodes) {
        for statement in statements.statements {
            match statement {
                SahaStatement::SlotExpressionNode(v) => {}
                SahaStatement::SlotFor(v) => {}
                SahaStatement::SlotIf(v) => {}
                // plain text, no color
                SahaStatement::UnicodeText(_) => {}
            }
        }
        todo!()
    }
}
