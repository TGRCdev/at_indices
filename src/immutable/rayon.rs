use rayon::{
    prelude::*,
    iter::{
        plumbing::{ UnindexedConsumer, Consumer },
        Cloned,
    },
    slice::Iter,
};
use std::ops::Index;

pub struct ParSelectIndicesIter<'a, Data, Iter>
where
    Data: ?Sized,
{
    data: &'a Data,
    index_iter: Iter,
}

impl<'a, Data, Iter> ParallelIterator for ParSelectIndicesIter<'a, Data, Iter>
where
    Iter: ParallelIterator + Sync,
    Iter::Item: Copy,
    Data: ?Sized + Index<Iter::Item> + Sync + Send,
    Data::Output: 'a + Sync + Sized + Send,
{
    type Item = &'a Data::Output;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result 
    where
        C: UnindexedConsumer<Self::Item>
    {
        let data = self.data;
        self.index_iter
            .map(|index| &data[index])
            .drive_unindexed(consumer)
    }
}

impl<'a, Data, Iter> IndexedParallelIterator for ParSelectIndicesIter<'a, Data, Iter>
where
    Iter: IndexedParallelIterator + Sync,
    Iter::Item: Copy,
    Data: ?Sized + Index<Iter::Item> + Sync + Send,
    Data::Output: 'a + Sync + Send + Sized,
{
    fn len(&self) -> usize {
        self.index_iter.len()
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        let data = self.data;
        self.index_iter
            .map(|index| &data[index])
            .drive(consumer)
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        let data = self.data;
        self.index_iter
            .map(|index| &data[index])
            .with_producer(callback)
    }
}

pub trait ParSelectIndices<'a>
where
    Self: 'a,
{
    fn par_select_indices<I>(&'a self, indices: &'a [I]) -> ParSelectIndicesIter<'a, Self, Cloned<Iter<'a, I>>>
    where
        I: Copy + Sync + Send,
        Self: Index<I>,
    {
        ParSelectIndicesIter {
            data: self,
            index_iter: indices.par_iter().cloned(),
        }
    }

    fn par_select_with_iter<Iter>(&'a self, index_iter: Iter) -> ParSelectIndicesIter<'a, Self, Iter>
    where
        Iter: IntoParallelIterator<Iter = Iter>,
        Iter::Item: Copy,
        Self: Index<Iter::Item>,
    {
        ParSelectIndicesIter {
            data: self,
            index_iter: index_iter.into_par_iter(),
        }
    }
}

impl<'a, Data> ParSelectIndices<'a> for Data
where
    Data: ?Sized + 'a,
{}

mod indexed {
    use super::*;

    pub struct ParSelectIndicesIndexedIter<'a, Data, Iter>(pub ParSelectIndicesIter<'a, Data, Iter>)
    where
        Iter: ParallelIterator,
        Data: ?Sized;
    
    impl<'a, Data, Iter> ParallelIterator for ParSelectIndicesIndexedIter<'a, Data, Iter>
    where
        Iter: ParallelIterator + Sync,
        Iter::Item: Copy + Eq,
        Data: ?Sized + Index<Iter::Item> + Sync + Send,
        Data::Output: 'a + Sync + Send + Sized,
    {
        type Item = (Iter::Item, &'a Data::Output);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item> {
            let data = self.0.data;
            self.0.index_iter
            .map(|index| {
                (index, &data[index])
            })
            .drive_unindexed(consumer)
        }
    }
}
pub use indexed::*;

impl<'a, D, I> ParSelectIndicesIter<'a, D, I>
where
    D: ?Sized,
    I: ParallelIterator,
{
    pub fn indexed(self) -> ParSelectIndicesIndexedIter<'a, D, I>
    {
        return ParSelectIndicesIndexedIter(self)
    }
}