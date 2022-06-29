use std::env::current_dir;

use diagnostic_quick::QError;

use dejavu_core::DejavuWorkspace;

mod hello;

#[test]
fn ready() {
    println!("it works!")
}

#[track_caller]
fn test_file(path: &str, vm: &mut DejavuWorkspace, errors: &mut Vec<QError>) {
    let file = vm.add_file(path).unwrap();
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
