mod basic_render;

pub struct RustTemplate {
    pub name: String,
    pub users: Vec<User>,
}

impl Default for RustTemplate {
    fn default() -> Self {
        Self { name: "crate::rust_codegen::RustTemplate".to_string(), users: vec![] }
    }
}

pub struct User {
    pub name: String,
}
