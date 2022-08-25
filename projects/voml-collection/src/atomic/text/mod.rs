use serde::{Deserialize, Serialize};

mod primitive;

/// A text with the format
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Text {
    /// The format of this text
    pub hint: String,
    /// The content of this text
    pub text: String,
}

impl Text {
    /// Create a new text
    pub fn new(text: impl Into<String>, hint: impl Into<String>) -> Self {
        Self { hint: hint.into(), text: text.into() }
    }
    /// Appends the given [`char`] to the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::from("abc");
    ///
    /// s.push('1');
    /// s.push('2');
    /// s.push('3');
    ///
    /// assert_eq!("abc123", s);
    /// ```
    #[inline]
    pub fn push(&mut self, c: char) {
        self.text.push(c)
    }
    /// Appends a given string slice onto the end of this `Text`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::from("foo");
    ///
    /// s.push_str("bar");
    ///
    /// assert_eq!("foobar", s);
    /// ```
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.text.push_str(s)
    }
}
