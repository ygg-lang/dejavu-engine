use voml_collection::{Dict, List, Namespace};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let ns = Namespace::from_iter(vec!["a", "b"]);
    println!("{:#?}", ns)
}

#[test]
fn list() {
    let mut ns: List<String> = List::from_iter(vec!["a", "b"]);
    println!("{:#?}", ns);
    ns.hint = "G".to_string();
    println!("{:#?}", ns);
    ns.clear();
    println!("{:#?}", ns);
}

#[test]
fn test_dict() {
    let mut dict = Dict::<u8>::default();
    dict.insert("a", 1);
    dict.insert("b", 1);
    dict.insert("a", 2);
    println!("{:#?}", dict);
    dict.hint = "G".to_string();
    println!("{:#?}", dict);
    dict.clear();
    println!("{:#?}", dict);
}
