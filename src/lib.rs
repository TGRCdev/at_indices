mod data;
pub use data::*;

mod iter;
pub use iter::*;

mod iter_mut;
pub use iter_mut::*;

pub trait AtIndices<'a>
{
    type SliceType: Sized;

    fn at_indices(&'a self, indices: &'a [usize]) -> AtIndicesIter<Self::SliceType>;
    unsafe fn at_indices_unchecked(&'a self, indices: &'a [usize]) -> AtIndicesIter<Self::SliceType>;
}

pub trait AtIndicesMut<'a>
{
    type SliceType: Sized;

    fn at_indices_mut(&'a mut self, indices: &'a [usize]) -> AtIndicesIterMut<Self::SliceType>;
    unsafe fn at_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> AtIndicesIterMut<Self::SliceType>;
}


mod slice;