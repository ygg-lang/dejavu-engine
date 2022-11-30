mod basic_render;

pub struct RustTemplate {
    pub name: String,
    pub users: Vec<User>,
    pub dejavu: DejavuConfig,
}

impl Default for RustTemplate {
    fn default() -> Self {
        Self { name: "crate::rust_codegen::RustTemplate".to_string(), users: vec![], dejavu: DejavuConfig { imports: vec![] } }
    }
}

pub struct DejavuConfig {
    pub imports: Vec<String>,
}

pub struct User {
    pub name: String,
}
