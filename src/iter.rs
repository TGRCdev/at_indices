use std::iter::{Cloned, Zip};
use core::slice::Iter;

use crate::data::SelectIndicesBase;

use num_traits::{ PrimInt, ToPrimitive };

pub struct SelectIndicesIter<'a, T, I: Copy + Clone + PrimInt + ToPrimitive>(pub(crate) SelectIndicesBase<'a, T, I>);

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> From<SelectIndicesBase<'a, T, I>> for SelectIndicesIter<'a, T, I>
{
    fn from(d: SelectIndicesBase<'a, T, I>) -> Self {
        Self(d)
    }
}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> Iterator for SelectIndicesIter<'a, &'a [T], I>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> DoubleEndedIterator for SelectIndicesIter<'a, &'a [T], I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> ExactSizeIterator for SelectIndicesIter<'a, &'a [T], I> {}

impl<'a, T, I: Copy + Clone + PrimInt + ToPrimitive> SelectIndicesIter<'a, &'a [T], I>
{
    /// Return an iterator that outputs a tuple with
    /// each given index and its corresponding element
    /// 
    /// ```
    /// # fn main() {
    /// # use select_indices::prelude::*;
    /// let data = vec![
    ///     11, 22, 33, 44, 55, 66, 77, 88,
    ///     99, 00, 11, 22, 33, 44, 55, 66,
    ///     77, 88, 99, 00, 11, 22, 33, 44
    /// ];
    /// 
    /// let mut output = [0;5];
    /// 
    /// data.select_indices(&[4, 23, 12, 1, 20])
    ///     .indexed()
    ///     .zip(output.iter_mut())
    ///     .for_each(|((i, x), output)| 
    ///     {
    ///         *output = x * i;
    ///     });
    /// 
    /// assert_eq!(
    ///     output,
    ///     [220,1012,396,22,220]
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