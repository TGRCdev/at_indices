use crate::{ AtIndices, AtIndicesMut, AtIndicesIter, AtIndicesIterMut, AtIndicesData };

use std::collections::HashSet;

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