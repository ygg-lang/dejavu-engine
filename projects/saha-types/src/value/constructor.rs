use super::*;

impl SahaNode {
    pub fn null() -> Self {
        Self { kind: SahaValue::Null, span: Default::default() }
    }
    pub fn boolean(v: bool) -> Self {
        Self { kind: SahaValue::Boolean(v), span: Default::default() }
    }
    pub fn get_boolean(&self) -> Option<&bool> {
        match &self.kind {
            SahaValue::Boolean(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_boolean(&mut self) -> Option<&mut bool> {
        match &mut self.kind {
            SahaValue::Boolean(s) => Some(s),
            _ => None,
        }
    }
    pub fn identifier(s: impl Into<String>) -> Self {
        Self { kind: SahaValue::Identifier(Box::new(s.into())), span: Default::default() }
    }
    pub fn text(s: impl Into<String>) -> Self {
        Self { kind: SahaValue::Text(Box::new(s.into())), span: Default::default() }
    }
    pub fn get_text(&self) -> Option<&String> {
        match &self.kind {
            SahaValue::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_text(&mut self) -> Option<&mut String> {
        match &mut self.kind {
            SahaValue::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn get_number(&self) -> Option<&Decimal> {
        match &self.kind {
            SahaValue::Number(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_number(&mut self) -> Option<&mut Decimal> {
        match &mut self.kind {
            SahaValue::Number(s) => Some(s),
            _ => None,
        }
    }
    #[inline]
    pub fn with_span(mut self, span: Location) -> Self {
        self.span = span;
        self
    }
    #[inline]
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.span.start = range.start;
        self.span.end = range.end;
        self
    }
    #[inline]
    pub fn with_file(mut self, id: &FileID) -> Self {
        self.span.file = id.clone();
        self
    }
}
