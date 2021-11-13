use crate::data::AtIndicesData;
use rayon::{
    prelude::*,
    iter::plumbing::{ Producer, bridge },
};

pub struct AtIndicesIterMutPar<'a, T>(AtIndicesData<'a, T>);

impl<'a, T> From<AtIndicesData<'a, &'a mut [T]>> for AtIndicesIterMutPar<'a, &'a mut [T]>
{
    fn from(d: AtIndicesData<'a, &'a mut [T]>) -> Self {
        Self(d)
    }
}

impl<'a, T: Send> ParallelIterator for AtIndicesIterMutPar<'a, &'a mut [T]>
{
    type Item = &'a mut T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl<'a, T: Send> IndexedParallelIterator for AtIndicesIterMutPar<'a, &'a mut [T]>
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