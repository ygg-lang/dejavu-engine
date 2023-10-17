use dejavu_engine::DejavuBuilder;
use dejavu_ir::hir::DejavuRoot;
use std::{fs::File, io::Write, str::FromStr};

mod test_if;

pub struct HelloTemplate {}

#[test]
fn test_codegen() {
    let cst = DejavuRoot::from_str(include_str!("test_if.djv")).unwrap();
    println!("{cst:#?}");
    let text = cst.to_string();
    let mut file = File::create("tests/test_control/test_if.rs").unwrap();
    file.write_all(text.as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}
