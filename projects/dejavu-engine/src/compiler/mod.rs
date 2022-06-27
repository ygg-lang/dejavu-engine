use std::{
    env::current_dir,
    mem::take,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

use diagnostic_quick::{print_errors, QError, QResult, TextStorage};
use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

use crate::{compiler::config::WorkspaceConfig, FileID};

pub mod config;
pub mod render;

pub struct DejavuWorkspace {
    config: WorkspaceConfig,
    /// `Dict<RelativePath, Cache>`
    store: TextStorage,
    error: Vec<QError>,
}

impl DejavuWorkspace {
    /// Use this when quickly configuring your project
    ///
    /// Contains some common configuration
    pub fn compile_project() -> QResult {
        // The directory with cargo.toml
        let dir = current_dir()?;
        let mut vm = DejavuWorkspace::new(&dir)?;
        let config_path = vm.config.default_path();
        if config_path.exists() {
            vm.reload_config(&config_path)?;
        }

        let err = vm.compile_all();
        vm.print_errors(&err)?;
        vm.format_rs()?;

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
    /// use dejavu_engine::DejavuWorkspace;
    /// ```
    pub fn new(workspace: &Path) -> QResult<DejavuWorkspace> {
        let mut store = TextStorage::default();
        store.force_lf();
        Ok(Self { config: WorkspaceConfig::new(workspace.canonicalize()?), store, error: vec![] })
    }
    pub fn reload_config(&mut self, path: &Path) -> QResult {
        self.config.reload_from(path)
    }
    pub fn format_rs(&self) -> QResult<ExitStatus> {
        Ok(Command::new("rustfmt").arg(&self.config.root).status()?)
    }
    pub fn compile_all(&mut self) -> Vec<QError> {
        let mut out = vec![];
        match self.try_compile_all() {
            Ok(_) => out.extend(take(&mut self.error)),
            Err(e) => {
                out.extend(take(&mut self.error));
                out.push(e);
            }
        }
        out
    }
    pub fn try_compile_all(&mut self) -> QResult {
        let glob = self.config.glob_pattern()?;
        let mut files = vec![];
        for entry in WalkDir::new(&self.config.root) {
            let entry = entry?;
            if !glob.is_match(entry.path()) {
                continue;
            }
            let file = self.store.file(entry.path())?;
            files.push(file);
        }
        for file in files {
            match self.compile(&file) {
                Ok(o) => self.error.extend(o),
                Err(e) => self.error.push(e),
            }
        }
        Ok(())
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
