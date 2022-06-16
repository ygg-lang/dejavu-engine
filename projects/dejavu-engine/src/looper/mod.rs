use core::iter::{Enumerate, Peekable};

pub trait Looper {}

impl<T: Iterator> Looper for ForLooper<T> {}

pub struct ForLooper<I>
where
    I: Iterator,
{
    iter: Peekable<Enumerate<I>>,
}

impl<I> ForLooper<I>
where
    I: Iterator,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        ForLooper { iter: iter.enumerate().peekable() }
    }
}

impl<I> Iterator for ForLooper<I>
where
    I: Iterator,
{
    type Item = (<I as Iterator>::Item, LoopItem);

    #[inline]
    fn next(&mut self) -> Option<(<I as Iterator>::Item, LoopItem)> {
        self.iter.next().map(|(index, item)| (item, LoopItem { index, first: index == 0, last: self.iter.peek().is_none() }))
    }
}

#[derive(Copy, Clone)]
pub struct LoopItem {
    pub index: usize,
    pub first: bool,
    pub last: bool,
}
