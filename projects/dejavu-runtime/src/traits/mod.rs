use core::fmt::Write;

mod escaper;

pub trait Template {
    /// Provides a conservative estimate of the expanded length of the rendered template
    const SIZE_HINT: usize;
    /// The MIME type (Content-Type) of the data that gets rendered by this Template
    const MIME_TYPE: &'static str;
    /// The template's extension, if provided
    const EXTENSION: &'static str;
    /// Helper method which allocates a new `String` and renders into it
    ///
    /// # Arguments
    ///
    /// * `fmt`:
    ///
    /// returns: Result<(), Error>
    ///
    /// # Examples
    ///
    /// ```
    /// use dejavu_runtime::Template;
    /// ```
    fn render(&self) -> anyhow::Result<String> {
        let mut buf = String::with_capacity(Self::SIZE_HINT);
        self.render_fmt(&mut buf)?;
        Ok(buf)
    }
    /// Renders the template to the given `writer` fmt buffer
    ///
    /// # Arguments
    ///
    /// * `fmt`:
    ///
    /// returns: Result<(), Error>
    ///
    /// # Examples
    ///
    /// ```
    /// use dejavu_runtime::Template;
    /// ```
    #[cfg(feature = "std")]
    fn render_io<W>(&self, io: &mut W) -> anyhow::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        io.write_all(self.render()?.as_bytes())?;
        Ok(())
    }

    /// Renders the template to the given `writer` fmt buffer
    ///
    /// # Arguments
    ///
    /// * `fmt`:
    ///
    /// returns: Result<(), Error>
    ///
    /// # Examples
    ///
    /// ```
    /// use dejavu_runtime::Template;
    /// ```
    fn render_fmt<W>(&self, fmt: &mut W) -> anyhow::Result<()>
    where
        W: Write + ?Sized;
}
