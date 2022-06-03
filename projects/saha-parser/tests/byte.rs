use std::str::FromStr;

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
