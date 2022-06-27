use dejavu_engine::{DejavuWorkspace, QResult};

#[test]
fn ready() {
    println!("dejavu-bootstrap works!")
}

#[test]
fn codegen() -> QResult {
    // println!("cargo:warning=Hello, world!");
    DejavuWorkspace::compile_project()
}
