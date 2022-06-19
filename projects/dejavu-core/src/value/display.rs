use super::*;

impl Debug for DjvNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("kind", &self.kind).field("span", &self.span).finish()
    }
}

impl Display for ASTKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Debug for ASTKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTKind::Null => f.write_str("null"),
            ASTKind::Boolean(v) => match v {
                true => f.write_str("true"),
                false => f.write_str("false"),
            },
            ASTKind::Text(v) => f.debug_tuple("Text").field(v).finish(),
            ASTKind::Integer(v) => {
                write!(f, "{v}")
            }
            ASTKind::Decimal(v) => {
                write!(f, "{v}")
            }
            ASTKind::Identifier(v) => f.debug_tuple("Identifier").field(v).finish(),
            ASTKind::Vector(_) => {
                todo!()
            }
            ASTKind::Statements(_) => {
                todo!()
            }
            ASTKind::LeftDestroyer(v) => {
                write!(f, "{{%{v}")
            }
            ASTKind::RightDestroyer(v) => {
                write!(f, "{v}%}}")
            }
            ASTKind::ForStatement(_) => {
                todo!()
            }
        }
    }
}
