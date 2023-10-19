use dejavu_parser::{
    dejavu::{DejavuParser, DejavuRule, RootNode},
    YggdrasilParser,
};
use std::{fs::File, io::Write, str::FromStr};

#[test]
fn test_unicode() {
    let cst = DejavuParser::parse_cst(include_str!("test_if.djv"), DejavuRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let ast = RootNode::from_str(include_str!("test_if.djv")).unwrap();
    let mut file = File::create("tests/test_control/test_if.ron").unwrap();
    file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
    // file.write_all(out.to_string().as_bytes()).unwrap();
}

#[test]
fn test_if() {
    let cst = DejavuParser::parse_cst(
        r###"<% if true %>
   if text
<% end %>"###,
        DejavuRule::IfBegin,
    )
    .unwrap();
    println!("Short Form:\n{}", cst);
}

#[test]
fn test_if_begin() {
    let cst = DejavuParser::parse_cst("<%= if ture -%>", DejavuRule::IfBegin).unwrap();
    println!("Short Form:\n{}", cst);
}

#[test]
fn test_if_end() {
    let cst = DejavuParser::parse_cst("<%= end -%>", DejavuRule::IfEnd).unwrap();
    println!("Short Form:\n{}", cst);
}
