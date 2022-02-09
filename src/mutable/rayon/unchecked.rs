use crate::{
    mutable::iter::ParSelectIndicesUncheckedMutIter,
    indexed_type::{ Unindexed, Indexed },
    traits::OneToOne,
};
use rayon::{
    prelude::*,
    iter::plumbing::{ Consumer, UnindexedConsumer },
};
use std::ops::IndexMut;
use force_send_sync::Sync as ForceSync;

mod unindexed {
    use super::*;

    impl<'a, Data, Indices> ParallelIterator for ParSelectIndicesUncheckedMutIter<'a, Data, Indices, Unindexed>
    where
        Indices: ParallelIterator,
        Indices::Item: Copy,
        Data: Send + IndexMut<Indices::Item> + OneToOne,
        Data::Output: 'a + Send,
    {
        type Item = &'a mut Data::Output;

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>
        {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                &mut data[index]
            }).drive_unindexed(consumer)
        }
    }

    impl<'a, Data, Indices> IndexedParallelIterator for ParSelectIndicesUncheckedMutIter<'a, Data, Indices, Unindexed>
    where
        Indices: IndexedParallelIterator,
        Indices::Item: Copy,
        Data: Send + IndexMut<Indices::Item> + OneToOne,
        Data::Output: 'a + Send,
    {
        fn len(&self) -> usize {
            self.indices.len()
        }

        fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                &mut data[index]
            }).drive(consumer)
        }

        fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                &mut data[index]
            }).with_producer(callback)
        }
    }
}
mod indexed {
    use super::*;

    impl<'a, Data, Indices> ParallelIterator for ParSelectIndicesUncheckedMutIter<'a, Data, Indices, Indexed>
    where
        Indices: ParallelIterator,
        Indices::Item: Copy,
        Data: Send + IndexMut<Indices::Item> + OneToOne,
        Data::Output: 'a + Send,
    {
        type Item = (Indices::Item, &'a mut Data::Output);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>
        {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                (index, &mut data[index])
            }).drive_unindexed(consumer)
        }
    }

    impl<'a, Data, Indices> IndexedParallelIterator for ParSelectIndicesUncheckedMutIter<'a, Data, Indices, Indexed>
    where
        Indices: IndexedParallelIterator,
        Indices::Item: Copy,
        Data: Send + IndexMut<Indices::Item> + OneToOne,
        Data::Output: 'a + Send,
    {
        fn len(&self) -> usize {
            self.indices.len()
        }

        fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                (index, &mut data[index])
            }).drive(consumer)
        }

        fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.indices.map(|index| {
                let data = unsafe { (*ptr).as_mut().unwrap() };
                (index, &mut data[index])
            }).with_producer(callback)
        }
    }
}