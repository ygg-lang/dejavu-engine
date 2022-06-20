use core::fmt::Write;

use dejavu_engine::{Result, Template};

impl Template for crate::hello::HelloTemplate {
    const SIZE_HINT: usize = 100;
    const MIME_TYPE: &'static str = "text/html; charset=utf-8";
    const EXTENSION: &'static str = "html";

    fn render_into<W: Write + ?Sized>(&self, fmt: &mut W) -> Result<()> {
        fmt.write_str("true")?;
        fmt.write_str(", nothing")?;
        fmt.write_str("false")?;
        fmt.write_char('.')?;
        fmt.write_char('0')?;
        fmt.write_str("quick brown fox jumps over the lazy dog!")?;
        fmt.write_char('1')?;
        fmt.write_str("quick brown fox jumps over the lazy dog!")?;
        fmt.write_char('2')?;
        fmt.write_str("quick brown foxes jump over the lazy dog!")?;
        Ok(())
    }
}
