use core::{
    fmt::{Display, Formatter, Result, Write},
    str,
};

pub mod utils;

/// Write escaped string
///
/// # Arguments
///
/// * `fmt`:
/// * `string`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use dejavu_runtime::Escaper;
/// ```
pub trait Escaper {
    /// Write escaped string
    ///
    /// # Arguments
    ///
    /// * `fmt`:
    /// * `string`:
    ///
    /// returns: Result<(), Error>
    ///
    /// # Examples
    ///
    /// ```
    /// use dejavu_runtime::Escaper;
    /// ```
    fn write_escaped<W>(&self, fmt: W, string: &str) -> Result
    where
        W: Write;
}
/// Write escaped string
///
/// # Arguments
///
/// * `fmt`:
/// * `string`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use dejavu_runtime::Escaper;
/// ```
pub struct XmlEscaper;

impl Escaper for XmlEscaper {
    fn write_escaped<W>(&self, mut fmt: W, string: &str) -> Result
    where
        W: Write,
    {
        for c in string.chars() {
            match c {
                '<' => fmt.write_str("&lt;")?,
                '>' => fmt.write_str("&gt;")?,
                '&' => fmt.write_str("&amp;")?,
                '"' => fmt.write_str("&quot;")?,
                '\'' => fmt.write_str("&#x27;")?,
                _ => fmt.write_char(c)?,
            }
        }
        Ok(())
    }
}
/// Write escaped string
///
/// # Arguments
///
/// * `fmt`:
/// * `string`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use dejavu_runtime::Escaper;
/// ```
pub struct PlainText;

impl Escaper for PlainText {
    fn write_escaped<W>(&self, mut fmt: W, string: &str) -> Result
    where
        W: Write,
    {
        fmt.write_str(string)
    }
}
