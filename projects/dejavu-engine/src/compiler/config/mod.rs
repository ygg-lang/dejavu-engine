use super::*;
use globset::GlobSet;
use serde::Deserialize;

mod der;
mod ser;

pub struct WorkspaceConfig {
    pub root: PathBuf,
    pub include: String,
}

const DEFAULT_INCLUDE: &str = "
*.dj
*.djv
*.dejavu
";

impl WorkspaceConfig {
    pub fn new(root: PathBuf) -> Self {
        Self { root, include: DEFAULT_INCLUDE.trim().to_string() }
    }
    pub fn default_path(&self) -> PathBuf {
        self.root.join("dejavu.toml")
    }
    pub fn reload_from(&mut self, config_path: &Path) -> QResult {
        println!("Reloading workspace config: {}", self.root.display());
        let toml = std::fs::read_to_string(config_path)?;
        WorkspaceConfig::deserialize_in_place(&mut toml::Deserializer::new(&toml), self)?;
        Ok(())
    }
    pub fn glob_pattern(&self) -> QResult<GlobSet> {
        let mut builder = GlobSetBuilder::new();
        for pattern in self.include.lines() {
            builder.add(Glob::new(pattern)?);
        }
        let set = builder.build()?;
        Ok(set)
    }
}
