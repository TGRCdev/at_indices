use std::{
    ops::IndexMut,
    collections::HashSet,
    hash::Hash,
};
use crate::{
    OneToOne,
    iter_type::Unindexed,
};

pub struct SelectIndicesIterMut<'a, D, I>
where
    I: Iterator,
    I::Item: Hash + Eq,
    D: ?Sized,
{
    data: &'a mut D,
    index_iter: I,
    visited: HashSet<I::Item>,
}

impl<'a, D, I> Iterator for SelectIndicesIterMut<'a, D, I>
where
    I: Iterator,
    I::Item: Copy + Hash + Eq,
    D: ?Sized + IndexMut<I::Item> + OneToOne,
    D::Output: 'a,
{
    type Item = &'a mut D::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.index_iter.next().map(|i| {
            assert!(self.visited.insert(i), "select_indices_mut was passed duplicate indices!");

            let ptr: *mut _ = self.data;
            unsafe { ptr.as_mut().unwrap().index_mut(i) }
        })
    }
}

impl<'a, D, I> ExactSizeIterator for SelectIndicesIterMut<'a, D, I>
where
    I: ExactSizeIterator,
    I::Item: Copy + Hash + Eq,
    D: ?Sized + IndexMut<I::Item> + OneToOne,
    D::Output: 'a,
{}

impl<'a, D, I> DoubleEndedIterator for SelectIndicesIterMut<'a, D, I>
where
    I: DoubleEndedIterator,
    I::Item: Copy + Hash + Eq,
    D: ?Sized + IndexMut<I::Item> + OneToOne,
    D::Output: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.index_iter.next_back().map(|i| {
            assert!(self.visited.insert(i), "select_indices_mut was passed duplicate indices!");

            let ptr: *mut _ = self.data;
            unsafe { ptr.as_mut().unwrap().index_mut(i) }
        })
    }
}

mod slice;
pub use self::slice::*;

pub trait SelectIndicesMut<'a, D>
where
    D: ?Sized,
{
    fn select_indices_mut<I>(&'a mut self, indices: &'a [I]) -> SelectIndicesSliceIterMut<D, I, Unindexed>
    where
        D: IndexMut<I> + OneToOne,
        I: Sized + Copy + Hash + Eq;
    
    fn select_with_iter_mut<I>(&'a mut self, indices: I) -> SelectIndicesIterMut<D, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Copy + Hash + Eq,
        D: IndexMut<I::Item> + OneToOne;
}

impl<'a, D> SelectIndicesMut<'a, D> for D
where
    D: ?Sized,
{
    fn select_indices_mut<I>(&'a mut self, indices: &'a [I]) -> SelectIndicesSliceIterMut<D, I, Unindexed>
    where
        D: IndexMut<I> + OneToOne,
        I: Sized + Copy + Hash + Eq,
    {
        {
            let mut values_check = HashSet::with_capacity(indices.len());
            assert!(indices.iter().all(|index| values_check.insert(index)));
        }

        SelectIndicesSliceIterMut {
            data: self,
            index_iter: indices.into_iter(),
            _phantom: Default::default(),
        }
    }

    fn select_with_iter_mut<I>(&'a mut self, indices: I) -> SelectIndicesIterMut<D, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Copy + Hash + Eq,
        D: IndexMut<I::Item> + OneToOne,
    {
        let index_iter = indices.into_iter();
        let size = index_iter.size_hint();
        let size = size.1.unwrap_or(size.0);
        SelectIndicesIterMut {
            data: self,
            index_iter,
            visited: HashSet::with_capacity(size)
        }
    }
}

mod indexed;
pub use self::indexed::*;

#[cfg(feature = "rayon")]
mod rayon;
#[cfg(feature = "rayon")]
pub use self::rayon::*;