use std::ops::Index;
use crate::indexed_type::Indexed;
use crate::immutable::iter::ParSelectIndicesIter;
use rayon::{
    prelude::*,
    iter::plumbing::{ Consumer, UnindexedConsumer },
};

impl<'a, Data, Indices> ParallelIterator for ParSelectIndicesIter<'a, Data, Indices, Indexed>
where
    Data: ?Sized + Index<Indices::Item> + Sync,
    Data::Output: 'a + Sync,
    Indices: ParallelIterator,
    Indices::Item: Copy,
{
    type Item = (Indices::Item, &'a Data::Output);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>
    {
        let data = self.data;
        self.indices.map(|index| {
            (index, &data[index])
        }).drive_unindexed(consumer)
    }
}

impl<'a, Data, Indices> IndexedParallelIterator for ParSelectIndicesIter<'a, Data, Indices, Indexed>
where
    Data: ?Sized + Index<Indices::Item> + Sync,
    Data::Output: 'a + Sync,
    Indices: IndexedParallelIterator,
    Indices::Item: Copy,
{
    fn len(&self) -> usize {
        self.indices.len()
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        let data = self.data;
        self.indices.map(|index| {
            (index, &data[index])
        }).drive(consumer)
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        let data = self.data;
        self.indices.map(|index| {
            (index, &data[index])
        }).with_producer(callback)
    }
}