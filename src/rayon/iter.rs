use crate::data::SelectIndicesBase;
use rayon::{
    prelude::*,
    slice::Iter,
    iter::{
        plumbing::bridge,
        Zip, Cloned
    },
};

use num_traits::{ PrimInt, ToPrimitive };

pub struct SelectIndicesIterPar<'a, T: Sync, I: Copy + Clone + PrimInt + ToPrimitive + Sync>(pub(crate) SelectIndicesBase<'a, T, I>);

impl<'a, T: Sync, I: Copy + Clone + PrimInt + ToPrimitive + Sync> From<SelectIndicesBase<'a, &'a [T], I>> for SelectIndicesIterPar<'a, &'a [T], I>
{
    fn from(d: SelectIndicesBase<'a, &'a [T], I>) -> Self {
        Self(d)
    }
}

impl<'a, T: Sync, I: Copy + Clone + PrimInt + ToPrimitive + Sync> ParallelIterator for SelectIndicesIterPar<'a, &'a [T], I>
{
    type Item = &'a T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T: Sync, I: Copy + Clone + PrimInt + ToPrimitive + Sync> IndexedParallelIterator for SelectIndicesIterPar<'a, &'a [T], I>
{
    fn len(&self) -> usize {
        self.0.len()
    }

    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(self.0)
    }
}

impl<'a, T: Sync, I: Copy + Clone + PrimInt + ToPrimitive + Sync + Send> SelectIndicesIterPar<'a, &'a [T], I>
{
    /// Return an iterator that outputs a tuple with
    /// each given index and its corresponding element
    /// 
    /// ```
    /// # fn main() {
    /// # use select_indices::prelude::*;
    /// # use rayon::prelude::*;
    /// let data = [
    ///     11, 22, 33, 44, 55, 66, 77, 88,
    ///     99, 00, 11, 22, 33, 44, 55, 66,
    ///     77, 88, 99, 00, 11, 22, 33, 44
    /// ];
    /// 
    /// let mut output = [0;5];
    /// 
    /// data.par_select_indices(&[4, 23, 12, 1, 20])
    ///     .indexed()
    ///     .zip(output.par_iter_mut())
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
            ].par_iter()
            .cloned()
            .zip(self);
    }
}