use std::ops::Index;
use crate::{
    indexed_type::Unindexed,
};
use super::iter::SeqSelectIndicesIter;

impl<'a, Data, Indices> Iterator for SeqSelectIndicesIter<'a, Data, Indices, Unindexed>
where
    Data: ?Sized + Index<Indices::Item>,
    Data::Output: 'a,
    Indices: Iterator,
    Indices::Item: Copy,
{
    type Item = &'a Data::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|index| {
            &self.data[index]
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<'a, Data, Indices> DoubleEndedIterator for SeqSelectIndicesIter<'a, Data, Indices, Unindexed>
where
    Data: ?Sized + Index<Indices::Item>,
    Data::Output: 'a,
    Indices: DoubleEndedIterator,
    Indices::Item: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.indices.next_back().map(|index| {
            &self.data[index]
        })
    }
}

impl<'a, Data, Indices> ExactSizeIterator for SeqSelectIndicesIter<'a, Data, Indices, Unindexed>
where
    Data: ?Sized + Index<Indices::Item>,
    Data::Output: 'a,
    Indices: ExactSizeIterator,
    Indices::Item: Copy,
{}