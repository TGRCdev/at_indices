use rayon::{
    prelude::*,
    iter::plumbing::Producer
};
use std::slice::from_raw_parts_mut;

use crate::prelude::*;
use crate::AtIndicesData;

impl<'a, T: Send> Producer for AtIndicesData<'a, &'a mut [T]>
{
    type Item = &'a mut T;

    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        return self;
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let data_len = self.data.len();
        let ptr = self.data.as_mut_ptr();
        let refs = unsafe { (
            from_raw_parts_mut(ptr.clone(), data_len),
            from_raw_parts_mut(ptr, data_len)
        ) };
        let split = self.indices.split_at(index); // TODO: Unchecked
        return (
            AtIndicesData {
                data: refs.0,
                indices: split.0,
                start: 0,
                end: split.0.len(),
            }.into(),
            AtIndicesData {
                data: refs.1,
                indices: split.1,
                start: 0,
                end: split.1.len()
            }.into(),
        );
    }
}