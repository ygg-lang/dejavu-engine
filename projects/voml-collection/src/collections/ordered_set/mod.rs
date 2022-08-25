use super::*;
use indexmap::set::Iter;
/// Ordered set of values
#[derive(Clone, Default, Debug)]
pub struct OrderedSet<T> {
    inner: IndexSet<Literal<T>>,
}

impl<T> OrderedSet<T> {
    /// Returns an iterator over the slice.
    #[inline]
    pub fn iter(&self) -> OrderedSetIter<T> {
        OrderedSetIter { inner: self.inner.iter() }
    }
}

impl<'i, T> IntoIterator for &'i OrderedSet<T> {
    type Item = &'i T;
    type IntoIter = OrderedSetIter<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedSetIter { inner: self.inner.iter() }
    }
}
/// Wrapper type of [`OrderedSet::iter`]
pub struct OrderedSetIter<'i, T> {
    inner: Iter<'i, Literal<T>>,
}

impl<'i, T> Iterator for OrderedSetIter<'i, T> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| &f.value)
    }
}

impl<'i, T> DoubleEndedIterator for OrderedSetIter<'i, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|f| &f.value)
    }
}

impl<'i, T> ExactSizeIterator for OrderedSetIter<'i, T> {}
