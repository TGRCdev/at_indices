use rayon::iter::plumbing::Producer;

use crate::{ SelectIndicesBase, SelectIndicesMutBase };

use std::ops::{ Index, IndexMut };

impl<'a, T: 'a + IndexMut<I, Output = O> + ?Sized + Send, I: Clone + Sync, O: 'a + Send> Producer for SelectIndicesMutBase<'a, T, I>
{
    type Item = &'a mut O;

    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        return self;
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let ptr: *mut _ = self.data;
        let refs: (&mut T, &mut T) = unsafe { (
            &mut *ptr.clone(),
            &mut *ptr
        ) };
        let split = self.indices.split_at(index); // TODO: Unchecked
        return (
            SelectIndicesMutBase {
                data: refs.0,
                indices: split.0,
                start: 0,
                end: split.0.len(),
            }.into(),
            SelectIndicesMutBase {
                data: refs.1,
                indices: split.1,
                start: 0,
                end: split.1.len()
            }.into(),
        );
    }
}

impl<'a, T: 'a + Index<I, Output = O> + ?Sized + Sync, I: Clone + Sync, O: 'a + Sync> Producer for SelectIndicesBase<'a, T, I>
{
    type Item = &'a O;
    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        return self;
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let split = self.indices.split_at(index);
        return (
            SelectIndicesBase {
                data: self.data,
                indices: split.0,
                start: 0,
                end: split.0.len(),
            }.into(),
            SelectIndicesBase {
                data: self.data,
                indices: split.1,
                start: 0,
                end: split.1.len()
            }.into(),
        )
    }
}