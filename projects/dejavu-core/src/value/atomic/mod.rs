pub struct Namespace {
    pub path: Vec<Identifier>,
}

pub struct Identifier {
    pub name: String,
}

impl Namespace {
    pub fn new() -> Self {
        Self { path: vec![] }
    }
}
