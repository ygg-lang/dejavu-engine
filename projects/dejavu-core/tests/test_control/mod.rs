use super::*;

use dejavu_ir::hir::DejavuRoot;
use dejavu_parser::{
    dejavu::{DejavuParser, DejavuRule, RootNode},
    YggdrasilNode, YggdrasilParser,
};
use std::{fs::File, io::Write};

#[test]
fn test_unicode() {
    let cst = DejavuParser::parse_cst(include_str!("test_if.djv"), DejavuRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = DejavuRoot::from(RootNode::from_cst(cst).unwrap());
    let mut file = File::create("tests/test_control/test_if.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

#[test]
fn test_unicode2() {
    let cst = DejavuParser::parse_cst("true", DejavuRule::TemplateIf).unwrap();
    println!("Short Form:\n{}", cst);
}
