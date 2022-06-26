use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use crate::compiler::config::WorkspaceConfig;
use diagnostic_quick::{print_errors, QError, QResult, TextStorage};

use crate::FileID;

pub mod config;
pub mod render;

pub struct DejavuWorkspace {
    config: WorkspaceConfig,
    /// `Dict<RelativePath, Cache>`
    store: TextStorage,
}

impl DejavuWorkspace {
    /// Use this when quickly configuring your project
    ///
    /// Contains some common configuration
    pub fn compile_project() {
        if let Err(e) = Self::try_compile_project() {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
    fn try_compile_project() -> QResult {
        // The directory with cargo.toml
        let dir = current_dir()?;
        let mut vm = DejavuWorkspace::new(&dir)?;
        vm.config.root

        vm.reload_config()?;

        let errors = vm.compile_all();
        vm.print_errors(&errors)?;
        Ok(())
    }

    /// For step-by-step configuration of your project with better flexibility
    ///
    /// # Arguments
    ///
    /// * `workspace`:
    ///
    /// returns: Result<DejavuWorkspace, QError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(workspace: &Path) -> QResult<DejavuWorkspace> {
        let mut store = TextStorage::default();
        store.force_lf();
        let mut config = WorkspaceConfig::new(workspace.canonicalize()?);
        Ok(Self { config: WorkspaceConfig::new(workspace.canonicalize()?), store })
    }
    pub fn reload_config(&mut self, path: &Path) -> QResult {
        self.config.reload_from(path)
    }

    pub fn compile_all(&self) -> Vec<QError> {
        todo!()
    }
}

impl DejavuWorkspace {
    pub fn get_text(&self, id: &FileID) -> QResult<&str> {
        Ok(self.store.get_text(id)?)
    }
    pub fn add_file(&mut self, path: impl AsRef<Path>) -> QResult<FileID> {
        let absolute = path.as_ref().canonicalize()?;
        Ok(self.store.file(absolute)?)
    }
    pub fn print_errors(&self, errors: &[QError]) -> QResult {
        print_errors(&self.store, errors)
    }
}
