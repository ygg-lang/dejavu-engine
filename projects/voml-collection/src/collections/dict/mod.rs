use std::fmt::{Debug, Formatter, Write};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A dict with string keys
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dict<T> {
    /// The type hint in this dict
    pub hint: String,
    /// The key-value pairs in this table
    pub dict: IndexMap<String, T>,
}

impl<T> Debug for Dict<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.hint.is_empty() {
            f.write_str(&self.hint)?;
            f.write_char(' ')?;
        }
        f.debug_map().entries(self.dict.iter()).finish()
    }
}

impl<O, K, V> FromIterator<(K, V)> for Dict<O>
where
    O: From<V>,
    String: From<K>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        Dict { hint: "".to_string(), dict: IndexMap::from_iter(iter.into_iter().map(|(k, v)| (String::from(k), O::from(v)))) }
    }
}

impl<T> Dict<T> {
    /// Remove all key-value pairs in the map, while preserving its capacity.
    ///
    /// Computes in **O(n)** time.
    pub fn clear(&mut self) {
        self.dict.clear()
    }
    /// Insert a key-value pair in the map.
    ///
    /// If an equivalent key already exists in the map: the key remains and
    /// retains in its place in the order, its corresponding value is updated
    /// with `value` and the older value is returned inside `Some(_)`.
    ///
    /// If no equivalent key existed in the map: the new key-value pair is
    /// inserted, last in order, and `None` is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    ///
    /// See also [`entry`](#method.entry) if you you want to insert *or* modify
    /// or if you need to get the index of the corresponding key-value pair.
    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<T>
    where
        K: Into<String>,
        V: Into<T>,
    {
        self.dict.insert(k.into(), v.into())
    }
    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_key(&self, query: &str) -> Option<&T> {
        self.dict.get(query)
    }
    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_key_mut(&mut self, query: &str) -> Option<&mut T> {
        self.dict.get_mut(query)
    }
    /// Get a key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_index(&self, query: usize) -> Option<&T> {
        self.dict.get_index(query).map(|v| v.1)
    }
    /// Get a key-value pair by index
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    pub fn get_index_mut(&mut self, query: usize) -> Option<&mut T> {
        self.dict.get_index_mut(query).map(|v| v.1)
    }
}
