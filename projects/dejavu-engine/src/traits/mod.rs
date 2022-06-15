use std::fmt::Display;

pub trait Template: Display {
    /// Provides a conservative estimate of the expanded length of the rendered template
    const SIZE_HINT: usize;
    /// The MIME type (Content-Type) of the data that gets rendered by this Template
    const MIME_TYPE: Option<&'static str>;
    /// The template's extension, if provided
    const EXTENSION: Option<&'static str>;
    /// Helper method which allocates a new `String` and renders into it
    fn render(&self) -> anyhow::Result<String> {
        let mut buf = String::with_capacity(Self::SIZE_HINT);
        self.render_into(&mut buf)?;
        Ok(buf)
    }

    /// Renders the template to the given `writer` fmt buffer
    fn render_into(&self, writer: &mut (impl std::fmt::Write + ?Sized)) -> anyhow::Result<()>;

    /// Renders the template to the given `writer` io buffer
    #[inline]
    fn write_into(&self, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        writer.write_fmt(format_args!("{}", self))
    }
}

pub trait Escaper {
    fn write_escaped<W>(&self, fmt: W, string: &str) -> std::fmt::Result
    where
        W: std::fmt::Write;
}

#[test]
fn test() {}
