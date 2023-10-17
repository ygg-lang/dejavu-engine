use super::*;

#[derive(Clone, Debug)]
pub enum DejavuExpression {
    Null,
    Boolean(bool),
}

impl Default for DejavuExpression {
    fn default() -> Self {
        Self::Null
    }
}

impl Display for DejavuExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("true")
    }
}
