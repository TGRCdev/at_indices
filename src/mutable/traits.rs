use std::{
    ops::IndexMut,
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
};

/// This trait makes a number of guarantees to make sure that
/// multiple mutable index operations do not break XOR mutability
/// or cause undefined behavior.
/// 
/// # Safety
/// By declaring this trait for an [`IndexMut`] type, you
/// are asserting the following guarantees for your type:
/// 
/// 1. For every valid index, there is only one output.
/// 2. Invalid indices should panic and not return multiple references
///     to one object.
/// 2. When mutably indexed, the type never reads any other
///     indexable objects ([`HashMap`s](std::collection::HashMap) are not `OneToOne` because of this).
/// 3. When mutably indexed, the type does not mutate itself. It
///     should only return a mutable index to an object held by the
///     type.
pub unsafe trait OneToOne<Idx> : IndexMut<Idx> {}

unsafe impl<T> OneToOne<usize> for [T] {}
unsafe impl<T, const N: usize> OneToOne<usize> for [T; N] {}
unsafe impl<T> OneToOne<usize> for Vec<T> {}

#[cfg(feature = "ndarray")]
mod ndarray {
    use super::OneToOne;
    use ::ndarray::{
        prelude::*,
        Dimension, DataMut,
        NdIndex
    };

    unsafe impl<S, D, I> OneToOne<I> for ArrayBase<S, D>
    where
        S: DataMut,
        D: Dimension,
        I: NdIndex<D>,
    {}
}

/// Selectively iterate through a mutable collection
/// with a list of indices or an index iterator.
pub trait SelectIndicesMut<'a>
{
    /// Iterate through a collection with an iterator that produces indices,
    /// without checking for duplicate indices.
    /// 
    /// # Performance
    /// 
    /// This method will not check the indices before or during
    /// iteration, which makes it faster than the safe form.
    /// 
    /// # Safety
    /// This is safe if the indices produced are unique and do not
    /// violate [`OneToOne`] guarantees with the given collection.
    /// Otherwise, undefined behavior will occur and XOR mutability
    /// will be violated.
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

    /// Iterate through a collection with a slice of indices, without
    /// checking if the indices are unique.
    /// 
    /// # Performance
    /// 
    /// This method will not check the indices before or during
    /// iteration, which makes it faster than the safe form.
    /// 
    /// # Safety
    /// This is safe if the given indices are unique and do not
    /// violate [`OneToOne`] guarantees with the given collection.
    /// Otherwise, undefined behavior will occur and XOR mutability
    /// will be violated.
    unsafe fn select_indices_mut_unchecked<Idx>(&'a mut self, indices: &'a [Idx]) -> SeqSelectIndicesUncheckedMutIter<Self, Copied<Iter<'a, Idx>>, Unindexed>
    where
        Self:IndexMut<Idx>,
        Idx: Copy,
    {
        self.select_with_iter_mut_unchecked(indices.iter().copied())
    }

    /// Iterate through a collection with a slice of indices.
    /// 
    /// This method requires that the collection given implements [`OneToOne`]. For the
    /// unsafe form without this requirement, see
    /// [`select_indices_mut_unchecked`](SelectIndicesMut::select_indices_mut_unchecked).
    /// 
    /// # Performance
    /// 
    /// This method checks that all indices are unique before returning
    /// the iterator. Because of that, this method is slightly slower
    /// than [`select_with_iter_mut`](SelectIndicesMut::select_with_iter_mut),
    /// but the resulting iterator is slightly faster, because it does
    /// not need to check for duplicate indices at every index. Overall,
    /// this method is slightly faster.
    fn select_indices_mut<Idx>(&'a mut self, indices: &'a [Idx]) -> SeqSelectIndicesUncheckedMutIter<Self, Copied<Iter<'a, Idx>>, Unindexed>
    where
        Self: OneToOne<Idx>,
        Idx: Sized + Eq + Hash + Copy,
    {
        {
            let index_check: HashSet<Idx> = indices.iter().copied().collect();
            assert!(
                index_check.len() == indices.len(),
                "par_select_indices_mut was passed duplicate indices!",
            )
        }

        unsafe { self.select_with_iter_mut_unchecked(indices.iter().copied()) }
    }
    
    /// Iterate through a collection given an iterator that produces indices.
    /// 
    /// This method requires that the collection given implements [`OneToOne`]. For the
    /// unsafe form without this requirement, see
    /// [`select_with_iter_mut_unchecked`](SelectIndicesMut::select_with_iter_mut_unchecked).
    /// 
    /// # Performance
    /// 
    /// The iterator produced by this method maintains a HashSet of previously returned
    /// references to ensure that XOR mutability is not violated. If your indices are
    /// contained within a slice, consider using
    /// [`select_indices_mut`](SelectIndicesMut::select_indices_mut)
    /// for a faster and more efficient iterator.
    fn select_with_iter_mut<Indices>(&'a mut self, indices: Indices) -> SeqSelectIndicesMutIter<Self, Indices::IntoIter, Unindexed>
    where
        Indices: IntoIterator,
        Indices::Item: Copy + Hash + Eq,
        Self: OneToOne<Indices::Item>,
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
mod parallel {
    use std::{
        ops::IndexMut,
        hash::Hash,
        collections::HashSet,
    };
    use crate::{
        indexed_type::Unindexed,
        mutable::iter::{ ParSelectIndicesMutIter, ParSelectIndicesUncheckedMutIter },
    };

    use super::OneToOne;
    
    use ::rayon::{
        prelude::*,
        slice::Iter,
        iter::Copied,
    };
    
    /// Selectively iterate through a mutable collection
    /// with a list of indices or an index iterator.
    /// Parallel form of [`SelectIndicesMut`](crate::mutable::traits::SelectIndicesMut).
    pub trait ParSelectIndicesMut<'a>
    {
        /// Iterate through a collection with an iterator that produces indices,
        /// without checking for duplicate indices. Parallel form of
        /// [`select_with_iter_mut_unchecked`](crate::mutable::traits::SelectIndicesMut::select_with_iter_mut_unchecked).
        /// 
        /// # Performance
        /// 
        /// This method will not check the indices before or during
        /// iteration, which makes it faster than the safe form.
        /// 
        /// # Safety
        /// This is safe if the indices produced are unique and do not
        /// violate [`OneToOne`] guarantees with the given collection.
        /// Otherwise, undefined behavior will occur and XOR mutability
        /// will be violated.
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

        /// Iterate through a collection with a slice of indices, without
        /// checking if the indices are unique.
        /// 
        /// Parallel form of [`select_indices_mut_unchecked`](crate::mutable::traits::SelectIndicesMut::select_indices_mut_unchecked).
        /// 
        /// # Performance
        /// 
        /// This method will not check the indices before or during
        /// iteration, which makes it faster than the safe form.
        /// 
        /// # Safety
        /// This is safe if the given indices are unique and do not
        /// violate [`OneToOne`] guarantees with the given collection.
        /// Otherwise, undefined behavior will occur and XOR mutability
        /// will be violated.
        unsafe fn par_select_indices_mut_unchecked<Idx>(&'a mut self, indices: &'a [Idx]) -> ParSelectIndicesUncheckedMutIter<Self, Copied<Iter<'a, Idx>>, Unindexed>
        where
            Self: OneToOne<Idx>,
            Idx: Copy + Sync + Send,
        {
            self.par_select_with_iter_mut_unchecked(indices.into_par_iter().copied())
        }

        /// Iterate through a collection with a slice of indices.
        /// 
        /// This method requires that the collection given implements [`OneToOne`]. For the
        /// unsafe form without this requirement, see
        /// [`par_select_indices_mut_unchecked`](ParSelectIndicesMut::par_select_indices_mut_unchecked).
        /// 
        /// # Performance
        /// 
        /// This method checks that all indices are unique before returning
        /// the iterator. Because of that, this method of construction is slightly slower
        /// than [`par_select_with_iter_mut`](ParSelectIndicesMut::par_select_with_iter_mut),
        /// but the resulting iterator is much faster, because it does
        /// not need to sync to a mutex to check for duplicate indices. Overall,
        /// this method is much faster than [`par_select_with_iter_mut`](ParSelectIndicesMut::par_select_with_iter_mut),
        /// and should be used whenever possible.
        fn par_select_indices_mut<Idx>(&'a mut self, indices: &'a [Idx]) -> ParSelectIndicesUncheckedMutIter<Self, Copied<Iter<'a, Idx>>, Unindexed>
        where
            Self: OneToOne<Idx>,
            Idx: Copy + Hash + Eq + Sync + Send,
        {
            {
                let index_check: HashSet<Idx> = indices.iter().copied().collect();
                assert!(
                    index_check.len() == indices.len(),
                    "par_select_indices_mut was passed duplicate indices!",
                )
            }
            

            // Safety: We just checked that all indices are unique. As long as OneToOne has
            // not been erroneously implemented (OneToOne types should never produce the same output
            // for two indices), this should be safe.
            unsafe { self.par_select_with_iter_mut_unchecked(indices.into_par_iter().copied()) }
        }

        /// Iterate through a collection given an iterator that produces indices.
        /// 
        /// This method requires that the collection given implements [`OneToOne`]. For the
        /// unsafe form without this requirement, see
        /// [`select_with_iter_mut_unchecked`](crate::mutable::traits::SelectIndicesMut::select_with_iter_mut_unchecked).
        /// 
        /// # Performance
        /// 
        /// The iterator produced by this method maintains a [`Mutex`](std::sync::Mutex)-held [`HashSet`]
        /// of previously returned references to ensure that XOR mutability is
        /// not violated. This massively drains performance, so you might instead want to
        /// collect your indices into a [`Vec`], then use
        /// [`par_select_indices_mut`](ParSelectIndicesMut::par_select_indices_mut).
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
pub use self::parallel::ParSelectIndicesMut;