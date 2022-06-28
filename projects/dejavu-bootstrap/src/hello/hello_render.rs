use core::fmt::Write;

use dejavu_runtime::{Result, Template};

impl Template for crate::hello::HelloTemplate {
    const SIZE_HINT: usize = 100;
    const MIME_TYPE: &'static str = "text/html; charset=utf-8";
    const EXTENSION: &'static str = "html";

    fn render_into<W: Write + ?Sized>(&self, fmt: &mut W) -> Result<()> {
        if true {}
        fmt.write_str("impl")?;
        fmt.write_str("{}", self.target)?;
        fmt.write_str("Display for")?;
        fmt.write_str("{}", self.target)?;
        fmt.write_str("{\n    #[inline]\n    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {")?;
        fmt.write_str("{}", self.dejavu)?;
        fmt.write_str("Template::render_into(self, f).map_err(|_| core::fmt::Error {})\n    }\n}\n\nimpl")?;
        fmt.write_str("{}", self.target)?;
        fmt.write_str("Template for")?;
        fmt.write_str("{}", self.target)?;
        fmt.write_str("{\n    const SIZE_HINT: usize =")?;
        fmt.write_str("{}", self.target)?;
        fmt.write_str(";\n    const MIME_TYPE: &'static str = \"")?;
        fmt.write_str("{}", self.target)?;
        fmt.write_str("\";\n    const EXTENSION: &'static str = \"")?;
        fmt.write_str("{}", self.target)?;
        fmt.write_str("\";\n\n    fn render_into(&self, writer: &mut (impl Write + ?Sized)) -> Result<()> {\n        writer.write_str(\"<h1>Users</h1>\\n<ul>\\n    \")?;\n        {\n            let mut did_loop = false;\n            let iter = (&self.users).into_iter();\n            if !did_loop {}\n        }\n        writer.write_str(\"\\n</ul>\")?;\n        Ok(())\n    }\n}")?;
        Ok(())
    }
}
