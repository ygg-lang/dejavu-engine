use super::*;

impl Display for SahaNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for SahaValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SahaValue::Null => f.write_str("null")?,
            SahaValue::Boolean(v) => write!(f, "{}", v)?,
            SahaValue::Text(v) => write!(f, "{}", v)?,
            SahaValue::Number(v) => write!(f, "{}", v)?,
            SahaValue::Identifier(v) => write!(f, "{}", v)?,
            SahaValue::Statements(_) => {}
            SahaValue::Vector => {}
            SahaValue::LeftDestroyer(_) => {}
            SahaValue::RightDestroyer(_) => {}
        }
        Ok(())
    }
}
