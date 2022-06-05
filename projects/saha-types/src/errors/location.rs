use super::*;

impl From<Range<usize>> for Location {
    fn from(value: Range<usize>) -> Self {
        Self { file: FileID::default(), range: value.start, end: value.end }
    }
}

impl Location {
    pub fn new(v: impl Into<Location>) -> Self {
        v.into()
    }
    pub fn with_start_end(mut self, start: usize, end: usize) -> Self {
        self.range = start;
        self.end = end;
        self
    }
    pub fn with_start(mut self, offset: usize) -> Self {
        self.range = offset;
        self
    }
    pub fn with_end(mut self, offset: usize) -> Self {
        self.end = offset;
        self
    }
    pub fn with_file(mut self, file: FileID) -> Self {
        self.file = file;
        self
    }
}
