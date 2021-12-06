use crate::{
    prelude::*,
    SelectIndicesIter, SelectIndicesIterMut, SelectIndicesBase, SelectIndicesMutBase
};

use std::collections::HashSet;

impl<'a, T: 'a> SelectIndices<'a, [T], usize> for [T]
{
    fn select_indices(&'a self, indices: &'a [usize]) -> SelectIndicesIter<[T], usize> {
        let len = self.len();
        
        indices.iter().for_each(|&i| {
            assert!(i < len, "select_indices was given an out-of-bounds index!");
        });

        return unsafe { self.select_indices_unchecked(indices) };
    }

    unsafe fn select_indices_unchecked(&'a self, indices: &'a [usize]) -> SelectIndicesIter<[T], usize> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}


impl<'a, T: 'a> SelectIndicesMut<'a, [T], usize> for [T]
{
    fn select_indices_mut(&'a mut self, indices: &'a [usize]) -> SelectIndicesIterMut<[T], usize> {
        let len = self.len();
        let indices_len = indices.len();

        // If indices is longer than the slice, either there are
        // duplicates, or some indices are out of bounds.
        assert!(indices_len <= len,
            "select_indices_mut was passed more indices than are possible without breaking mutability rules!"); 

        let mut indexset = HashSet::with_capacity(indices_len);
        // TODO: Safety checks without heap allocation
        
        indices.iter().for_each(|&i| {
            assert!(i < len, "select_indices_mut was passed an out-of-bounds index!");
            assert!(indexset.insert(i), "select_indices_mut was passed a duplicate index!");
        });

        return unsafe { self.select_indices_mut_unchecked(indices) };
    }

    unsafe fn select_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> SelectIndicesIterMut<[T], usize> {
        SelectIndicesMutBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
    
}

#[cfg(feature = "rayon")]
impl<'a, T: 'a + Send, I: Copy + Clone + PrimInt + ToPrimitive + Sync> SelectIndicesParMut<'a, I> for [T]
{
    type SliceType = &'a mut [T];

    fn par_select_indices_mut(&'a mut self, indices: &'a [I]) -> crate::rayon::SelectIndicesIterMutPar<'a, Self::SliceType, I> {
        SelectIndicesBase::<Self::SliceType, I>::safety_check(self, indices);

        return unsafe { self.par_select_indices_mut_unchecked(indices) };
    }

    unsafe fn par_select_indices_mut_unchecked(&'a mut self, indices: &'a [I]) -> crate::rayon::SelectIndicesIterMutPar<'a, Self::SliceType, I> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}

#[cfg(feature = "rayon")]
impl<'a, T: 'a + Sync, I: Copy + Clone + PrimInt + ToPrimitive + Sync> SelectIndicesPar<'a, I> for [T]
{
    type SliceType = &'a [T];

    fn par_select_indices(&'a self, indices: &'a [I]) -> crate::rayon::SelectIndicesIterPar<'a, Self::SliceType, I> {
        SelectIndicesBase::<Self::SliceType, I>::safety_check(self, indices);

        return unsafe { self.par_select_indices_unchecked(indices) };
    }

    unsafe fn par_select_indices_unchecked(&'a self, indices: &'a [I]) -> crate::rayon::SelectIndicesIterPar<'a, Self::SliceType, I> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}