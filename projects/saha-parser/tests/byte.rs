use std::str::FromStr;

use ucd_trie::TrieSetOwned;

use saha_parser::parse;
use saha_types::{Decimal, Failure, SahaCompiler, Success};

#[test]
fn test() {
    let mut vm = SahaCompiler::default();
    let file = vm.add_file("tests/test.saha").unwrap();
    let text = vm.get_text(&file).unwrap();
    match parse(text, &file) {
        Success { value: _, diagnostics } => vm.print_errors(&diagnostics).unwrap(),
        Failure { fatal, diagnostics } => {
            vm.print_errors(&diagnostics).unwrap();
            vm.print_errors(&[fatal]).unwrap()
        }
    }
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
