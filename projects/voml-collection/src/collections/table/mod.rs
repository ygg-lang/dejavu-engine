use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Table<T> {
    /// The type hint of this table
    pub hint: String,
    /// The items in this table
    pub list: Vec<T>,
    /// The key-value pairs in this table
    pub dict: IndexMap<String, T>,
}

impl<T> Table<T> {
    /// Remove all key-value pairs in the map, while preserving its capacity.
    ///
    /// Computes in **O(n)** time.
    pub fn clear(&mut self) {
        self.list.clear();
        self.dict.clear();
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

    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_index(&self, query: usize) -> Option<&T> {
        self.list.get(query)
    }

    /// Return a reference to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_index_mut(&mut self, query: usize) -> Option<&mut T> {
        self.list.get_mut(query)
    }
}
