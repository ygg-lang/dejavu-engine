use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter, Write},
    ops::{AddAssign, Range},
};

pub use self::{
    cond::{DejavuBranches, DejavuConditional},
    expr::DejavuExpression,
    looping::{DejavuLoop, DejavuPattern},
    text::{DejavuText, DejavuTextTrim},
};
use indentation::{display_wrap, DisplayIndent, IndentFormatter};

mod cond;
mod expr;
mod looping;
mod text;

#[derive(Clone, Debug)]
pub struct DejavuRoot {
    pub body: DejavuSequence,
}

#[derive(Clone, Debug, Default)]
pub struct DejavuSequence {
    pub statements: Vec<DejavuStatement>,
}

#[derive(Clone)]
pub enum DejavuStatement {
    Text(DejavuText),
    Branches(DejavuBranches),
    ForLoop(DejavuLoop),
}

#[derive(Clone)]
pub struct DejavuIdentifier {
    pub text: String,
    pub range: Range<usize>,
}

impl Debug for DejavuIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Identifier({}, {:?})", self.text, self.range)
    }
}

impl Debug for DejavuStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            DejavuStatement::Text(v) => Debug::fmt(v, f),
            DejavuStatement::Branches(v) => Debug::fmt(v, f),
            DejavuStatement::ForLoop(v) => Debug::fmt(v, f),
        }
    }
}

impl DisplayIndent for DejavuRoot {
    fn fmt_indent<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result {
        f.write_str("use super::*;\n\n")?;
        f.write_str("impl core::fmt::Display for HelloTemplate {")?;
        f.indent();
        f.write_newline()?;
        f.write_str("#[inline]")?;
        f.write_newline()?;
        f.write_str("fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {")?;
        f.indent();
        for e in &self.body.statements {
            f.write_newline()?;
            e.fmt_indent(f)?;
        }
        f.dedent();
        f.write_str("\n        Ok(())\n    }\n}")
    }
}

impl DisplayIndent for DejavuSequence {
    fn fmt_indent<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result {
        f.write_str("{")?;
        f.indent();
        for e in &self.statements {
            f.write_newline()?;
            e.fmt_indent(f)?;
        }
        f.dedent();
        f.write_newline()?;
        f.write_str("}")?;
        Ok(())
    }
}

display_wrap![DejavuRoot, DejavuStatement, DejavuText];
display_wrap![DejavuBranches, DejavuConditional];
impl DisplayIndent for DejavuStatement {
    fn fmt_indent<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result {
        match self {
            DejavuStatement::Text(v) => v.fmt_indent(f),
            DejavuStatement::Branches(v) => v.fmt_indent(f),
            DejavuStatement::ForLoop(v) => v.fmt_indent(f),
        }
    }
}

impl<T> AddAssign<T> for DejavuSequence
where
    T: Into<DejavuStatement>,
{
    fn add_assign(&mut self, rhs: T) {
        self.statements.push(rhs.into())
    }
}

impl DejavuSequence {
    pub fn new(capacity: usize) -> Self {
        Self { statements: Vec::with_capacity(capacity) }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
    /// `%} ... {%`
    pub fn trim_text(&mut self, head: DejavuTextTrim, tail: DejavuTextTrim) {
        if let Some(DejavuStatement::Text(v)) = self.statements.first_mut() {
            v.trim_head(head)
        }
        if let Some(DejavuStatement::Text(v)) = self.statements.last_mut() {
            v.trim_tail(tail)
        }
    }
}

pub trait CodeGenerator {}
