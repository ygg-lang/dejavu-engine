use std::str::FromStr;

use diagnostic_quick::{QError, QResult};

use super::*;

impl DjvNode {
    pub fn null() -> Self {
        Self { kind: DjvKind::Null, span: Default::default(), file: Default::default() }
    }
    pub fn boolean(v: bool) -> Self {
        Self { kind: DjvKind::Boolean(v), span: Default::default(), file: Default::default() }
    }
    pub fn get_boolean(&self) -> Option<&bool> {
        match &self.kind {
            DjvKind::Boolean(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_boolean(&mut self) -> Option<&mut bool> {
        match &mut self.kind {
            DjvKind::Boolean(s) => Some(s),
            _ => None,
        }
    }
    pub fn text(s: impl Into<String>) -> Self {
        Self { kind: DjvKind::Text(s.into()), span: Default::default(), file: Default::default() }
    }
    pub fn get_text(&self) -> Option<&String> {
        match &self.kind {
            DjvKind::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_text(&mut self) -> Option<&mut String> {
        match &mut self.kind {
            DjvKind::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn integer(number: &str) -> QResult<Self> {
        let int = i128::from_str(number)?;
        Ok(Self { kind: DjvKind::Integer(int), span: Default::default(), file: Default::default() })
    }
    pub fn decimal(number: &str) -> QResult<Self> {
        let fp = f64::from_str(number)?;
        if !fp.is_subnormal() {
            Err(QError::syntax_error(format!("`{number}` is not a normal float number")))?
        }
        Ok(Self { kind: DjvKind::Decimal(fp), span: Default::default(), file: Default::default() })
    }
    #[inline]
    pub fn with_range(mut self, range: &Range<usize>) -> Self {
        self.span = range.clone();
        self
    }
    #[inline]
    pub fn with_file(mut self, id: &FileID) -> Self {
        self.file = id.clone();
        self
    }
}
