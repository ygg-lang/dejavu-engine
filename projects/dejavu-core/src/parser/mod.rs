use std::ops::Range;

use diagnostic_quick::{Failure, FileID, QError, QErrorKind, Success, Validation};
use peginator::PegParser;

use dejavu_parser::{SahaParser, SahaStatement, SahaStatementNodes, SlotExpressionNode, ValueNode};

use crate::value::{for_statement::ForStatement, SahaNode, SpaceDestroyer};

mod condition;
mod expression;
mod slots;
mod value;
mod whitespace;

pub struct ParserContext {
    file: FileID,
    errors: Vec<QError>,
}

impl ParserContext {
    pub fn custom_error(&mut self, message: impl Into<String>) {
        self.errors.push(QError { error: Box::new(QErrorKind::Custom(message)), level: Default::default(), source: None });
    }
}

pub fn parse(input: &str, file: &FileID) -> Validation<Vec<SahaNode>> {
    let mut ctx = ParserContext { file: file.clone(), errors: vec![] };
    if input.contains('\r') {
        ctx.errors.push(QError::syntax_error("CRLF"))
    }
    match SahaParser::parse(input) {
        Ok(s) => {
            let value = s.visit(&mut ctx);
            Success { value, diagnostics: ctx.errors }
        }
        Err(e) => Failure { fatal: QError::from(e).with_file(file), diagnostics: ctx.errors },
    }
}
