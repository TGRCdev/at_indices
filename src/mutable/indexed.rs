use std::ops::IndexMut;
use crate::SelectIndicesIterMut;
use std::{
    collections::HashSet,
    hash::Hash,
};

pub struct SelectIndicesIndexedIterMut<'a, D, I>
where
    D: ?Sized,
    I: Iterator,
    I::Item: Copy + Hash + Eq,
{
    data: &'a mut D,
    index_iter: I,
    visited: HashSet<I::Item>
}

impl<'a, D, I> Iterator for SelectIndicesIndexedIterMut<'a, D, I>
where
    I: Iterator,
    I::Item: Copy + Hash + Eq,
    D: ?Sized + IndexMut<I::Item>,
    D::Output: 'a,
{
    type Item = (I::Item, &'a mut D::Output);

    fn next(&mut self) -> Option<Self::Item> {
        self.index_iter.next().map(|i| {
            assert!(self.visited.insert(i), "select_indices_mut was passed duplicate indices!");

            let ptr: *mut _ = self.data;
            (i, unsafe { ptr.as_mut().unwrap().index_mut(i) })
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.index_iter.size_hint()
    }
}

impl<'a, D, I> ExactSizeIterator for SelectIndicesIndexedIterMut<'a, D, I>
where
    I: ExactSizeIterator,
    I::Item: Copy + Hash + Eq,
    D: ?Sized + IndexMut<I::Item>,
    D::Output: 'a,
{}

impl<'a, D, I> DoubleEndedIterator for SelectIndicesIndexedIterMut<'a, D, I>
where
    I: DoubleEndedIterator,
    I::Item: Copy + Hash + Eq,
    D: ?Sized + IndexMut<I::Item>,
    D::Output: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.index_iter.next_back().map(|i| {
            assert!(self.visited.insert(i), "select_indices_mut was passed duplicate indices!");

            let ptr: *mut _ = self.data;
            (i, unsafe { ptr.as_mut().unwrap().index_mut(i) })
        })
    }
}

impl<'a, D, I> SelectIndicesIterMut<'a, D, I>
where
    D: ?Sized,
    I: Iterator,
    I::Item: Copy + Hash + Eq,
{
    pub fn indexed(self) -> SelectIndicesIndexedIterMut<'a, D, I>
    {
        SelectIndicesIndexedIterMut {
            data: self.data,
            index_iter: self.index_iter,
            visited: self.visited,
        }
    }
}