use std::str::FromStr;
use ucd_trie::TrieSetOwned;

use saha_parser::parse;
use saha_types::Decimal;

#[test]
fn test() {
    parse(include_str!("test.saha")).unwrap();
}

#[test]
fn test_null() {
    parse("{% null %}").unwrap();
}

#[test]
fn test_boolean() {
    parse("{% true %}").unwrap();
    parse("{% false %}").unwrap();
}

#[test]
fn test_number() {
    Decimal::from_str("0").unwrap();
    parse("{% 0 %}").unwrap();
}

#[test]
pub fn test_trie() {
    let set = TrieSetOwned::from_scalars(vec!['-']).unwrap();
    println!("{:#?}", set.as_slice())
}
