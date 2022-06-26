use super::*;

mod der;
mod ser;

pub struct WorkspaceConfig {
    pub root: PathBuf,
}

impl WorkspaceConfig {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn default_path(&self) -> PathBuf {
        self.root.join("dejavu.toml")
    }
    pub fn reload_from(&mut self, config_path: &Path) -> QResult {
        println!("Reloading workspace config: {}", self.root.display());
        Ok(())
    }
}
