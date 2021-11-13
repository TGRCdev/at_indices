use crate::data::AtIndicesData;
use rayon::{
    prelude::*,
    iter::plumbing::bridge,
};

pub struct AtIndicesIterPar<'a, T>(AtIndicesData<'a, T>);

impl<'a, T> From<AtIndicesData<'a, &'a [T]>> for AtIndicesIterPar<'a, &'a [T]>
{
    fn from(d: AtIndicesData<'a, &'a [T]>) -> Self {
        Self(d)
    }
}

impl<'a, T: Send + Sync> ParallelIterator for AtIndicesIterPar<'a, &'a [T]>
{
    type Item = &'a T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T: Send + Sync> IndexedParallelIterator for AtIndicesIterPar<'a, &'a [T]>
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