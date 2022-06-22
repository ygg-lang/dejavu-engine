use std::{fmt::Octal, io::Write};

use crate::{value::ASTKind, DjvNode};

pub struct NodeWriter<'i, W: Write> {
    pub writer: &'i mut W,
    pub node: &'i DjvNode,
    pub is_root: bool,
}

impl<'i, W: Write> Write for NodeWriter<'i, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<'i, W: Write> NodeWriter<'i, W> {
    pub fn write_nodes(&mut self) -> std::io::Result<()> {
        match &self.node.kind {
            ASTKind::Null => {
                todo!()
            }
            ASTKind::Boolean(v) => write!(self, r#"fmt.write_str("{v}")?;"#)?,
            ASTKind::Text(v) => {
                match v.chars().count() {
                    // drop
                    0 => {}
                    1 => unsafe { write!(self, "fmt.write_char({c:?})?;", c = v.chars().next().unwrap_unchecked())? },
                    _ => write!(self, "fmt.write_str({v:?})?;")?,
                }
            }
            ASTKind::Integer(v) => match *v {
                i if (0..=9).contains(&i) => write!(self, r#"fmt.write_char('{v}')?;"#)?,
                _ => write!(self, r#"fmt.write_str("{v}")?;"#)?,
            },
            ASTKind::Decimal(_) => {
                todo!()
            }
            ASTKind::Identifier(_) => {
                todo!()
            }
            ASTKind::Vector(_) => {
                todo!()
            }
            ASTKind::Statements(_) => {
                todo!()
            }
            ASTKind::LeftDestroyer(_) => {
                todo!()
            }
            ASTKind::RightDestroyer(_) => {
                todo!()
            }
            ASTKind::ForStatement(_) => {
                todo!()
            }
            ASTKind::Binary(_) => {
                todo!()
            }
        }
        Ok(())
    }
}
