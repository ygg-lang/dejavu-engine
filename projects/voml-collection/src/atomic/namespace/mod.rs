use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A symbol with namespace
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Namespace {
    /// The path of the namespace
    pub path: Vec<String>,
}

impl Namespace {
    /// Clear the symbol path
    pub fn clear(&mut self) {
        self.path.clear()
    }
    /// Push the symbol to name
    pub fn push<T>(&mut self, value: T)
    where
        String: From<T>,
    {
        self.path.push(String::from(value))
    }
    /// The module path of the symbol
    pub fn module(&self) -> &[String] {
        let len = self.path.len();
        if len != 0 { &self.path[0..len - 1] } else { &[] }
    }
    /// The name of the symbol
    pub fn name(&self) -> &str {
        match self.path.last() {
            Some(s) => s.as_str(),
            None => "",
        }
    }
}

impl<V> FromIterator<V> for Namespace
where
    String: From<V>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        Namespace { path: Vec::from_iter(iter.into_iter().map(|v| String::from(v))) }
    }
}
