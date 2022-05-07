use crate::Location;

#[derive(Debug, Copy, Clone)]
pub struct SahaError {
    kind: Box<SahaErrorKind>,
    span: Location,
}
pub struct Location {
    pub file: String,
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Location {
    fn from(value: Range<usize>) -> Self {
        Self {
            file: "".to_string(),
            start: value.start,
            end: value.end,
        }
    }
}

pub enum SahaErrorKind {}

pub type SahaResult<T> = Result<T, SahaError>;
