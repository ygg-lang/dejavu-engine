use super::*;
use std::slice::Iter;

impl<T> LiteralVector<T> {
    /// Returns the number of elements in the vector, also referred to as its 'length'.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns true if the vector contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> LiteralVector<T> {
    /// Returns an iterator over the slice.
    #[inline]
    pub fn iter(&self) -> LiteralPatternIter<T> {
        LiteralPatternIter { inner: self.inner.iter() }
    }
}

/// Wrapper type of [`LiteralPattern::iter`]
pub struct LiteralPatternIter<'i, T> {
    inner: Iter<'i, Literal<T>>,
}

impl<'i, T> Iterator for LiteralPatternIter<'i, T> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| &f.value)
    }
}
