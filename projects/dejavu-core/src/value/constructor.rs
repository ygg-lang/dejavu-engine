use std::str::FromStr;

use diagnostic_quick::{QError, QResult};

use super::*;

impl SahaNode {
    pub fn null() -> Self {
        Self { kind: ASTKind::Null, span: Default::default(), file: Default::default() }
    }
    pub fn boolean(v: bool) -> Self {
        Self { kind: ASTKind::Boolean(v), span: Default::default(), file: Default::default() }
    }
    pub fn get_boolean(&self) -> Option<&bool> {
        match &self.kind {
            ASTKind::Boolean(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_boolean(&mut self) -> Option<&mut bool> {
        match &mut self.kind {
            ASTKind::Boolean(s) => Some(s),
            _ => None,
        }
    }
    pub fn identifier(s: impl Into<String>) -> Self {
        Self { kind: ASTKind::Identifier(s.into()), span: Default::default(), file: Default::default() }
    }
    pub fn text(s: impl Into<String>) -> Self {
        Self { kind: ASTKind::Text(s.into()), span: Default::default(), file: Default::default() }
    }
    pub fn get_text(&self) -> Option<&String> {
        match &self.kind {
            ASTKind::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_text(&mut self) -> Option<&mut String> {
        match &mut self.kind {
            ASTKind::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn integer(number: &str) -> QResult<Self> {
        let int = i128::from_str(number)?;
        Ok(Self { kind: ASTKind::Integer(int), span: Default::default(), file: Default::default() })
    }
    pub fn decimal(number: &str) -> QResult<Self> {
        let fp = f64::from_str(number)?;
        if !fp.is_subnormal() { 
            Err(QError::syntax_error(format!("`{number}` is not a normal float number")))?
        }
        Ok(Self { kind: ASTKind::Decimal(fp), span: Default::default(), file: Default::default() })
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
