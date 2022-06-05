use std::{
    borrow::BorrowMut,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic_quick::{DiagnosticLevel, Failure, FileID, Success};
use serde::{Deserialize, Serialize};

mod location;

mod display;

#[cfg(feature = "peginator")]
mod for_peg;

pub type SahaResult<T = ()> = Result<T, SahaError>;
use std::cmp::{max, min};

pub struct SahaError {
    kind: Box<SahaErrorKind>,
    level: DiagnosticLevel,
    error: Option<Box<dyn Error>>,
}

pub type Validation<T> = diagnostic::Validation<T, SahaError>;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    pub file: FileID,
    pub range: Range<usize>,
}

impl Default for Location {
    fn default() -> Self {
        Self { file: Default::default(), range: Range { start: 0, end: 0 } }
    }
}

impl From<SahaErrorKind> for SahaError {
    fn from(value: SahaErrorKind) -> Self {
        Self { kind: Box::new(value), level: Default::default(), error: None }
    }
}

#[derive(Debug, Clone)]
pub enum SahaErrorKind {
    IoError { message: String, file: FileID },
    SyntaxError { message: String, span: Location },
    RuntimeError { message: String },
}

impl SahaError {
    pub fn with_range(self, range: Range<usize>) -> Self {
        self.with_span(range.start, range.end)
    }
    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        match self.kind.borrow_mut() {
            SahaErrorKind::IoError { .. } => {}
            SahaErrorKind::SyntaxError { span, .. } => {
                span.range.start = start;
                span.range.end = end;
            }
            SahaErrorKind::RuntimeError { .. } => {}
        }
        self
    }
    pub fn with_file(mut self, file: &FileID) -> Self {
        match self.kind.borrow_mut() {
            SahaErrorKind::IoError { file: old, .. } => *old = file.clone(),
            SahaErrorKind::SyntaxError { span, .. } => span.file = file.clone(),
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
