use std::ops::Index;
use crate::SelectIndicesIter;

pub struct SelectIndicesIndexedIter<'a, D, I>
where
    I: Iterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
{
    data: &'a D,
    index_iter: I,
}

impl<'a, D, I> Iterator for SelectIndicesIndexedIter<'a, D, I>
where
    I: Iterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
    D::Output: 'a,
{
    type Item = (I::Item, &'a D::Output);

    fn next(&mut self) -> Option<Self::Item> {
        self.index_iter.next().map(|i| {
            (i, self.data.index(i))
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.index_iter.size_hint()
    }
}

impl<'a, D, I> ExactSizeIterator for SelectIndicesIndexedIter<'a, D, I>
where
    I: ExactSizeIterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
    D::Output: 'a,
{}

impl<'a, D, I> DoubleEndedIterator for SelectIndicesIndexedIter<'a, D, I>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
    D::Output: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.index_iter.next_back().map(|i| {
            (i, self.data.index(i))
        })
    }
}

impl <'a, D, I> SelectIndicesIter<'a, D, I>
where
    I: Iterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
{
    pub fn indexed(self) -> SelectIndicesIndexedIter<'a, D, I>
    {
        SelectIndicesIndexedIter {
            data: self.data,
            index_iter: self.index_iter,
        }
    }
}