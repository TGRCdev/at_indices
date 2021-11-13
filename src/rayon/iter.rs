use crate::data::SelectIndicesBase;
use rayon_crate::{
    prelude::*,
    iter::plumbing::bridge,
};

pub struct SelectIndicesIterPar<'a, T>(SelectIndicesBase<'a, T>);

impl<'a, T> From<SelectIndicesBase<'a, &'a [T]>> for SelectIndicesIterPar<'a, &'a [T]>
{
    fn from(d: SelectIndicesBase<'a, &'a [T]>) -> Self {
        Self(d)
    }
}

impl<'a, T: Send + Sync> ParallelIterator for SelectIndicesIterPar<'a, &'a [T]>
{
    type Item = &'a T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon_crate::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T: Send + Sync> IndexedParallelIterator for SelectIndicesIterPar<'a, &'a [T]>
{
    fn len(&self) -> usize {
        self.0.len()
    }

    fn drive<C: rayon_crate::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB: rayon_crate::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        callback.callback(self.0)
    }
}