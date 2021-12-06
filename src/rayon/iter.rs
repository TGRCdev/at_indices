use crate::data::SelectIndicesBase;
use rayon::{
    prelude::*,
    slice::Iter,
    iter::{
        plumbing::bridge,
        Zip
    },
};
use std::ops::Index;

pub struct SelectIndicesIterPar<'a, T, I>
    (pub(crate) SelectIndicesBase<'a, T, I>)
    where
        T: 'a + Index<I> + ?Sized + Sync,
        I: Clone + Sync,
        <T as Index<I>>::Output: 'a + Sync;

impl<'a, T, I: Clone + Sync> From<SelectIndicesBase<'a, T, I>> for SelectIndicesIterPar<'a, T, I>
where
    T: 'a + Index<I> + ?Sized + Sync,
    I: Clone + Sync,
    <T as Index<I>>::Output: 'a + Sync
{
    fn from(d: SelectIndicesBase<'a, T, I>) -> Self {
        Self(d)
    }
}

impl<'a, T, I, O> ParallelIterator for SelectIndicesIterPar<'a, T, I>
where
    T: 'a + Index<I, Output = O> + ?Sized + Sync,
    I: Clone + Sync,
    O: 'a + Sync,
{
    type Item = &'a O;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T, I, O> IndexedParallelIterator for SelectIndicesIterPar<'a, T, I>
where
    T: 'a + Index<I, Output = O> + ?Sized + Sync,
    I: Clone + Sync,
    O: 'a + Sync,
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

impl<'a, T, I, O> SelectIndicesIterPar<'a, T, I>
where
    T: 'a + Index<I, Output = O> + ?Sized + Sync,
    I: Clone + Sync,
    O: 'a + Sync,
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
    pub fn indexed(self) -> Zip<Iter<'a, I>, Self>
    {
        return self.0.indices[
            self.0.start
            ..
            self.0.end
            ].par_iter()
            .zip(self);
    }
}