use std::{env::current_dir, path::PathBuf};

use ucd_trie::TrieSetOwned;

use saha_parser::parse;
use saha_types::{Decimal, Failure, QError, SahaVM, SahaNode, Success};

#[test]
fn test() {
    let here = current_dir().unwrap();
    let mut vm = SahaVM::new(&here).unwrap();
    let mut errors = vec![];
    let _ = test_file("tests/literal.md", &mut vm, &mut errors);
    let _ = test_file("tests/for-loop.md", &mut vm, &mut errors);
    vm.print_errors(&errors).unwrap()
}

#[track_caller]
fn test_file(path: &str, vm: &mut SahaVM, errors: &mut Vec<QError>) -> Vec<SahaNode> {
    let file = vm.add_file(&PathBuf::from(path)).unwrap();
    let text = vm.get_text(&file).unwrap();
    match parse(text, &file) {
        Success { value, diagnostics } => {
            errors.extend(diagnostics);
            value
        }
        Failure { fatal, diagnostics } => {
            errors.extend(diagnostics);
            errors.push(fatal);
            vec![]
        }
    }
}

#[test]
fn test_number() {
    assert_eq!(Decimal::from_str_exact("10").unwrap(), Decimal::from(10))
}

#[test]
pub fn test_trie() {
    let set = TrieSetOwned::from_scalars(vec!['-']).unwrap();
    println!("{:#?}", set.as_slice())
}
