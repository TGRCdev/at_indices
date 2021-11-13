mod producer;

mod iter;
pub use iter::*;

mod iter_mut;
pub use iter_mut::*;

pub trait AtIndicesPar<'a>
{
    type SliceType: Sized;

    fn par_at_indices(&'a self, indices: &'a [usize]) -> AtIndicesIterPar<'a, Self::SliceType>;
    unsafe fn par_at_indices_unchecked(&'a self, indices: &'a [usize]) -> AtIndicesIterPar<'a, Self::SliceType>;
}

pub trait AtIndicesParMut<'a>
{
    type SliceType: Sized;

    fn par_at_indices_mut(&'a mut self, indices: &'a [usize]) -> AtIndicesIterMutPar<'a, Self::SliceType>;
    unsafe fn par_at_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> AtIndicesIterMutPar<'a, Self::SliceType>;
}

pub mod prelude {
    pub use super::{ AtIndicesPar, AtIndicesParMut };
}