use crate::data::SelectIndicesMutBase;
use rayon::{
    prelude::*,
    slice::Iter,
    iter::{
        plumbing::bridge,
        Zip
    },
};

use std::ops::{ Index, IndexMut };

pub struct SelectIndicesIterMutPar<'a, T: 'a + IndexMut<I> + ?Sized, I: Clone + Sync>
    (pub(crate) SelectIndicesMutBase<'a, T, I>)
    where <T as Index<I>>::Output: 'a + Send;

impl<'a, T: 'a + IndexMut<I, Output = O> + ?Sized, I: Clone + Sync, O: 'a + Send> From<SelectIndicesMutBase<'a, T, I>> for SelectIndicesIterMutPar<'a, T, I>
{
    fn from(d: SelectIndicesMutBase<'a, T, I>) -> Self {
        Self(d)
    }
}

impl<'a, T: 'a + IndexMut<I, Output = O> + ?Sized + Send, I: Clone + Sync, O: 'a + Send> ParallelIterator for SelectIndicesIterMutPar<'a, T, I>
{
    type Item = &'a mut O;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T: 'a + IndexMut<I, Output = O> + ?Sized + Send, I: Clone + Sync, O: 'a + Send> IndexedParallelIterator for SelectIndicesIterMutPar<'a, T, I>
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

impl<'a, T: 'a + IndexMut<I, Output = O> + ?Sized + Send, I: Clone + Sync, O: 'a + Send> SelectIndicesIterMutPar<'a, T, I>
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