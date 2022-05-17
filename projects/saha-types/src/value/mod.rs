use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::FileID;

use crate::Location;

#[derive(Debug)]
pub struct SahaNode {
    pub kind: SahaValue,
    pub span: Location,
}

impl Display for SahaNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

#[derive(Debug)]
pub enum SahaValue {
    Null,
    Boolean(bool),
    Text(String),
    Identifier(Box<Identifier>),
    Vector,
}

impl SahaNode {
    pub fn mut_text(&mut self) -> Option<&mut String> {
        match &mut self.kind {
            SahaValue::Text(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

impl SahaNode {
    pub fn identifier(name: &str) -> Self {
        let kind = Identifier { name: name.to_string() };
        Self { kind: SahaValue::Identifier(Box::new(kind)), span: Default::default() }
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
