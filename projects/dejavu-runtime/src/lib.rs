#![no_std]
extern crate alloc;

mod display;
pub mod escaper;
pub mod looper;
mod traits;

pub use crate::{
    display::{EscapeDisplay, Escaper},
    traits::Template,
};
