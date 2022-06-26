use std::fmt::{Arguments, Display, Octal, Result, Write};

use itertools::Itertools;

use crate::{
    DjvNode,
    ForStatement, Identifier, IfStatement, Namespace, value::{DjvKind, for_statement::DjvPattern},
};

pub struct NodeWriter<'i, W: Write> {
    pub writer: &'i mut W,
    pub depth: u8,
    pub predefined_identifiers: &'i [String],
}

impl<'i, W: Write> Write for NodeWriter<'i, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.writer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.writer.write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
        self.writer.write_fmt(args)
    }
}

impl<'i, W: Write> NodeWriter<'i, W> {

}

impl DjvNode {
   pub(super) fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>) -> Result {
        match &self.kind {
            DjvKind::Null => {
                todo!()
            }
            DjvKind::Boolean(v) => write!(self, r#"fmt.write_str("{v}")?;"#)?,
            DjvKind::Text(v) => {
                match v.chars().count() {
                    // drop
                    0 => {}
                    1 => unsafe { write!(self, "fmt.write_char({c:?})?;", c = v.chars().next().unwrap_unchecked())? },
                    _ => write!(self, "fmt.write_str({v:?})?;")?,
                }
            }
            DjvKind::Integer(v) => match *v {
                i if (0..=9).contains(&i) => write!(self, r#"fmt.write_char('{v}')?;"#)?,
                _ => write!(self, r#"fmt.write_str("{v}")?;"#)?,
            },
            DjvKind::Decimal(_) => {
                todo!()
            }
            DjvKind::Namespace(v) => v.write_nodes(self, self.depth == 0)?,
            DjvKind::Vector(_) => {
                todo!()
            }
            DjvKind::Statements(_) => {
                todo!()
            }
            DjvKind::LeftDestroyer(_) => {
                todo!()
            }
            DjvKind::RightDestroyer(_) => {
                todo!()
            }

            DjvKind::IfStatement(v) => v.write_nodes(self, self.depth + 1)?,
            DjvKind::ForStatement(v) => v.write_nodes(self, self.depth + 1)?,
            DjvKind::Binary(_) => {
                todo!()
            }
        }
    }
}

impl Namespace {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>) -> Result {
        match self.path.len() {
            1 => unsafe { self.path.get_unchecked(0).write_nodes(w, w.depth ==0) },
            _ => {
                let name = self.path.iter().map(|v| v.name.as_str()).join("::");
                write!(w, r#"fmt.write_str("{{}}", {name})?;"#)
            }
        }
    }
}

impl Identifier {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>, is_root: bool) -> Result {
        match w.predefined_identifiers.contains(&self.name) {
            true => write!(w, r#"fmt.write_str("{{}}", {name})?;"#, name = self.name),
            false if is_root => write!(w, r#"fmt.write_str("{{}}", self.{name})?;"#, name = self.name),
            false => write!(w, r#"fmt.write_str("{{}}", {name})?;"#, name = self.name),
        }
    }
}

impl IfStatement {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>) -> Result {
        write!(w, "if true {{}}")
    }
}

impl ForStatement {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>) -> Result {
        if self.backpack.is_empty() {
            self.write_pattern(w, depth)?;
        }
        else {
            self.write_pattern(w, depth)?;
        }
        Ok(())
    }
    fn write_pattern<W: Write>(&self, w: &mut NodeWriter<W>) -> Result {
        write!(w, "for {} in ", self.pattern)?;
        self.iterable.w

        Ok(())
    }
}

