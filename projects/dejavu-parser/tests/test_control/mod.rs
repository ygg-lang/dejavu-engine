use super::*;


#[test]
fn test_unicode() {
    let cst = DejavuParser::parse_cst(include_str!("test_if.djv"), DejavuRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let first = RootNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}
