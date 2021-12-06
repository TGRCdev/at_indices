use crate::{
    prelude::*,
    SelectIndicesBase, SelectIndicesMutBase
};
use ndarray::{
    prelude::*,
    IntoDimension, NdIndex,
    Data, DataMut, RawData,
};
use std::{
    collections::HashSet,
    hash::Hash,
};

impl<'a, S, D> SelectIndicesPar<'a, ArrayBase<S, D::Dim>, D> for ArrayBase<S, D::Dim>
where
    S: 'a + Data + Sync,
    D: 'a + IntoDimension + Clone + NdIndex<<D as IntoDimension>::Dim> + Sync,
    <S as RawData>::Elem: Sync
{
    fn par_select_indices(&'a self, indices: &'a [D]) -> crate::rayon::SelectIndicesIterPar<'a, ArrayBase<S, D::Dim>, D> {
        indices.iter().for_each(|i| {
            assert!(self.get(i.clone()).is_some(), "select_indices was given an invalid index! ({:?})", i);
        });

        unsafe { self.par_select_indices_unchecked(indices) }
    }

    unsafe fn par_select_indices_unchecked(&'a self, indices: &'a [D]) -> crate::rayon::SelectIndicesIterPar<'a, ArrayBase<S, D::Dim>, D> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}

impl<'a, S, D> SelectIndicesParMut<'a, ArrayBase<S, D::Dim>, D> for ArrayBase<S, D::Dim>
where
    S: 'a + DataMut + Send,
    D: 'a + IntoDimension + Clone + NdIndex<<D as IntoDimension>::Dim> + Sync + Eq + Hash,
    <S as RawData>::Elem: Send,
{
    fn par_select_indices_mut(&'a mut self, indices: &'a [D]) -> crate::rayon::SelectIndicesIterMutPar<'a, ArrayBase<S, D::Dim>, D> {
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

        unsafe { self.par_select_indices_mut_unchecked(indices) }
    }

    unsafe fn par_select_indices_mut_unchecked(&'a mut self, indices: &'a [D]) -> crate::rayon::SelectIndicesIterMutPar<'a, ArrayBase<S, D::Dim>, D> {
        SelectIndicesMutBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}