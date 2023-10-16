use askama::Template;
use dejavu_engine::DejavuBuilder;
use std::{fs::File, io::Write};

#[test]
fn test_codegen() {
    let cst = DejavuBuilder::new(include_str!("test_if.djv"));
    let text = cst.render().unwrap();
    let mut file = File::create("tests/test_control/test_if.rs").unwrap();
    file.write_all(text.as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}
