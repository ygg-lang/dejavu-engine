use core::fmt::{Debug, Display, Formatter};
use std::error::Error;

pub type Result<T> = std::result::Result<T, DejavuError>;

#[derive(Debug)]
pub enum DejavuError {
    Unknown,
}
impl Error for DejavuError {}

impl Display for DejavuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
