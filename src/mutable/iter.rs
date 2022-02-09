use std::{
    marker::PhantomData,
    collections::HashSet,
    sync::Mutex,
};
use crate::indexed_type::{ Unindexed, Indexed };

pub struct SelectIndicesMutIter<'a, Data, Indices, VisitedSet, IndexedType>
where
    Data: ?Sized,
{
    pub(crate) data: &'a mut Data,
    pub(crate) indices: Indices,
    pub(crate) visited_refs: VisitedSet,
    pub(crate) _phantom: PhantomData<IndexedType>,
}

impl<'a, Data, Indices, VisitedSet> SelectIndicesMutIter<'a, Data, Indices, VisitedSet, Unindexed>
{
    pub fn indexed(self) -> SelectIndicesMutIter<'a, Data, Indices, VisitedSet, Indexed>
    {
        SelectIndicesMutIter {
            data: self.data,
            indices: self.indices,
            visited_refs: self.visited_refs,
            _phantom: Default::default(),
        }
    }
}

pub type SelectIndicesUncheckedMutIter<'a, Data, Indices, IndexedType> = SelectIndicesMutIter<'a, Data, Indices, (), IndexedType>;

pub type SeqSelectIndicesMutIter<'a, Data, Indices, IndexedType> = SelectIndicesMutIter<'a, Data, Indices, HashSet<usize>, IndexedType>;
#[cfg(feature = "rayon")]
pub type ParSelectIndicesMutIter<'a, Data, Indices, IndexedType> = SelectIndicesMutIter<'a, Data, Indices, Mutex<HashSet<usize>>, IndexedType>;

pub type SeqSelectIndicesUncheckedMutIter<'a, Data, Indices, IndexedType> = SelectIndicesUncheckedMutIter<'a, Data, Indices, IndexedType>;
#[cfg(feature = "rayon")]
pub type ParSelectIndicesUncheckedMutIter<'a, Data, Indices, IndexedType> = SelectIndicesUncheckedMutIter<'a, Data, Indices, IndexedType>;

