use std::iter::{Cloned, Zip};
use core::slice::Iter;

use crate::data::SelectIndicesBase;

use num_traits::{ PrimInt, ToPrimitive };

pub struct SelectIndicesIterMut<'a, T, I: Copy + Clone + PrimInt + ToPrimitive>(pub(crate) SelectIndicesBase<'a, T, I>);

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> From<SelectIndicesBase<'a, T, I>> for SelectIndicesIterMut<'a, T, I>
{
    fn from(d: SelectIndicesBase<'a, T, I>) -> Self {
        Self(d)
    }
}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> Iterator for SelectIndicesIterMut<'a, &'a mut [T], I>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> DoubleEndedIterator for SelectIndicesIterMut<'a, &'a mut [T], I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> ExactSizeIterator for SelectIndicesIterMut<'a, &'a mut [T], I> {}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> SelectIndicesIterMut<'a, &'a mut [T], I>
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
    ///         *x = i;
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
    pub fn indexed(self) -> Zip<Cloned<Iter<'a, I>>, Self>
    {
        return self.0.indices[
            self.0.start
            ..
            self.0.end
            ].iter()
            .cloned()
            .zip(self);
    }
}