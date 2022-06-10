use ucd_trie::TrieSetOwned;

use saha_parser::parse;
use saha_types::{Failure, SahaCompiler, Success};

#[test]
fn test() {
    let mut vm = SahaCompiler::default();
    let file = vm.add_file("tests/literal.saha").unwrap();
    let text = vm.get_text(&file).unwrap();
    match parse(text, &file) {
        Success { value, diagnostics } => {
            println!("{:#?}", value);
            vm.print_errors(&diagnostics).unwrap()
        }
        Failure { fatal, diagnostics } => {
            vm.print_errors(&diagnostics).unwrap();
            vm.print_errors(&[fatal]).unwrap()
        }
    }
}

#[test]
pub fn test_trie() {
    let set = TrieSetOwned::from_scalars(vec!['-']).unwrap();
    println!("{:#?}", set.as_slice())
}
