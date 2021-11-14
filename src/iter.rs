use std::iter::{Cloned, Zip};
use core::slice::Iter;

use crate::data::SelectIndicesBase;

pub struct SelectIndicesIter<'a, T>(pub(crate) SelectIndicesBase<'a, T>);

impl<'a, T> From<SelectIndicesBase<'a, T>> for SelectIndicesIter<'a, T>
{
    fn from(d: SelectIndicesBase<'a, T>) -> Self {
        Self(d)
    }
}

impl<'a, T> Iterator for SelectIndicesIter<'a, &'a [T]>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for SelectIndicesIter<'a, &'a [T]>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T> ExactSizeIterator for SelectIndicesIter<'a, &'a [T]> {}

impl<'a, T> SelectIndicesIter<'a, &'a [T]>
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
    pub fn indexed(self) -> Zip<Cloned<Iter<'a, usize>>, Self>
    {
        return self.0.indices.iter().cloned().zip(self);
    }
}