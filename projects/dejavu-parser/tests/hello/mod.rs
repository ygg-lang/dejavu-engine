use dejavu_parser::parse_dejavu;

#[test]
fn test() {
    let a = parse_dejavu(include_str!("basic.md.dejavu")).unwrap();
}
