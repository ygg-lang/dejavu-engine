use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};
use dejavu_parser::dejavu::{DejavuParser, DejavuRule, RootNode};

#[test]
fn ready() {
    println!("it works!")
}

mod test_control;


#[test]
fn test_ascii() {
    let cst = DejavuParser::parse_cst("[true, false, 1, 2, null]", DejavuRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let first = RootNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}
