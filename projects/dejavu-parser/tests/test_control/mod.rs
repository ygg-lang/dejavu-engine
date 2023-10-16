use super::*;

use std::{fs::File, io::Write};

#[test]
fn test_unicode() {
    let cst = NexusParser::parse_cst(include_str!("test_if.djv"), NexusRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = RootNode::from_cst(cst).unwrap().as_hir();
    let mut file = File::create("tests/test_control/test_if.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

#[test]
fn test_unicode2() {
    let cst = NexusParser::parse_cst("true", NexusRule::TemplateIf).unwrap();
    println!("Short Form:\n{}", cst);
}
