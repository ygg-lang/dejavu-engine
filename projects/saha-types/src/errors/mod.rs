use std::ops::Range;
use diagnostic::FileID;

use crate::Location;

#[derive(Debug, Copy, Clone)]
pub struct SahaError {
    kind: Box<SahaErrorKind>,
    span: Location,
}

pub struct Location {
    pub file: FileID,
    pub start: usize,
    pub end: usize,
}

mod location;


pub enum SahaErrorKind {}

pub type SahaResult<T> = Result<T, SahaError>;
