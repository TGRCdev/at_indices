use crate::data::SelectIndicesBase;
use rayon::{
    prelude::*,
    slice::Iter,
    iter::{
        plumbing::bridge,
        Zip, Cloned
    },
};

pub struct SelectIndicesIterMutPar<'a, T>(pub(crate) SelectIndicesBase<'a, T>);

impl<'a, T> From<SelectIndicesBase<'a, &'a mut [T]>> for SelectIndicesIterMutPar<'a, &'a mut [T]>
{
    fn from(d: SelectIndicesBase<'a, &'a mut [T]>) -> Self {
        Self(d)
    }
}

impl<'a, T: Send> ParallelIterator for SelectIndicesIterMutPar<'a, &'a mut [T]>
{
    type Item = &'a mut T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T: Send> IndexedParallelIterator for SelectIndicesIterMutPar<'a, &'a mut [T]>
{
    fn len(&self) -> usize {
        self.0.indices.len()
    }

    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(self.0)
    }
}

impl<'a, T: Send + Sync> SelectIndicesIterMutPar<'a, &'a mut [T]>
{
    /// Return an iterator that outputs a tuple with
    /// each given index and its corresponding element
    /// 
    /// ```
    /// # fn main() {
    /// # use select_indices::prelude::*;
    /// # use rayon::prelude::*;
    /// let mut data = [
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    ///     00,00,00,00,00,
    /// ];
    /// 
    /// data.par_select_indices_mut(&[2, 6, 8, 10, 14, 16, 18, 22])
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
    pub fn indexed(self) -> Zip<Cloned<Iter<'a, usize>>, Self>
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