use crate::{
    prelude::*,
    SelectIndicesIter, SelectIndicesIterMut, SelectIndicesBase
};

use num_traits::{ PrimInt, ToPrimitive };

impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> SelectIndices<'a, I> for [T]
{
    type SliceType = &'a [T];

    fn select_indices(&'a self, indices: &'a [I]) -> SelectIndicesIter<Self::SliceType, I> {
        SelectIndicesBase::<Self::SliceType, I>::safety_check(self, indices);

        return unsafe { self.select_indices_unchecked(indices) };
    }

    unsafe fn select_indices_unchecked(&'a self, indices: &'a [I]) -> SelectIndicesIter<Self::SliceType, I> {
        SelectIndicesBase {
            data: self,
            indices,
            start: 0,
            end: indices.len(),
        }.into()
    }
}

impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> SelectIndicesMut<'a, I> for [T]
{
    type SliceType = &'a mut [T];

    fn select_indices_mut(&'a mut self, indices: &'a [I]) -> SelectIndicesIterMut<Self::SliceType, I> {
        SelectIndicesBase::<Self::SliceType, I>::safety_check(self, indices);

        return unsafe { self.select_indices_mut_unchecked(indices) };
    }

    unsafe fn select_indices_mut_unchecked(&'a mut self, indices: &'a [I]) -> SelectIndicesIterMut<Self::SliceType, I> {
        SelectIndicesBase {
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