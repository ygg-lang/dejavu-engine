use dejavu_parser::{
    dejavu::{DejavuParser, DejavuRule, RootNode},
    YggdrasilParser,
};
use std::{fs::File, io::Write, str::FromStr};

#[test]
fn test_if_ast() {
    let cst = DejavuParser::parse_cst(include_str!("test_if.dj"), DejavuRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = RootNode::from_str(include_str!("test_if.dj")).unwrap();
    let mut file = File::create("tests/test_control/test_if.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
}

#[test]
fn test_for_ast() {
    let input = include_str!("test_for.dj");
    let cst = DejavuParser::parse_cst(input, DejavuRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = RootNode::from_str(input).unwrap();
    let mut file = File::create("tests/test_control/test_for.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
}

#[test]
fn test_if() {
    let cst = DejavuParser::parse_cst(
        r###"{% if true %}
   if text
{% end %}"###,
        DejavuRule::TemplateIf,
    )
    .unwrap();
    println!("Short Form:\n{}", cst);
}

#[test]
fn test_for_begin() {
    let cst = DejavuParser::parse_cst("{%= for i in j -%}", DejavuRule::ForBegin).unwrap();
    println!("Short Form:\n{}", cst);
}

#[test]
fn test_if_begin() {
    let cst = DejavuParser::parse_cst("{%= if ture -%}", DejavuRule::IfBegin).unwrap();
    println!("Short Form:\n{}", cst);
}

#[test]
fn test_if_end() {
    let cst = DejavuParser::parse_cst("{%= end -%}", DejavuRule::IfEnd).unwrap();
    println!("Short Form:\n{}", cst);
}
