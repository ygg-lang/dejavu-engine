use std::fmt::{Formatter, Octal, Result};

use crate::{value::ASTKind, DjvNode};

impl Octal for DjvNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.kind {
            ASTKind::Null => {
                todo!()
            }
            ASTKind::Boolean(_) => {
                todo!()
            }
            ASTKind::Text(v) => {
                write!(f, "f.write_str({v:?})")
            }
            ASTKind::Integer(_) => {
                todo!()
            }
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
    }
}
