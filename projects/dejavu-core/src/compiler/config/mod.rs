use super::*;
use globset::GlobSet;
use serde::Deserialize;

mod der;
mod ser;

#[derive(Debug)]
pub struct WorkspaceConfig {
    pub root: PathBuf,
    pub include: String,
}

impl WorkspaceConfig {
    pub fn new(root: PathBuf) -> Self {
        Self { root, ..Default::default() }
    }
    pub fn default_path(&self) -> PathBuf {
        self.root.join("dejavu.toml")
    }
    pub fn reload_from(&mut self, config_path: &Path) -> QResult {
        let toml = std::fs::read_to_string(config_path)?;
        WorkspaceConfig::deserialize_in_place(&mut toml::Deserializer::new(&toml), self)?;
        Ok(())
    }
    pub fn glob_pattern(&self) -> QResult<GlobSet> {
        let mut builder = GlobSetBuilder::new();
        for pattern in self.include.trim().lines() {
            builder.add(Glob::new(pattern)?);
        }
        let set = builder.build()?;
        Ok(set)
    }
}
