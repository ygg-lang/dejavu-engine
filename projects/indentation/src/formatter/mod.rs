use core::{
    borrow::{Borrow, BorrowMut},
    fmt::{Arguments, Debug, Formatter, Write},
    ops::{AddAssign, SubAssign},
};

/// A wrapper around a `Formatter` that adds indentation.
pub struct IndentFormatter<W> {
    raw: W,
    /// The current indentation level.
    indent_level: usize,
    /// The characters to use for indentation.
    indent_chars: &'static str,
}

impl<'i, W> Borrow<W> for &'i mut IndentFormatter<W> {
    fn borrow(&self) -> &W {
        &self.raw
    }
}

impl<'i, W> BorrowMut<W> for &'i mut IndentFormatter<W> {
    fn borrow_mut(&mut self) -> &mut W {
        &mut self.raw
    }
}

impl<W> Debug for IndentFormatter<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IndentFormatter")
            .field("indent_level", &self.indent_level)
            .field("indent_chars", &self.indent_chars)
            .finish()
    }
}

impl<W> AddAssign<usize> for IndentFormatter<W> {
    fn add_assign(&mut self, rhs: usize) {
        self.indent_level = self.indent_level.saturating_add(rhs);
    }
}

impl<W> SubAssign<usize> for IndentFormatter<W> {
    fn sub_assign(&mut self, rhs: usize) {
        self.indent_level = self.indent_level.saturating_sub(rhs)
    }
}

impl<W> Write for IndentFormatter<W>
where
    W: Write,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.raw.write_str(s)
    }
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.raw.write_char(c)
    }
    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> core::fmt::Result {
        self.raw.write_fmt(args)
    }
}

impl<W> IndentFormatter<W> {
    /// Create a new `IndentFormatter` from a `Formatter` and an indent string.
    pub fn new(f: W, indent: &'static str) -> Self {
        Self { raw: f, indent_level: 0, indent_chars: indent }
    }
    /// Wrap an `IndentDisplay` in an `IndentFormatter`.
    pub fn wrap<T: DisplayIndent>(item: &T, f: &mut W) -> core::fmt::Result
    where
        W: Write,
    {
        item.fmt_indent(&mut IndentFormatter::new(f, "    "))
    }
    /// Unwrap the `IndentFormatter` to get the underlying `Formatter`.
    pub fn unwrap(self) -> W {
        self.raw
    }
    /// Write the current indentation level to the formatter.
    pub fn write_indent(&mut self) -> core::fmt::Result
    where
        W: Write,
    {
        for _ in 0..self.indent_level {
            self.raw.write_str(self.indent_chars)?;
        }
        Ok(())
    }
    /// Write a newline and the current indentation level to the formatter.
    pub fn write_newline(&mut self) -> core::fmt::Result
    where
        W: Write,
    {
        self.raw.write_char('\n')?;
        self.write_indent()?;
        Ok(())
    }
    /// Write a string, splitting it into lines and indenting each line.
    pub fn write_lines(&mut self, s: &str) -> core::fmt::Result
    where
        W: Write,
    {
        for line in s.lines() {
            self.write_indent()?;
            self.raw.write_str(line)?;
        }
        Ok(())
    }
    /// Add one to the current indentation level.
    pub fn indent(&mut self) {
        self.indent_level = self.indent_level.saturating_add(1);
    }
    /// Subtract one from the current indentation level.
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
}

/// A trait for types that can be displayed with indentation.
pub trait DisplayIndent {
    /// Display the type with indentation.
    fn fmt_indent<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result;
}
