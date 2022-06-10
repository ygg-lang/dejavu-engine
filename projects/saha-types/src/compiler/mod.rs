use std::path::Path;

use diagnostic_quick::{print_errors, QError, QResult, TextStorage};
use serde::{Deserialize, Serialize};

use crate::FileID;

pub mod render;

pub struct SahaCompiler {
    /// `Dict<RelativePath, Cache>`
    store: TextStorage,
}

impl Default for SahaCompiler {
    fn default() -> Self {
        let mut store = TextStorage::default();
        store.force_lf();
        Self { store }
    }
}

impl SahaCompiler {
    pub fn get_text(&mut self, id: &FileID) -> QResult<&str> {
        Ok(self.store.get_text(id)?)
    }
    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> QResult<FileID> {
        let absolute = path.as_ref().canonicalize()?;
        Ok(self.store.file(absolute)?)
    }
    pub fn print_errors(&self, errors: &[QError]) -> QResult {
        print_errors(&self.store, errors)
    }
}
