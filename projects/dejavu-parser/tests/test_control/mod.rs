use super::*;

#[test]
fn test_unicode() {
    let cst = NexusParser::parse_cst(include_str!("test_if.djv"), NexusRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let first = RootNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}
