use crate::exports::dejavu::core::{
    backends::{DejavuError, DejavuRoot, GuestRustDejavu},
    syntax_tree::{RootItem, TextElement},
};
use indentation::IndentFormatter;
use std::fmt::Write;

pub struct RsDejavu {}

impl GuestRustDejavu for RsDejavu {
    fn new(directory: String) -> Self {
        todo!()
    }

    fn generate(&self, ast: DejavuRoot) -> Result<(), DejavuError> {
        let mut f = IndentFormatter::new(String::new(), "    ");
        ast.rs_dejavu(&mut f).unwrap();
        Ok(())
    }
}

pub trait RsDejavuBuilder {
    /// Display the type with indentation.
    fn rs_dejavu<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result;
}

impl RsDejavuBuilder for DejavuRoot {
    fn rs_dejavu<W: Write>(&self, f: &mut IndentFormatter<W>) -> std::fmt::Result {
        f.write_str("use super::*;\n\n")?;
        f.write_str("impl core::fmt::Display for HelloTemplate {")?;
        f.indent();
        f.write_newline()?;
        f.write_str("#[inline]")?;
        f.write_newline()?;
        f.write_str("fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {")?;
        f.indent();
        for e in &self.blocks {
            f.write_newline()?;
            e.rs_dejavu(f)?;
        }
        f.dedent();
        f.write_str("\n        Ok(())\n    }\n}")
    }
}

impl RsDejavuBuilder for RootItem {
    fn rs_dejavu<W: Write>(&self, f: &mut IndentFormatter<W>) -> std::fmt::Result {
        match self {
            Self::Placeholder => {}
            Self::Text(e) => e.rs_dejavu(f)?,
        }
        Ok(())
    }
}

impl RsDejavuBuilder for TextElement {
    fn rs_dejavu<W: Write>(&self, f: &mut IndentFormatter<W>) -> std::fmt::Result {
        Ok(())
    }
}
