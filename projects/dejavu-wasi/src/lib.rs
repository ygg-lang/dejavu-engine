#![allow(unused_braces)]

pub struct DejavuHost {}

pub use crate::{bindings::UrlNative, exports::dejavu::core::types::DejavuError};

include!("exports/host.rs");
export!(DejavuHost);

mod bindings;
pub mod helpers;
mod traits;

pub mod backends;

pub type Result<T> = std::result::Result<T, DejavuError>;
