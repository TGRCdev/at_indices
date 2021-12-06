pub(crate) struct SelectIndicesBase<'a, T: ?Sized, I: Clone>
{
    pub(crate) data: &'a T,
    pub(crate) indices: &'a [I],
    pub(crate) start: usize,
    pub(crate) end: usize,
}

pub(crate) struct SelectIndicesMutBase<'a, T: ?Sized, I: Clone>
{
    pub(crate) data: &'a mut T,
    pub(crate) indices: &'a [I],
    pub(crate) start: usize,
    pub(crate) end: usize,
}

use std::{
    ops::{ Index, IndexMut },
};

impl<'a, T: 'a + Index<I> + ?Sized, I: Clone> Iterator for SelectIndicesBase<'a, T, I>
{
    type Item = &'a T::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            let ind = &self.indices[self.start];
            self.start += 1;
            return Some(
                &self.data[ind.clone()]
            );
        }
        else {
            return None;
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        return (len, Some(len));
    }
}
impl<'a, T: 'a + Index<I> + ?Sized, I: Clone> DoubleEndedIterator for SelectIndicesBase<'a, T, I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            self.end -= 1;
            let ind = &self.indices[self.end];
            return Some(&self.data[ind.clone()]);
        }
        else {
            return None;
        }
    }
}
impl<'a, T: 'a + Index<I> + ?Sized, I: Clone> ExactSizeIterator for SelectIndicesBase<'a, T, I> {}

impl<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone> Iterator for SelectIndicesMutBase<'a, T, I>
{
    type Item = &'a mut T::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            let ind = &self.indices[self.start];
            self.start += 1;
            let ptr: *mut T = self.data;
            return Some(
                unsafe { &mut (*ptr)[ind.clone()] }
            );
        }
        else {
            return None;
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        return (len, Some(len));
    }
}
impl<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone> DoubleEndedIterator for SelectIndicesMutBase<'a, T, I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            self.end -= 1;
            let ind = &self.indices[self.end];
            let ptr: *mut T = self.data;
            return Some(
                unsafe { &mut (*ptr)[ind.clone()] }
            );
        }
        else {
            return None;
        }
    }
}

impl<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone> ExactSizeIterator for SelectIndicesMutBase<'a, T, I> {}