use std::{
    ops::Index,
    slice::Iter,
    iter::Copied,
};
use crate::{
    indexed_type::Unindexed,
    immutable::iter::SeqSelectIndicesIter,
};

/// Selectively iterate through a collection
/// with a list of indices or an index iterator.
/// 
/// SelectIndices does not require [`Index`] types to implement
/// [`OneToOne`](crate::prelude::OneToOne) because having multiple immutable references
/// to an object is valid in Rust.
pub trait SelectIndices<'a> {
    /// Iterate through a collection with an iterator that produces indices.
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

    /// Iterate through a collection with a slice of indices.
    /// 
    /// This is just an alias for [`data.select_with_iter(indices.iter().copied())`](SelectIndices::select_with_iter).
    fn select_indices<Idx>(&'a self, indices: &'a [Idx]) -> SeqSelectIndicesIter<'a, Self, Copied<Iter<'a, Idx>>, Unindexed>
    where
        Self: Index<Idx>,
        Idx: Copy,
    {
        self.select_with_iter(indices.iter().copied())
    }
}


impl<D> SelectIndices<'_> for D
where
    D: ?Sized
{}

#[cfg(feature = "rayon")]
mod parallel {
    use std::ops::Index;
    use crate::{
        indexed_type::Unindexed,
        immutable::iter::ParSelectIndicesIter,
    };
    use ::rayon::{
        prelude::*,
        slice::Iter,
        iter::Copied,
    };

    /// Selectively iterate through a collection
    /// with a list of indices or an index iterator.
    /// Parallel form of [`SelectIndices`](crate::prelude::SelectIndices).
    ///
    /// ParSelectIndices does not require [Index] types to implement
    /// [`OneToOne`](crate::prelude::OneToOne) because having multiple immutable references
    /// to an object is valid in Rust.
    pub trait ParSelectIndices<'a>
    {
        /// Iterate through a collection with an iterator that produces indices.
        /// 
        /// Parallel form of [`select_with_iter`](crate::prelude::SelectIndices::select_with_iter).
        fn par_select_with_iter<Indices>(&'a self, indices: Indices) -> ParSelectIndicesIter<'a, Self, Indices::Iter, Unindexed>
        where
            Indices: IntoParallelIterator,
            Indices::Item: Copy,
            Self: Index<Indices::Item>,
        {
            ParSelectIndicesIter {
                data: self,
                indices: indices.into_par_iter(),
                _phantom: Default::default(),
            }
        }

        /// Iterate through a collection with a slice of indices.
        /// 
        /// Parallel form of [`select_indices`](crate::prelude::SelectIndices::select_indices).
        /// Alias for [`self.par_select_with_iter(indices.into_par_iter().copied())`](ParSelectIndices::par_select_with_iter).
        fn par_select_indices<Idx>(&'a self, indices: &'a [Idx]) -> ParSelectIndicesIter<'a, Self, Copied<Iter<'a, Idx>>, Unindexed>
        where
            Self: Index<Idx>,
            Idx: Copy + Sync + Send,
        {
            self.par_select_with_iter(indices.into_par_iter().copied())
        }
    }

    impl<D> ParSelectIndices<'_> for D
    where
        D: ?Sized
    {}
}
#[cfg(feature = "rayon")]
pub use self::parallel::ParSelectIndices;