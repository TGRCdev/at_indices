use ndarray::{
    prelude::*,
    IntoDimension, NdIndex,
    Data, DataMut,
};
use crate::{
    prelude::*,
    SelectIndicesBase, SelectIndicesMutBase,
};
use std::{
    collections::HashSet,
    hash::Hash,
};

impl<'a, S, D> SelectIndices<'a, ArrayBase<S, D::Dim>, D> for ArrayBase<S, D::Dim>
where
    S: 'a + Data,
    D: 'a + IntoDimension + Clone + NdIndex<<D as IntoDimension>::Dim>,
{
    fn select_indices(&'a self, indices: &'a [D]) -> crate::SelectIndicesIter<ArrayBase<S, D::Dim>, D> {
        indices.iter().for_each(|i| {
            assert!(self.get(i.clone()).is_some(), "select_indices was given an invalid index! ({:?})", i);
        });

        unsafe { self.select_indices_unchecked(indices) }
    }

    unsafe fn select_indices_unchecked(&'a self, indices: &'a [D]) -> crate::SelectIndicesIter<ArrayBase<S, D::Dim>, D> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}

impl<'a, S, D> SelectIndicesMut<'a, ArrayBase<S, D::Dim>, D> for ArrayBase<S, D::Dim>
where
    S: 'a + DataMut,
    D: 'a + IntoDimension + Clone + NdIndex<<D as IntoDimension>::Dim> + Eq + Hash,
{
    fn select_indices_mut(&'a mut self, indices: &'a [D]) -> crate::SelectIndicesIterMut<ArrayBase<S, D::Dim>, D> {
        let indices_len = indices.len();
        
        // If indices is longer than the slice, either there are
        // duplicates, or some indices are out of bounds.
        assert!(indices.len() <= self.len(),
            "select_indices_mut was passed more indices than are possible without breaking mutability rules!"); 

        let mut indexset = HashSet::with_capacity(indices_len);
        // TODO: Safety checks without heap allocation

        indices.iter().for_each(|i| {
            assert!(self.get(i.clone()).is_some(), "select_indices was given an invalid index! ({:?})", i);
            assert!(indexset.insert(i.clone()), "select_indices_mut was passed a duplicate index!");
        });

        unsafe { self.select_indices_mut_unchecked(indices) }
    }

    unsafe fn select_indices_mut_unchecked(&'a mut self, indices: &'a [D]) -> crate::SelectIndicesIterMut<ArrayBase<S, D::Dim>, D> {
        SelectIndicesMutBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}