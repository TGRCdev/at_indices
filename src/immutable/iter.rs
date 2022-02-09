use std::{
    marker::PhantomData,
    collections::HashSet,
    sync::Mutex,
};
use crate::indexed_type::{ Unindexed, Indexed };
use crate::iter_type::{ Parallel, Sequential };

pub struct SelectIndicesIter<'a, Data, Indices, IterType, IndexedType>
where
    Data: ?Sized,
{
    pub(crate) data: &'a Data,
    pub(crate) indices: Indices,
    pub(crate) _phantom: PhantomData<(IterType, IndexedType)>,
}

impl<'a, Data, Indices, VisitedSet> SelectIndicesIter<'a, Data, Indices, VisitedSet, Unindexed>
{
    pub fn indexed(self) -> SelectIndicesIter<'a, Data, Indices, VisitedSet, Indexed>
    {
        SelectIndicesIter {
            data: self.data,
            indices: self.indices,
            _phantom: Default::default(),
        }
    }
}

pub type SeqSelectIndicesIter<'a, Data, Indices, IndexedType> = SelectIndicesIter<'a, Data, Indices, Sequential, IndexedType>;
#[cfg(feature = "rayon")]
pub type ParSelectIndicesIter<'a, Data, Indices, IndexedType> = SelectIndicesIter<'a, Data, Indices, Parallel, IndexedType>;