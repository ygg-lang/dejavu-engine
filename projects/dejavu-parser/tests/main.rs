use dejavu_parser::dejavu::{NexusParser, NexusRule, RootNode};
use std::fmt::Formatter;
use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};

#[test]
fn ready() {
    println!("it works!")
}

mod test_class;
mod test_control;

#[test]
fn test_ascii() {
    let cst = NexusParser::parse_cst("[true, false, 1, 2, null]", NexusRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let first = RootNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}
