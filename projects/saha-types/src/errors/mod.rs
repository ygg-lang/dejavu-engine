use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;

use crate::Location;

pub type SahaResult<T> = Result<T, SahaError>;

#[derive(Debug, Copy, Clone)]
pub struct SahaError {
    kind: Box<SahaErrorKind>,
    span: Location,
    error: Option<Box<dyn Error>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub file: FileID,
    pub start: usize,
    pub end: usize,
}

mod location;

impl Default for Location {
    fn default() -> Self {
        Self { file: Default::default(), start: 0, end: 0 }
    }
}

impl From<SahaErrorKind> for SahaError {
    fn from(value: SahaErrorKind) -> Self {
        Self { kind: Box::new(value), span: Default::default(), error: None }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SahaErrorKind {
    IoError { message: String },
}

impl SahaError {
    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.span.start = start;
        self.span.end = end;
        self
    }
    pub fn with_error<E: Error>(mut self, e: E) -> Self {
        self.error = Some(Box::new(e));
        self
    }
}

impl Display for SahaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
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
