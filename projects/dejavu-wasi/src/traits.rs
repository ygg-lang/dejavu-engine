use indentation::IndentFormatter;
use std::fmt::Write;

pub trait RsDejavuBuilder {
    /// Display the type with indentation.
    fn rs_dejavu<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result;
}
