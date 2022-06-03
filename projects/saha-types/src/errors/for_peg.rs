use peginator::ParseError;
use rust_decimal::Error;

use crate::{Location, SahaError, SahaErrorKind};

impl From<ParseError> for SahaError {
    fn from(value: ParseError) -> Self {
        SahaError {
            kind: Box::new(SahaErrorKind::SyntaxError {
                message: value.specifics.to_string(),
                span: Location { file: Default::default(), start: value.position, end: value.position },
            }),
            error: Some(Box::new(value)),
        }
    }
}

impl From<Error> for SahaError {
    fn from(value: Error) -> Self {
        SahaError {
            kind: Box::new(SahaErrorKind::SyntaxError {
                message: value.to_string(),
                span: Location { file: Default::default(), start: 0, end: 0 },
            }),
            error: Some(Box::new(value)),
        }
    }
}
