use super::*;

#[test]
fn test_unicode() {
    let cst = DejavuParser::parse_cst(include_str!("test_class.djv"), DejavuRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let first = RootNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}

#[test]
fn test_unicode2() {
    let cst = DejavuParser::parse_cst("<% export a to a by class a %>", DejavuRule::TemplateExport).unwrap();
    println!("Short Form:\n{}", cst);
}
