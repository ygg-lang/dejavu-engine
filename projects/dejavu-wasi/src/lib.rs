#![allow(unused_braces)]

pub struct NotedownHost {}

pub use crate::bindings::UrlNative;

include!("exports/host.rs");
export!(NotedownHost);

mod bindings;
pub mod helpers;
mod traits;

pub use crate::exports::notedown::core::types::NotedownError;

pub type Result<T> = std::result::Result<T, NotedownError>;
