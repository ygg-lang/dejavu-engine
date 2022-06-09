use std::str::FromStr;
use ucd_trie::TrieSetOwned;

use saha_parser::parse;
use saha_types::Decimal;

#[test]
fn test() {
    let mut vm = SahaVM::default();

    parse(include_str!("test.saha"), &Default::default()).unwrap();
}

#[test]
fn test_null() {
    parse("{% null %}", &Default::default()).unwrap();
}

#[test]
fn test_boolean() {
    parse("{% true %}", &Default::default()).unwrap();
    parse("{% false %}", &Default::default()).unwrap();
}

#[test]
fn test_number() {
    Decimal::from_str("0").unwrap();
    parse("{% 0 %}", &Default::default()).unwrap();
}

#[test]
pub fn test_trie() {
    let set = TrieSetOwned::from_scalars(vec!['-']).unwrap();
    println!("{:#?}", set.as_slice())
}
