use super::*;

mod test_if;

use std::{fs::File, io::Write};

#[test]
fn test_unicode() {
    let cst = NexusParser::parse_cst(include_str!("test_if.djv"), NexusRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let out = DejavuDisplay { root: RootNode::from_cst(cst).unwrap() };
    let mut file = File::create("tests/test_control/test_if.rs").unwrap();
    file.write_all(out.to_string().as_bytes()).unwrap();
}
