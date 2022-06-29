use super::*;

impl Debug for DjvNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("kind", &self.kind).field("span", &self.span).finish()
    }
}

impl Display for DjvKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Debug for DjvKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DjvKind::Null => f.write_str("null"),
            DjvKind::Boolean(v) => match v {
                true => f.write_str("true"),
                false => f.write_str("false"),
            },
            DjvKind::Text(v) => f.debug_tuple("Text").field(v).finish(),
            DjvKind::String(v) => Debug::fmt(&v.value, f),
            DjvKind::Integer(v) => {
                write!(f, "{v}")
            }
            DjvKind::Decimal(v) => {
                write!(f, "{v}")
            }
            DjvKind::Namespace(v) => f.debug_tuple("Identifier").field(v).finish(),
            DjvKind::Vector(_) => {
                todo!()
            }
            DjvKind::Statements(_) => {
                todo!()
            }
            DjvKind::LeftDestroyer(v) => {
                write!(f, "{{%{v}")
            }
            DjvKind::RightDestroyer(v) => {
                write!(f, "{v}%}}")
            }
            DjvKind::ForStatement(_) => {
                todo!()
            }
            DjvKind::Binary(_) => {
                todo!()
            }
            DjvKind::IfStatement(_) => {
                todo!()
            }
        }
    }
}
