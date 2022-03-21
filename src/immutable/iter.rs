use std::marker::PhantomData;
use crate::indexed_type::{ Unindexed, Indexed };
use crate::iter_type::Sequential;
#[cfg(feature = "rayon")]
use crate::iter_type::Parallel;

/// Immutably iterates, with a list of indices, through an [`Index`](std::ops::Index) collection.
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
    /// Converts the iterator's return type from
    /// `&Item` to `(IndexType, &Item)`
    pub fn indexed(self) -> SelectIndicesIter<'a, Data, Indices, VisitedSet, Indexed>
    {
        SelectIndicesIter {
            data: self.data,
            indices: self.indices,
            _phantom: Default::default(),
        }
    }
}

/// Return type for [`select_indices`](crate::SelectIndices::select_indices).
pub type SeqSelectIndicesIter<'a, Data, Indices, IndexedType> = SelectIndicesIter<'a, Data, Indices, Sequential, IndexedType>;
#[cfg(feature = "rayon")]
/// Return type for [`par_select_indices`](crate::ParSelectIndices::par_select_indices).
pub type ParSelectIndicesIter<'a, Data, Indices, IndexedType> = SelectIndicesIter<'a, Data, Indices, Parallel, IndexedType>;