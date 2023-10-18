use std::{fs::File, io::Write, str::FromStr};

use dejavu_ir::hir::DejavuRoot;

mod test_if;

pub struct HelloTemplate {}

#[test]
fn test_rs_if() {
    let cst = DejavuRoot::from_str(include_str!("test_if.dj")).unwrap();
    println!("{cst:#?}");
    let text = cst.to_string();
    let mut file = File::create("tests/test_control/test_if.rs").unwrap();
    file.write_all(text.as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

#[test]
fn test_rs_for() {
    let cst = DejavuRoot::from_str(include_str!("test_for.dj")).unwrap();
    println!("{cst:#?}");
    let text = cst.to_string();
    let mut file = File::create("tests/test_control/test_for.rs").unwrap();
    file.write_all(text.as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}
