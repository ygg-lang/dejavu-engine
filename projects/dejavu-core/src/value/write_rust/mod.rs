use std::fmt::{Formatter, Octal, Result};

use crate::{value::ASTKind, DjvNode};

impl Octal for DjvNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.kind {
            ASTKind::Null => {
                todo!()
            }
            ASTKind::Boolean(v) => write!(f, r#"fmt.write_str("{v}")?;"#)?,
            ASTKind::Text(v) => {
                match v.chars().count() {
                    // drop
                    0 => {}
                    1 => unsafe { write!(f, "fmt.write_char({c:?})?;", c = v.chars().next().unwrap_unchecked())? },
                    _ => write!(f, "fmt.write_str({v:?})?;")?,
                }
            }
            ASTKind::Integer(v) => match *v {
                i if (0..=9).contains(&i) => write!(f, r#"fmt.write_char('{v}')?;"#)?,
                _ => write!(f, r#"fmt.write_str("{v}")?;"#)?,
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
        }
        Ok(())
    }
}
