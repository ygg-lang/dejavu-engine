use rust_decimal::Decimal;
use saha_types::{ASTKind, Location, SahaNode};
use std::mem::size_of;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn keep_size() {
    assert_eq!(size_of::<ASTKind>(), 32);
    assert_eq!(size_of::<SahaNode>(), 64);

    assert_eq!(size_of::<u8>(), 1);
    assert_eq!(size_of::<String>(), 24);
    assert_eq!(size_of::<Decimal>(), 16);
    assert_eq!(size_of::<Vec<ASTKind>>(), 24);
    assert_eq!(size_of::<Location>(), 32);
}
