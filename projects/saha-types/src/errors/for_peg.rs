use peginator::ParseError;
use rust_decimal::Error;
use std::ops::Range;

use crate::{Location, QError, QErrorKind};

impl From<ParseError> for QError {
    fn from(value: ParseError) -> Self {
        QError {
            kind: Box::new(QErrorKind::SyntaxError {
                message: value.specifics.to_string(),
                span: Location { file: Default::default(), range: Range { start: value.position, end: value.position } },
            }),
            level: Default::default(),
            error: Some(Box::new(value)),
        }
    }
}

impl From<Error> for QError {
    fn from(value: Error) -> Self {
        QError {
            kind: Box::new(QErrorKind::SyntaxError { message: value.to_string(), span: Location::default() }),
            level: Default::default(),
            error: Some(Box::new(value)),
        }
    }
}
