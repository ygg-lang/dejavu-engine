use std::{env::current_dir, path::PathBuf};

use diagnostic_quick::QError;

use dejavu_core::DejavuWorkspace;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let here = current_dir().unwrap();
    let mut vm = DejavuWorkspace::new(&here).unwrap();
    let mut errors = vec![];
    test_file("tests/basic.md.djv", &mut vm, &mut errors);
    // let _ = test_file("tests/for-loop.md", &mut vm, &mut errors);
    vm.print_errors(&errors).unwrap()
}

#[track_caller]
fn test_file(path: &str, vm: &mut DejavuWorkspace, errors: &mut Vec<QError>) {
    let file = vm.add_file(&PathBuf::from(path)).unwrap();
    match vm.compile(&file) {
        Ok(o) => errors.extend(o),
        Err(e) => errors.push(e),
    }
}
//
// #[test]
// fn test_number() {
//     assert_eq!(Decimal::from_str_exact("10").unwrap(), Decimal::from(10))
// }
//
// #[test]
// pub fn test_trie() {
//     let set = TrieSetOwned::from_scalars(vec!['-']).unwrap();
//     println!("{:#?}", set.as_slice())
// }
