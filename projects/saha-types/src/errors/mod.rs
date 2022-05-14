use std::{
    borrow::BorrowMut,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;

mod location;

mod display;

pub type SahaResult<T> = Result<T, SahaError>;

pub struct SahaError {
    kind: Box<SahaErrorKind>,
    error: Option<Box<dyn Error>>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub file: FileID,
    pub start: usize,
    pub end: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { file: Default::default(), start: 0, end: 0 }
    }
}

impl From<SahaErrorKind> for SahaError {
    fn from(value: SahaErrorKind) -> Self {
        Self { kind: Box::new(value), error: None }
    }
}

#[derive(Debug, Clone)]
pub enum SahaErrorKind {
    IoError { message: String, file: FileID },
    SyntaxError { message: String, span: Location },
    RuntimeError { message: String },
}

impl SahaError {
    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        match self.kind.borrow_mut() {
            SahaErrorKind::IoError { .. } => {}
            SahaErrorKind::SyntaxError { span, .. } => {
                span.start = start;
                span.end = end;
            }
            SahaErrorKind::RuntimeError { .. } => {}
        }
        self
    }
    pub fn with_error<E>(mut self, e: E) -> Self
    where
        E: Error + 'static,
    {
        self.error = Some(Box::new(e));
        self
    }
}

impl Error for SahaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.error {
            Some(s) => Some(&**s),
            None => None,
        }
    }
}
