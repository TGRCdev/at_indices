use crate::data::SelectIndicesBase;
use rayon_crate::{
    prelude::*,
    iter::plumbing::bridge,
};

pub struct SelectIndicesIterMutPar<'a, T>(SelectIndicesBase<'a, T>);

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
        C: rayon_crate::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T: Send> IndexedParallelIterator for SelectIndicesIterMutPar<'a, &'a mut [T]>
{
    fn len(&self) -> usize {
        self.0.indices.len()
    }

    fn drive<C: rayon_crate::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB: rayon_crate::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(self.0)
    }
}