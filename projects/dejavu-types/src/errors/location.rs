use super::*;

impl From<Range<usize>> for Location {
    fn from(value: Range<usize>) -> Self {
        Self { file: FileID::default(), range: value }
    }
}

impl Location {
    pub fn new(v: impl Into<Location>) -> Self {
        v.into()
    }
    pub fn with_start_end(mut self, start: usize, end: usize) -> Self {
        self.range.start = start;
        self.range.end = end;
        self
    }
    pub fn with_start(mut self, offset: usize) -> Self {
        self.range.start = offset;
        self.range.end = max(self.range.start, self.range.end);
        self
    }
    pub fn with_end(mut self, offset: usize) -> Self {
        self.range.end = offset;
        self.range.start = min(self.range.start, self.range.end);
        self
    }
    pub fn with_file(mut self, file: FileID) -> Self {
        self.file = file;
        self
    }
}
