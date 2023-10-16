use dejavu_parser::{
    dejavu::{NexusParser, NexusRule, RootNode},
    DejavuDisplay,
};
use std::{fmt::Formatter, str::FromStr};
use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};

#[test]
fn ready() {
    println!("it works!")
}

mod test_class;
mod test_control;

#[test]
fn test_ascii() {
    let outer = DejavuDisplay { root: RootNode::from_str("[true, false, 1, 2, null]").unwrap() };
    println!("{}", outer)
}
