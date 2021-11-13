use crate::{
    prelude::*,
    AtIndicesIter, AtIndicesIterMut, AtIndicesData
};

use std::collections::HashSet;

impl<'a, T: 'a> AtIndices<'a> for [T]
{
    type SliceType = &'a [T];

    fn at_indices(&'a self, indices: &'a [usize]) -> AtIndicesIter<Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.at_indices_unchecked(indices) };
    }

    unsafe fn at_indices_unchecked(&'a self, indices: &'a [usize]) -> AtIndicesIter<Self::SliceType> {
        AtIndicesData {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}

impl<'a, T: 'a> AtIndicesMut<'a> for [T]
{
    type SliceType = &'a mut [T];

    fn at_indices_mut(&'a mut self, indices: &'a [usize]) -> AtIndicesIterMut<Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.at_indices_mut_unchecked(indices) };
    }

    unsafe fn at_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> AtIndicesIterMut<Self::SliceType> {
        AtIndicesData {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
    
}

#[cfg(feature = "rayon-iters")]
impl<'a, T: 'a + Send> AtIndicesParMut<'a> for [T]
{
    type SliceType = &'a mut [T];

    #[inline(always)]
    fn par_at_indices_mut(&'a mut self, indices: &'a [usize]) -> crate::rayon::AtIndicesIterMutPar<'a, Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.par_at_indices_mut_unchecked(indices) };
    }

    unsafe fn par_at_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> crate::rayon::AtIndicesIterMutPar<'a, Self::SliceType> {
        AtIndicesData {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}

#[cfg(feature = "rayon-iters")]
impl<'a, T: 'a + Send> AtIndicesPar<'a> for [T]
{
    type SliceType = &'a [T];

    fn par_at_indices(&'a self, indices: &'a [usize]) -> crate::rayon::AtIndicesIterPar<'a, Self::SliceType> {
        { // Safety checks
            let mut indexset = HashSet::with_capacity(indices.len());
            let len = self.len();
            indices.iter().for_each(|&i| {
                assert!(i < len);
                assert!(indexset.insert(i));
            });
        }

        return unsafe { self.par_at_indices_unchecked(indices) };
    }

    unsafe fn par_at_indices_unchecked(&'a self, indices: &'a [usize]) -> crate::rayon::AtIndicesIterPar<'a, Self::SliceType> {
        AtIndicesData {
            data: self,
            indices,
            start: 0,
            end: indices.len()
        }.into()
    }
}