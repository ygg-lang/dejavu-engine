use peginator::ParseError;
use rust_decimal::Error;

use crate::{Location, SahaError, SahaErrorKind};

impl From<ParseError> for SahaError {
    fn from(value: ParseError) -> Self {
        SahaError {
            kind: Box::new(SahaErrorKind::SyntaxError {
                message: value.specifics.to_string(),
                span: Location { file: Default::default(), range: value.position, end: value.position },
            }),
            level: Default::default(),
            error: Some(Box::new(value)),
        }
    }
}

impl From<Error> for SahaError {
    fn from(value: Error) -> Self {
        SahaError {
            kind: Box::new(SahaErrorKind::SyntaxError {
                message: value.to_string(),
                span: Location { file: Default::default(), range: 0, end: 0 },
            }),
            level: Default::default(),
            error: Some(Box::new(value)),
        }
    }
}
