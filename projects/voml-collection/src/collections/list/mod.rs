use std::fmt::{Debug, Formatter, Write};

use serde::{Deserialize, Serialize};

/// A
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct List<T> {
    /// The type hint of this list
    pub hint: String,
    /// The items of this list
    pub list: Vec<T>,
}

impl<T> Debug for List<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.hint.is_empty() {
            f.write_str(&self.hint)?;
            f.write_char(' ')?;
        }
        f.debug_list().entries(self.list.iter()).finish()
    }
}

impl<O, V> FromIterator<V> for List<O>
where
    O: From<V>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        let list = Vec::from_iter(iter.into_iter().map(|v| O::from(v)));
        List { hint: "".to_string(), list }
    }
}

impl<T> List<T> {
    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = vec![1, 2, 3];
    ///
    /// v.clear();
    ///
    /// assert!(v.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.list.clear()
    }
    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2];
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    pub fn push(&mut self, value: T) {
        self.list.push(value)
    }
    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or `None` if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range,
    ///   or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert_eq!(Some(&40), v.get(1));
    /// assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    /// assert_eq!(None, v.get(3));
    /// assert_eq!(None, v.get(0..4));
    /// ```
    pub fn get(&mut self, query: usize) -> Option<&T> {
        self.list.get(query)
    }
    /// Returns a mutable reference to an element or subslice depending on the
    /// type of index (see [`get`]) or `None` if the index is out of bounds.
    ///
    /// [`get`]: slice::get
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some(elem) = x.get_mut(1) {
    ///     *elem = 42;
    /// }
    /// assert_eq!(x, &[0, 42, 2]);
    /// ```
    pub fn get_mut(&mut self, query: usize) -> Option<&mut T> {
        self.list.get_mut(query)
    }
    /// Extends a collection with the contents of an iterator.
    ///
    /// As this is the only required method for this trait, the [trait-level] docs
    /// contain more details.
    ///
    /// [trait-level]: Extend
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // You can extend a String with some chars:
    /// let mut message = String::from("abc");
    ///
    /// message.extend(['d', 'e', 'f'].iter());
    ///
    /// assert_eq!("abcdef", &message);
    /// ```
    pub fn extend<I, V>(&mut self, iter: I)
    where
        I: IntoIterator<Item = V>,
        T: From<V>,
    {
        self.list.extend(iter.into_iter().map(|v| T::from(v)))
    }
}
