use super::*;
use indexmap::map::{Iter, Keys, Values};
use std::convert::TryFrom;
/// Ordered map of key value pairs
#[derive(Clone, Default, Debug)]
pub struct OrderedMap<T> {
    inner: IndexMap<String, KVPair<T>>,
}

impl<T> OrderedMap<T> {
    /// Get value from Ordered Map
    #[inline]
    pub fn get(&self, key: &str) -> Option<&T> {
        self.inner.get(key).map(|f| &f.value.value)
    }
    /// Get value from Ordered Map
    #[inline]
    pub fn get_bool(&self, key: &str) -> Option<bool>
    where
        bool: TryFrom<T>,
        T: Clone,
    {
        self.get(key).cloned().and_then(|f| bool::try_from(f).ok())
    }
    /// Get value from Ordered Map
    #[inline]
    pub fn get_string(&self, key: &str) -> Option<String>
    where
        String: TryFrom<T>,
        T: Clone,
    {
        self.get(key).cloned().and_then(|f| String::try_from(f).ok())
    }
}

impl<T> OrderedMap<T> {
    /// Extract value from Ordered Map
    #[inline]
    pub fn extract(&mut self, key: &str) -> Option<T> {
        self.inner.remove(key).map(|f| f.value.value)
    }
    /// Extract value from Ordered Map
    #[inline]
    pub fn extract_bool(&mut self, key: &str) -> Option<bool>
    where
        bool: TryFrom<T>,
    {
        self.extract(key).and_then(|f| bool::try_from(f).ok())
    }
    /// Extract value from Ordered Map
    #[inline]
    pub fn extract_string(&mut self, key: &str) -> Option<String>
    where
        String: TryFrom<T>,
    {
        self.extract(key).and_then(|f| String::try_from(f).ok())
    }
}

impl<T> OrderedMap<T> {
    /// Returns true if the array contains no elements
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Insert a ranged key-value pair in the map.
    #[inline]
    pub fn insert_raw(&mut self, pair: KVPair<T>) -> Option<KVPair<T>> {
        self.inner.insert(pair.key.value.to_owned(), pair)
    }
    /// Insert a key-value pair in the map.
    #[inline]
    pub fn insert(&mut self, key: String, value: T) -> Option<T> {
        let pair = KVPair { key: Literal { value: key, range: None }, value: Literal { value, range: None } };
        self.inner.insert(pair.key.value.to_owned(), pair).map(|pair| pair.value.value)
    }
}

/// Iterator related methods
impl<T> OrderedMap<T> {
    /// Return an iterator over the key-value pairs of the map in their order
    #[inline]
    pub fn iter(&self) -> OrderedMapIter<T> {
        OrderedMapIter { inner: self.inner.iter() }
    }
    /// Return an iterator over the key-value pairs of the map in their order
    #[inline]
    pub fn iter_raw(&self) -> OrderedMapIterRaw<T> {
        OrderedMapIterRaw { inner: self.inner.iter() }
    }
    /// Return an iterator over the keys of the map in their order
    #[inline]
    pub fn keys(&self) -> OrderedMapKeys<T> {
        OrderedMapKeys { inner: self.inner.keys() }
    }
    /// Return an iterator over the values of the map in their order
    #[inline]
    pub fn values(&self) -> OrderedMapValues<T> {
        OrderedMapValues { inner: self.inner.values() }
    }
}

impl<'i, T> IntoIterator for &'i OrderedMap<T> {
    type Item = (&'i String, &'i T);
    type IntoIter = OrderedMapIter<'i, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// impl<'i, K, V, S> IntoIterator for &'i mut IndexMap<K, V, S> {
//     type Item = (&'i K, &'i mut V);
//     type IntoIter = IterMut<'i, K, V>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_mut()
//     }
// }

/// Wrapper type of [`OrderedMap::iter`]
pub struct OrderedMapIter<'i, T> {
    inner: Iter<'i, String, KVPair<T>>,
}

impl<'i, T> Iterator for OrderedMapIter<'i, T> {
    type Item = (&'i String, &'i T);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, &v.value.value))
    }
}

impl<'i, T> ExactSizeIterator for OrderedMapIter<'i, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Wrapper type of [`OrderedMap::iter_raw`]
pub struct OrderedMapIterRaw<'i, T> {
    inner: Iter<'i, String, KVPair<T>>,
}

impl<'i, T> Iterator for OrderedMapIterRaw<'i, T> {
    type Item = (&'i Literal<String>, &'i Literal<T>);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| (&v.key, &v.value))
    }
}

/// Wrapper type of [`OrderedMap::values`]
pub struct OrderedMapKeys<'i, T> {
    inner: Keys<'i, String, KVPair<T>>,
}

impl<'i, T> Iterator for OrderedMapKeys<'i, T> {
    type Item = &'i String;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Wrapper type of [`OrderedMap::values`]
pub struct OrderedMapValues<'i, T> {
    inner: Values<'i, String, KVPair<T>>,
}

impl<'i, T> Iterator for OrderedMapValues<'i, T> {
    type Item = &'i T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| &f.value.value)
    }
}

impl<'i, T> DoubleEndedIterator for OrderedMapValues<'i, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|f| &f.value.value)
    }
}
