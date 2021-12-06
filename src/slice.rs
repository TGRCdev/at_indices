use crate::{
    prelude::*,
    SelectIndicesIter, SelectIndicesIterMut, SelectIndicesBase, SelectIndicesMutBase
};

use std::collections::HashSet;

fn slice_safety_check<T>(slice: &[T], indices: &[usize])
{
    let len = slice.len();
        
    indices.iter().for_each(|&i| {
        assert!(i < len, "select_indices was given an out-of-bounds index!");
    });
}

fn slice_safety_check_mut<T>(slice: &mut [T], indices: &[usize])
{
    let len = slice.len();
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
}

impl<'a, T: 'a> SelectIndices<'a, [T], usize> for [T]
{
    fn select_indices(&'a self, indices: &'a [usize]) -> SelectIndicesIter<[T], usize> {
        slice_safety_check(self, indices);

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
        slice_safety_check_mut(self, indices);

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
impl<'a, T: 'a + Send> SelectIndicesParMut<'a, [T], usize, T> for [T]
{
    fn par_select_indices_mut(&'a mut self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterMutPar<'a, [T], usize, T> {
        slice_safety_check_mut(self, indices);

        return unsafe { self.par_select_indices_mut_unchecked(indices) };
    }

    unsafe fn par_select_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterMutPar<'a, [T], usize, T> {
        SelectIndicesMutBase {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}

#[cfg(feature = "rayon")]
impl<'a, T: 'a + Sync> SelectIndicesPar<'a, [T], usize, T> for [T]
{
    fn par_select_indices(&'a self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterPar<'a, [T], usize, T> {
        slice_safety_check(self, indices);

        return unsafe { self.par_select_indices_unchecked(indices) };
    }

    unsafe fn par_select_indices_unchecked(&'a self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterPar<'a, [T], usize, T> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}