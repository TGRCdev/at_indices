use std::{
    iter::Zip,
    ops::IndexMut,
};
use core::slice::Iter;

use crate::data::SelectIndicesMutBase;

pub struct SelectIndicesIterMut<'a, T: ?Sized, I: Clone>(pub(crate) SelectIndicesMutBase<'a, T, I>);

impl<'a, T: ?Sized, I: Clone> From<SelectIndicesMutBase<'a, T, I>> for SelectIndicesIterMut<'a, T, I>
{
    fn from(d: SelectIndicesMutBase<'a, T, I>) -> Self {
        Self(d)
    }
}

impl<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone> Iterator for SelectIndicesIterMut<'a, T, I>
{
    type Item = &'a mut T::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone> DoubleEndedIterator for SelectIndicesIterMut<'a, T, I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone> ExactSizeIterator for SelectIndicesIterMut<'a, T, I> {}

//pub type SelectIndicesIndexedIterMut<'a, T, I> = Zip<Cloned<Iter<'a, I>>, SelectIndicesIterMut<'a, &'a mut [T], I>>;

impl<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone> SelectIndicesIterMut<'a, T, I>
{
    /// Return an iterator that outputs a tuple with
    /// each given index and its corresponding element
    /// 
    /// ```
    /// # fn main() {
    /// # use select_indices::prelude::*;
    /// let mut data = [
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    /// ];
    /// 
    /// data.select_indices_mut(&[2, 6, 8, 10, 14, 16, 18, 22])
    ///     .indexed()
    ///     .for_each(|(i, x)| {
    ///         *x = *i;
    ///     });
    /// 
    /// assert_eq!(
    ///     data,
    ///     [
    ///         00,00,02,00,00,
    ///         00,06,00,08,00,
    ///         10,00,00,00,14,
    ///         00,16,00,18,00,
    ///         00,00,22,00,00,
    ///     ]
    /// );
    /// # }
    /// ```
    pub fn indexed(self) -> Zip<Iter<'a, I>, SelectIndicesIterMut<'a, T, I>>
    {
        let iter = self.0.indices[
            self.0.start
            ..
            self.0.end
            ].iter()
            .zip(self);
        
        return iter;
    }
}