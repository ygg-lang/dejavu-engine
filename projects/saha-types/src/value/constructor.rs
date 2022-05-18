use super::*;

impl SahaNode {
    pub fn null() -> Self {
        let kind = s.into();
        Self { kind: SahaValue::Null, span: Default::default() }
    }
    pub fn boolean(v: bool) -> Self {
        let kind = s.into();
        Self { kind: SahaValue::Boolean(v), span: Default::default() }
    }

    pub fn identifier(name: &str) -> Self {
        let kind = Identifier { name: name.to_string() };
        Self { kind: SahaValue::Identifier(Box::new(kind)), span: Default::default() }
    }
    pub fn text(s: impl Into<String>) -> Self {
        let kind = s.into();
        Self { kind: SahaValue::Text(Box::new(kind)), span: Default::default() }
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
    pub fn get_number(&self) -> Option<&Number> {
        match &self.kind {
            SahaValue::Number(s) => Some(s),
            _ => None,
        }
    }
    pub fn mut_number(&mut self) -> Option<&mut Number> {
        match &mut self.kind {
            SahaValue::Number(s) => Some(s),
            _ => None,
        }
    }
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.span.start = range.start;
        self.span.end = range.end;
        self
    }
    pub fn with_file(mut self, id: &FileID) -> Self {
        self.span.file = id.clone();
        self
    }
}
