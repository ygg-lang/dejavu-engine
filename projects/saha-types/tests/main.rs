use saha_types::{Location, SahaNode, SahaValue};
use std::mem::size_of;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn keep_size() {
    assert_eq!(size_of::<SahaValue>(), 32);
    assert_eq!(size_of::<SahaNode>(), 64);

    assert_eq!(size_of::<u8>(), 1);
    assert_eq!(size_of::<String>(), 24);
    assert_eq!(size_of::<Vec<SahaValue>>(), 24);
    assert_eq!(size_of::<Location>(), 32);
}
