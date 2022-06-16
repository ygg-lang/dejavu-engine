#![allow(unused_imports)]

use core::fmt::{Display, Formatter, Write};

use dejavu_engine::{Result, Template};

impl Display for crate::hello::HelloTemplate {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Template::render_into(self, f).map_err(|_| core::fmt::Error {})
    }
}

impl Template for crate::hello::HelloTemplate {
    const SIZE_HINT: usize = 100;
    const MIME_TYPE: &'static str = "text/html; charset=utf-8";
    const EXTENSION: &'static str = "html";

    fn render_into<W: Write + ?Sized>(&self, writer: &mut W) -> Result<()> {
        writer.write_fmt(format_args!("<h1>Users</h1>"))?;
        Ok(())
    }
}
