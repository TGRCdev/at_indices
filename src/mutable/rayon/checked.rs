use crate::{
    mutable::iter::ParSelectIndicesMutIter,
    indexed_type::{ Unindexed, Indexed },
    traits::OneToOne,
};
use rayon::{
    prelude::*,
    iter::plumbing::{ Consumer, UnindexedConsumer },
};
use force_send_sync::Sync as ForceSync;

mod unindexed {
    use super::*;

    impl<'a, Data, Indices> ParallelIterator for ParSelectIndicesMutIter<'a, Data, Indices, Unindexed>
    where
        Indices: ParallelIterator,
        Indices::Item: Copy,
        Data: Send + OneToOne<Indices::Item>,
        Data::Output: 'a + Send,
    {
        type Item = &'a mut Data::Output;

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>
        {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            let visited_refs = self.visited_refs;
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                
                let val_ref = data.index_mut(index);

                let val_ptr_num = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(visited_refs.lock().unwrap().insert(val_ptr_num), "par_select_indices_mut was passed duplicate indices!");

                val_ref
            }).drive_unindexed(consumer)
        }
    }

    impl<'a, Data, Indices> IndexedParallelIterator for ParSelectIndicesMutIter<'a, Data, Indices, Unindexed>
    where
        Indices: IndexedParallelIterator,
        Indices::Item: Copy,
        Data: Send + OneToOne<Indices::Item>,
        Data::Output: 'a + Send,
    {
        fn len(&self) -> usize {
            self.indices.len()
        }

        fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            let visited_refs = self.visited_refs;
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                
                let val_ref = data.index_mut(index);

                let val_ptr_num = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(visited_refs.lock().unwrap().insert(val_ptr_num), "par_select_indices_mut was passed duplicate indices!");

                val_ref
            }).drive(consumer)
        }

        fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            let visited_refs = self.visited_refs;
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                
                let val_ref = data.index_mut(index);

                let val_ptr_num = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(visited_refs.lock().unwrap().insert(val_ptr_num), "par_select_indices_mut was passed duplicate indices!");

                val_ref
            }).with_producer(callback)
        }
    }
}
mod indexed {
    use super::*;

    impl<'a, Data, Indices> ParallelIterator for ParSelectIndicesMutIter<'a, Data, Indices, Indexed>
    where
        Indices: ParallelIterator,
        Indices::Item: Copy,
        Data: Send + OneToOne<Indices::Item>,
        Data::Output: 'a + Send,
    {
        type Item = (Indices::Item, &'a mut Data::Output);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>
        {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            let visited_refs = self.visited_refs;
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                
                let val_ref = data.index_mut(index);

                let val_ptr_num = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(visited_refs.lock().unwrap().insert(val_ptr_num), "par_select_indices_mut was passed duplicate indices!");

                (index, val_ref)
            }).drive_unindexed(consumer)
        }
    }

    impl<'a, Data, Indices> IndexedParallelIterator for ParSelectIndicesMutIter<'a, Data, Indices, Indexed>
    where
        Indices: IndexedParallelIterator,
        Indices::Item: Copy,
        Data: Send + OneToOne<Indices::Item>,
        Data::Output: 'a + Send,
    {
        fn len(&self) -> usize {
            self.indices.len()
        }

        fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            let visited_refs = self.visited_refs;
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                
                let val_ref = data.index_mut(index);

                let val_ptr_num = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(visited_refs.lock().unwrap().insert(val_ptr_num), "par_select_indices_mut was passed duplicate indices!");

                (index, val_ref)
            }).drive(consumer)
        }

        fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            let visited_refs = self.visited_refs;
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                
                let val_ref = data.index_mut(index);

                let val_ptr_num = (val_ref as *mut Data::Output).cast::<*mut ()>() as usize;

                assert!(visited_refs.lock().unwrap().insert(val_ptr_num), "par_select_indices_mut was passed duplicate indices!");

                (index, val_ref)
            }).with_producer(callback)
        }
    }
}