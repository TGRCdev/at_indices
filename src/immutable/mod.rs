use std::{
    ops::Index,
    slice::Iter,
    iter::Cloned,
};

pub struct SelectIndicesIter<'a, D, I>
where
    D: ?Sized,
{
    data: &'a D,
    index_iter: I,
}

impl<'a, D, I> Iterator for SelectIndicesIter<'a, D, I>
where
    I: Iterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
    D::Output: 'a,
{
    type Item = &'a D::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.index_iter.next().map(|i| {
            self.data.index(i)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.index_iter.size_hint()
    }
}

impl<'a, D, I> ExactSizeIterator for SelectIndicesIter<'a, D, I>
where
    I: ExactSizeIterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
    D::Output: 'a,
{}

impl<'a, D, I> DoubleEndedIterator for SelectIndicesIter<'a, D, I>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
    D: ?Sized + Index<I::Item>,
    D::Output: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.index_iter.next_back().map(|i| {
            self.data.index(i)
        })
    }
}

pub trait SelectIndices<'a, D>
where
    D: ?Sized
{
    fn select_indices<I>(&'a self, indices: &'a [I]) -> SelectIndicesIter<D, Cloned<Iter<'a, I>>>
    where
        D: Index<I>,
        I: Copy
    {
        self.select_with_iter(indices.iter().cloned())
    }

    fn select_with_iter<I>(&'a self, index_iter: I) -> SelectIndicesIter<D, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Copy,
        D: Index<I::Item>;
}

impl<'a, D> SelectIndices<'a, D> for D
where
    D: ?Sized,
{
    fn select_indices<I>(&'a self, indices: &'a [I]) -> SelectIndicesIter<'a, D, Cloned<std::slice::Iter<'a, I>>>
    where
        I: Copy,
        D: Index<I>
    {
        SelectIndicesIter { 
            data: self,
            index_iter: indices.iter().cloned()
        }
    }

    fn select_with_iter<I>(&'a self, index_iter: I) -> SelectIndicesIter<D, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Copy,
        D: Index<I::Item>
    {
        SelectIndicesIter {
            data: self,
            index_iter: index_iter.into_iter(),
        }
    }
}

mod indexed;
pub use self::indexed::*;

#[cfg(feature = "rayon")]
mod rayon;
#[cfg(feature = "rayon")]
pub use self::rayon::*;