use std::io::{Result, Write};

use itertools::Itertools;

use crate::{value::DjvKind, DjvNode, ForStatement, Identifier, IfStatement, Namespace};

pub struct NodeWriter<'i, W: Write> {
    pub writer: &'i mut W,
    pub node: &'i DjvNode,
    pub depth: u8,
    pub predefined_identifiers: &'i [String],
}

impl<'i, W: Write> Write for NodeWriter<'i, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

impl<'i, W: Write> NodeWriter<'i, W> {
    pub fn write_nodes(&mut self) -> Result<()> {
        match &self.node.kind {
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
        Ok(())
    }
}

impl Namespace {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>, is_root: bool) -> Result<()> {
        match self.path.len() {
            1 => unsafe { self.path.get_unchecked(0).write_nodes(w, is_root) },
            _ => {
                let name = self.path.iter().map(|v| v.name.as_str()).join("::");
                write!(w, r#"fmt.write_str("{{}}", {name})?;"#)
            }
        }
    }
}

impl Identifier {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>, is_root: bool) -> Result<()> {
        match w.predefined_identifiers.contains(&self.name) {
            true => write!(w, r#"fmt.write_str("{{}}", {name})?;"#, name = self.name),
            false if is_root => write!(w, r#"fmt.write_str("{{}}", self.{name})?;"#, name = self.name),
            false => write!(w, r#"fmt.write_str("{{}}", {name})?;"#, name = self.name),
        }
    }
}

impl IfStatement {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>, depth: u8) -> Result<()> {
        write!(w, "if true {{}}")
    }
}

impl ForStatement {
    fn write_nodes<W: Write>(&self, w: &mut NodeWriter<W>, depth: u8) -> Result<()> {
        if self.backpack.is_empty() {
            write!(w, "for {} in {} {{}}", self.identifier.name, self.expression)
        }
        else {
            write!(w, "for {} in {} {{}}", self.identifier.name, self.expression)
        }
    }
}
