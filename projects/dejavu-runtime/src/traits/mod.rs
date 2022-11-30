use core::fmt::Write;
use std::fmt::Display;

mod escaper;

pub trait Template {
    /// Provides a conservative estimate of the expanded length of the rendered template
    const SIZE_HINT: usize;
    /// The MIME type (Content-Type) of the data that gets rendered by this Template
    const MIME_TYPE: &'static str;
    /// The template's extension, if provided
    const EXTENSION: &'static str;
    /// Helper method which allocates a new `String` and renders into it
    fn render(&self) -> anyhow::Result<String> {
        let mut buf = String::with_capacity(Self::SIZE_HINT);
        self.render_fmt(&mut buf)?;
        Ok(buf)
    }
    /// Renders the template to the given `writer` fmt buffer
    fn render_fmt<W: Write + ?Sized>(&self, fmt: &mut W) -> anyhow::Result<()>;

    /// Renders the template to the given `writer` fmt buffer
    #[cfg(feature = "std")]
    fn render_io<W>(&self, io: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
        Self: Display,
    {
        io.write_fmt(format_args!("{}", self))
    }
}

#[test]
fn test() {}
