use std::ops::IndexMut;
use super::iter::SeqSelectIndicesMutIter;
use crate::indexed_type::{ Unindexed, Indexed };

mod unindexed {
    use super::*;

    impl<'a, Data, Indices> Iterator for SeqSelectIndicesMutIter<'a, Data, Indices, Unindexed>
    where
        Indices: Iterator,
        Indices::Item: Copy,
        Data: ?Sized + IndexMut<Indices::Item>,
        Data::Output: 'a,
    {
        type Item = &'a mut Data::Output;

        fn next(&mut self) -> Option<Self::Item> {
            self.indices.next().map(|index| {
                let ptr: *mut _ = self.data;

                let val_ref = unsafe { ptr.as_mut().unwrap().index_mut(index) };
                let val_ptr = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(self.visited_refs.insert(val_ptr), "select_indices_mut was passed duplicate indices!");

                val_ref
            })
        }
    }

    impl<'a, Data, Indices> ExactSizeIterator for SeqSelectIndicesMutIter<'a, Data, Indices, Unindexed>
    where
        Indices: ExactSizeIterator,
        Indices::Item: Copy,
        Data: ?Sized + IndexMut<Indices::Item>,
        Data::Output: 'a,
    {}

    impl<'a, Data, Indices> DoubleEndedIterator for SeqSelectIndicesMutIter<'a, Data, Indices, Unindexed>
    where
        Indices: DoubleEndedIterator,
        Indices::Item: Copy,
        Data: ?Sized + IndexMut<Indices::Item>,
        Data::Output: 'a,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.indices.next_back().map(|index| {
                let ptr: *mut _ = self.data;

                let val_ref = unsafe { ptr.as_mut().unwrap().index_mut(index) };
                let val_ptr = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(self.visited_refs.insert(val_ptr), "select_indices_mut was passed duplicate indices!");

                val_ref
            })
        }
    }
}

mod indexed {
    use super::*;

    impl<'a, Data, Indices> Iterator for SeqSelectIndicesMutIter<'a, Data, Indices, Indexed>
    where
        Indices: Iterator,
        Indices::Item: Copy,
        Data: ?Sized + IndexMut<Indices::Item>,
        Data::Output: 'a,
    {
        type Item = (Indices::Item, &'a mut Data::Output);

        fn next(&mut self) -> Option<Self::Item> {
            self.indices.next().map(|index| {
                let ptr: *mut _ = self.data;

                let val_ref = unsafe { ptr.as_mut().unwrap().index_mut(index) };
                let val_ptr = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(self.visited_refs.insert(val_ptr), "select_indices_mut was passed duplicate indices!");

                (index, val_ref)
            })
        }
    }

    impl<'a, Data, Indices> ExactSizeIterator for SeqSelectIndicesMutIter<'a, Data, Indices, Indexed>
    where
        Indices: ExactSizeIterator,
        Indices::Item: Copy,
        Data: ?Sized + IndexMut<Indices::Item>,
        Data::Output: 'a,
    {}

    impl<'a, Data, Indices> DoubleEndedIterator for SeqSelectIndicesMutIter<'a, Data, Indices, Indexed>
    where
        Indices: DoubleEndedIterator,
        Indices::Item: Copy,
        Data: ?Sized + IndexMut<Indices::Item>,
        Data::Output: 'a,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.indices.next_back().map(|index| {
                let ptr: *mut _ = self.data;

                let val_ref = unsafe { ptr.as_mut().unwrap().index_mut(index) };
                let val_ptr = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(self.visited_refs.insert(val_ptr), "select_indices_mut was passed duplicate indices!");

                (index, val_ref)
            })
        }
    }
}