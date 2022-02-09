use std::{
    ops::{ Index, IndexMut },
    hash::Hash,
    collections::HashSet,
    slice::Iter,
    iter::Copied,
};
use crate::{
    indexed_type::Unindexed,
    mutable::iter::{
        SeqSelectIndicesMutIter, SeqSelectIndicesUncheckedMutIter,
    },
    immutable::iter::SeqSelectIndicesIter,
};
pub use crate::one_to_one::OneToOne;

#[cfg(feature = "rayon")]
use crate::{
    mutable::iter::{
        ParSelectIndicesMutIter, ParSelectIndicesUncheckedMutIter,
    },
    immutable::iter::ParSelectIndicesIter,
};

pub trait SelectIndices<'a> {
    fn select_with_iter<Indices>(&'a self, indices: Indices) -> SeqSelectIndicesIter<'a, Self, Indices::IntoIter, Unindexed>
    where
        Indices: IntoIterator,
        Indices::Item: Copy,
        Self: Index<Indices::Item>,
    {
        SeqSelectIndicesIter {
            data: self,
            indices: indices.into_iter(),
            _phantom: Default::default(),
        }
    }

    fn select_indices<Idx>(&'a self, indices: &'a [Idx]) -> SeqSelectIndicesIter<'a, Self, Copied<Iter<'a, Idx>>, Unindexed>
    where
        Self: Index<Idx>,
        Idx: Copy,
    {
        SeqSelectIndicesIter {
            data: self,
            indices: indices.into_iter().copied(),
            _phantom: Default::default(),
        }
    }
}

impl<D> SelectIndices<'_> for D
where
    D: ?Sized
{}

pub trait SelectIndicesMut<'a>
{
    unsafe fn select_with_iter_mut_unchecked<Indices>(&'a mut self, indices: Indices) -> SeqSelectIndicesUncheckedMutIter<Self, Indices::IntoIter, Unindexed>
    where
        Indices: IntoIterator,
        Indices::Item: Copy,
        Self: IndexMut<Indices::Item>,
    {
        SeqSelectIndicesUncheckedMutIter {
            data: self,
            indices: indices.into_iter(),
            visited_refs: (),
            _phantom: Default::default(),
        }
    }

    fn select_indices_mut<Idx>(&'a mut self, indices: &'a [Idx]) -> SeqSelectIndicesUncheckedMutIter<Self, Copied<Iter<'a, Idx>>, Unindexed>
    where
        Self: IndexMut<Idx> + OneToOne,
        Idx: Sized + Eq + Hash + Copy,
    {
        {
            let mut values_check = HashSet::with_capacity(indices.len());
            assert!(indices.iter().all(|index| values_check.insert(index)));
        }

        unsafe { self.select_with_iter_mut_unchecked(indices.into_iter().copied()) }
    }
    
    fn select_with_iter_mut<Indices>(&'a mut self, indices: Indices) -> SeqSelectIndicesMutIter<Self, Indices::IntoIter, Unindexed>
    where
        Indices: IntoIterator,
        Indices::Item: Copy + Hash + Eq,
        Self: IndexMut<Indices::Item>,
    {
        let index_iter = indices.into_iter();
        let size = index_iter.size_hint();
        let size = size.1.unwrap_or(size.0);
        SeqSelectIndicesMutIter {
            data: self,
            indices: index_iter,
            visited_refs: HashSet::with_capacity(size),
            _phantom: Default::default(),
        }
    }
}

impl<D> SelectIndicesMut<'_> for D
where
    D: ?Sized,
{}

#[cfg(feature = "rayon")]
mod rayon {
    use super::*;
    use ::rayon::{
        prelude::*,
        slice::Iter,
        iter::Copied,
    };

    pub trait ParSelectIndices<'a>
    {
        fn par_select_with_iter<Indices>(&'a self, indices: Indices) -> ParSelectIndicesIter<'a, Self, Indices::Iter, Unindexed>
        where
            Indices: IntoParallelIterator,
            Indices::Item: Copy,
            Self: IndexMut<Indices::Item>,
        {
            ParSelectIndicesIter {
                data: self,
                indices: indices.into_par_iter(),
                _phantom: Default::default(),
            }
        }

        fn par_select_indices<Idx>(&'a self, indices: &'a [Idx]) -> ParSelectIndicesIter<'a, Self, Copied<Iter<'a, Idx>>, Unindexed>
        where
            Self: Index<Idx>,
            Idx: Copy + Sync + Send,
        {
            ParSelectIndicesIter {
                data: self,
                indices: indices.into_par_iter().copied(),
                _phantom: Default::default(),
            }
        }
    }

    impl<D> ParSelectIndices<'_> for D
    where
        D: ?Sized
    {}

    pub trait ParSelectIndicesMut<'a>
    {
        unsafe fn par_select_with_iter_mut_unchecked<Indices>(&'a mut self, indices: Indices) -> ParSelectIndicesUncheckedMutIter<Self, Indices::Iter, Unindexed>
        where
            Indices: IntoParallelIterator,
            Indices::Item: Copy,
            Self: IndexMut<Indices::Item>,
        {
            ParSelectIndicesUncheckedMutIter {
                data: self,
                indices: indices.into_par_iter(),
                visited_refs: (),
                _phantom: Default::default(),
            }
        }

        unsafe fn par_select_indices_mut_unchecked<Idx>(&'a mut self, indices: &'a [Idx]) -> ParSelectIndicesUncheckedMutIter<Self, Copied<Iter<'a, Idx>>, Unindexed>
        where
            Self: IndexMut<Idx> + OneToOne,
            Idx: Copy + Sync + Send,
        {
            self.par_select_with_iter_mut_unchecked(indices.into_par_iter().copied())
        }

        fn par_select_indices_mut<Idx>(&'a mut self, indices: &'a [Idx]) -> ParSelectIndicesUncheckedMutIter<Self, Copied<Iter<'a, Idx>>, Unindexed>
        where
            Self: IndexMut<Idx> + OneToOne,
            Idx: Copy + Hash + Eq + Sync + Send,
        {
            {
                let mut values_check = HashSet::with_capacity(indices.len());
                assert!(indices.iter().all(|index| values_check.insert(index)));
            }
    
            // Safety: We just checked that all indices are unique. As long as OneToOne has
            // not been erroneously implemented (OneToOne should never produce the same output
            // for two indices), this should be safe.
            unsafe { self.par_select_with_iter_mut_unchecked(indices.into_par_iter().copied()) }
        }

        fn par_select_with_iter_mut<Indices>(&'a mut self, indices: Indices) -> ParSelectIndicesMutIter<Self, Indices::Iter, Unindexed>
        where
            Indices: IntoParallelIterator,
            Indices::Item: Copy,
            Self: IndexMut<Indices::Item>,
        {
            ParSelectIndicesMutIter {
                data: self,
                indices: indices.into_par_iter(),
                visited_refs: Default::default(),
                _phantom: Default::default(),
            }
        }
    }

    impl<D> ParSelectIndicesMut<'_> for D
    where
        D: ?Sized
    {}
}

#[cfg(feature = "rayon")]
pub use self::rayon::{ ParSelectIndices, ParSelectIndicesMut };