use std::path::{Path, PathBuf};

use diagnostic_quick::{print_errors, QError, QResult, TextStorage};
use serde::{Deserialize, Serialize};

use crate::FileID;

pub mod render;

pub struct DejavuWorkspace {
    root: PathBuf,
    /// `Dict<RelativePath, Cache>`
    store: TextStorage,
}

impl DejavuWorkspace {
    pub fn new(workspace: &Path) -> QResult<DejavuWorkspace> {
        let mut store = TextStorage::default();
        store.force_lf();
        let root = workspace.canonicalize()?;
        Ok(Self { root, store })
    }

    pub fn get_text(&self, id: &FileID) -> QResult<&str> {
        Ok(self.store.get_text(id)?)
    }
    pub fn add_file(&mut self, path: &Path) -> QResult<FileID> {
        let absolute = path.canonicalize()?;
        Ok(self.store.file(absolute)?)
    }
    pub fn print_errors(&self, errors: &[QError]) -> QResult {
        print_errors(&self.store, errors)
    }
}
