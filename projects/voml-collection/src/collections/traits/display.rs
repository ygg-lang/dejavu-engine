use super::*;
use std::fmt::{Debug, Display, Formatter};

impl<T: Debug> Debug for LiteralVector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter().map(|s| &s.value)).finish()
    }
}

impl<T: Display> Display for LiteralVector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.inner {
            write!(f, "[{}]", i.value)?;
        }
        Ok(())
    }
}
