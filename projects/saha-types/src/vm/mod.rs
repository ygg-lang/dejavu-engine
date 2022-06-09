use std::path::Path;

use diagnostic_quick::{print_errors, QError, QResult, TextStorage};

use crate::FileID;

pub struct SahaCompiler {
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
    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> QResult<FileID> {
        Ok(self.store.file(path)?)
    }
    pub fn print_errors(&self, errors: &[QError]) -> QResult {
        print_errors(&self.store, errors)
    }
}
