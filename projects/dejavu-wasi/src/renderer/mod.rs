use core::fmt::Write;
use dejavu_runtime::Template;

impl Template for crate::exports::dejavu::core::syntax_tree::DejavuTemplate {
    const EXTENSION: &'static str = "txt";
    const MIME_TYPE: &'static str = "text/plain; charset=utf-8";
    const SIZE_HINT: usize = 1024;

    fn write_fmt<W>(&self, w: &mut W) -> core::fmt::Result
    where
        W: Write + ?Sized,
    {
        w.write_str("use core::fmt::Write;\n")?;
        w.write_fmt(format_args!("{}", 0))?;

        Ok(())
    }
}
