use super::*;
use num::One;
use std::convert::TryFrom;

/// Ordered map of key value pairs
#[derive(Clone, Debug)]
pub struct KVPair<T> {
    key: Literal<String>,
    value: Literal<T>,
}

/// Literal Patterns for command
#[derive(Clone, Default, Hash)]
pub struct LiteralVector<T> {
    inner: Vec<Literal<T>>,
}

/// Sparse representation of the array, the subscript can be any non-zero integer
/// 1-index
#[derive(Clone, Default, Debug, Hash)]
pub struct SparseArray<T> {
    default: T,
    inner: BTreeMap<BigUint, Literal<T>>,
}

impl<T> SparseArray<T> {
    /// Returns a reference to the value corresponding to the key.
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form must match the ordering on the key type.
    #[inline]
    pub fn get(&self, index: &BigUint) -> Option<&T> {
        self.inner.get(index).map(|f| &f.value)
    }
    /// Returns a [`bool`] value corresponding to the key.
    /// Return [`None`] if the key is not [`bool`] or missing.
    #[inline]
    pub fn get_bool(&self, index: &BigUint) -> Option<bool>
    where
        bool: TryFrom<T>,
        T: Clone,
    {
        self.get(index).cloned().and_then(|f| bool::try_from(f).ok())
    }
    /// Returns a [`String`] value corresponding to the key.
    /// Return [`None`] if the key is not [`String`] or missing.
    #[inline]
    pub fn get_string(&self, index: &BigUint) -> Option<String>
    where
        String: TryFrom<T>,
        T: Clone,
    {
        self.get(index).cloned().and_then(|f| String::try_from(f).ok())
    }
    /// Get last key value of the array.
    #[inline]
    pub fn last_key_value(&self) -> Option<(&BigUint, &T)> {
        self.inner.last_key_value().map(|(k, v)| (k, &v.value))
    }
}

impl<T> SparseArray<T> {
    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    #[inline]
    pub fn extract(&mut self, index: &BigUint) -> Option<T> {
        self.inner.remove(index).map(|f| f.value)
    }
}

impl<T> SparseArray<T> {
    /// Returns true if the array contains no elements
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Appends an element to the back of a table
    #[inline]
    #[allow(mutable_borrow_reservation_conflict)]
    pub fn push(&mut self, value: Literal<T>) {
        let last = self.inner.last_key_value().map(|f| f.0);
        match last {
            None => self.insert(BigUint::one(), value),
            Some(s) => self.inner.insert(s + 1u8, value),
        };
    }
    /// Inserts an element at position index within the vector, shifting all
    /// elements after it to the right
    #[inline]
    pub fn insert(&mut self, index: BigUint, value: Literal<T>) -> Option<Literal<T>> {
        self.inner.insert(index, value)
    }
}

impl<T> SparseArray<T> {
    /// Return an iterator over array with default value if not set
    #[inline]
    pub fn iter(&self) -> SparseArrayIter<T> {
        SparseArrayIter { current: BigUint::one(), default: &self.default, inner: &self.inner }
    }
}

/// Wrapper type of [`SparseArray::values`]
pub struct SparseArrayIter<'i, T> {
    current: BigUint,
    default: &'i T,
    inner: &'i BTreeMap<BigUint, Literal<T>>,
}

impl<'i, T> Iterator for SparseArrayIter<'i, T> {
    type Item = &'i T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1u8;
        match self.inner.get(&self.current) {
            None => Some(&self.default),
            Some(s) => Some(&s.value),
        }
    }
}
