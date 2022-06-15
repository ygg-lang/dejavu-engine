use std::fmt::{Display, Formatter, Write};

impl Display for HelloTemplate {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Template::render_into(self, f).map_err(|_| core::fmt::Error {})
    }
}

impl Template for HelloTemplate {
    fn render_into(&self, writer: &mut (impl Write + ?Sized)) -> ::askama::Result<()> {
        writer.write_str("<h1>Users</h1>\n<ul>\n    ")?;
        {
            let mut did_loop = false;
            let iter = (&self.users).into_iter();
            for (user, loop_item) in TemplateLoop::new(iter) {
                did_loop = true;
                writer.write_str("\n    ")?;
                if *(&(loop_item.first) as &bool) {
                    writer.write_fmt(format_args!(
                        "\n    <li>First: {expr0}</li>\n    ",
                        expr0 = &MarkupDisplay::new_unsafe(&(user.name), ::askama::Html),
                    ))?;
                }
                else {
                    writer.write_fmt(format_args!(
                        "\n    <li>User#{expr1}: {expr2}</li>\n    ",
                        expr1 = &MarkupDisplay::new_unsafe(&(loop_item.index + 1), ::askama::Html),
                        expr2 = &MarkupDisplay::new_unsafe(&(user.name), ::askama::Html),
                    ))?;
                }
                writer.write_str("\n    ")?;
            }
            if !did_loop {}
        }
        writer.write_str("\n</ul>")?;
        Ok(())
    }
    const EXTENSION: Option<&'static str> = Some("html");
    const SIZE_HINT: usize = 3;
    const MIME_TYPE: &'static str = "text/html; charset=utf-8";
}
