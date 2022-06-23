
        
use core::fmt::Write;

use dejavu_engine::{Result, Template};

impl Template for crate::hello::HelloTemplate {
    const SIZE_HINT: usize = 100;
    const MIME_TYPE: &'static str = "text/html; charset=utf-8";
    const EXTENSION: &'static str = "html";

    fn render_into<W: Write + ?Sized>(&self, fmt: &mut W) -> Result<()> {
        if true {}
        Ok(())
    }
}
        