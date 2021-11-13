use crate::{
    prelude::*,
    SelectIndicesIter, SelectIndicesIterMut, SelectIndicesBase
};

use std::collections::HashSet;

impl<'a, T: 'a> SelectIndices<'a> for [T]
{
    type SliceType = &'a [T];

    fn select_indices(&'a self, indices: &'a [usize]) -> SelectIndicesIter<Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            // TODO: Safety checks without heap allocation
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.select_indices_unchecked(indices) };
    }

    unsafe fn select_indices_unchecked(&'a self, indices: &'a [usize]) -> SelectIndicesIter<Self::SliceType> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}

impl<'a, T: 'a> SelectIndicesMut<'a> for [T]
{
    type SliceType = &'a mut [T];

    fn select_indices_mut(&'a mut self, indices: &'a [usize]) -> SelectIndicesIterMut<Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            // TODO: Safety checks without heap allocation
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.select_indices_mut_unchecked(indices) };
    }

    unsafe fn select_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> SelectIndicesIterMut<Self::SliceType> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
    
}

#[cfg(feature = "rayon")]
impl<'a, T: 'a + Send> SelectIndicesParMut<'a> for [T]
{
    type SliceType = &'a mut [T];

    #[inline(always)]
    fn par_select_indices_mut(&'a mut self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterMutPar<'a, Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            // TODO: Safety checks without heap allocation
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.par_select_indices_mut_unchecked(indices) };
    }

    unsafe fn par_select_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterMutPar<'a, Self::SliceType> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}

#[cfg(feature = "rayon")]
impl<'a, T: 'a + Send> SelectIndicesPar<'a> for [T]
{
    type SliceType = &'a [T];

    fn par_select_indices(&'a self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterPar<'a, Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            // TODO: Safety checks without heap allocation
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.par_select_indices_unchecked(indices) };
    }

    unsafe fn par_select_indices_unchecked(&'a self, indices: &'a [usize]) -> crate::rayon::SelectIndicesIterPar<'a, Self::SliceType> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}