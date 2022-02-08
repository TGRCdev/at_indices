use std::{
    ops::IndexMut,
    marker::PhantomData,
};
use crate::{
    OneToOne,
    iter_type::{ Unindexed, Indexed },
};
use rayon::{
    prelude::*,
    iter::plumbing::{ UnindexedConsumer, Consumer },
    slice::Iter,
};
use force_send_sync::Sync as ForceSync;

pub struct ParSelectIndicesSliceIterMut<'a, Data, Idx, IndexedType>
where
    Data: ?Sized + IndexMut<Idx> + OneToOne + Send,
    Data::Output: Send,
    Idx: Sized + Copy + Sync,
{
    pub(crate) data: &'a mut Data,
    pub(crate) index_iter: Iter<'a, Idx>,
    pub(crate) _phantom: PhantomData<IndexedType>,
}

mod unindexed {
    use super::*;

    impl<'a, Data, Idx> ParallelIterator for ParSelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne + Send + Sync,
        Data::Output: Send,
        Idx: Sized + Copy + Sync,
    {
        type Item = &'a mut Data::Output;

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>
        {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.index_iter.map(|&index| {
                let ptr = ptr.clone();
                unsafe { ptr.as_mut().unwrap().index_mut(index) }
            })
            .drive_unindexed(consumer)
        }
    }

    impl<'a, Data, Idx> IndexedParallelIterator for ParSelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne + Send + Sync,
        Data::Output: Send,
        Idx: Sized + Copy + Sync,
    {
        fn len(&self) -> usize {
            self.index_iter.len()
        }

        fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.index_iter.map(|&index| {
                let ptr = ptr.clone();
                unsafe { ptr.as_mut().unwrap().index_mut(index) }
            })
            .drive(consumer)
        }

        fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.index_iter.map(|&index| {
                let ptr = ptr.clone();
                unsafe { ptr.as_mut().unwrap().index_mut(index) }
            })
            .with_producer(callback)
        }
    }
}
pub use unindexed::*;

mod indexed {
    use super::*;

    impl<'a, Data, Idx> ParallelIterator for ParSelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne + Send + Sync,
        Data::Output: Send,
        Idx: Sized + Copy + Sync + Send,
    {
        type Item = (Idx, &'a mut Data::Output);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>
        {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.index_iter.map(|&index| {
                let ptr = ptr.clone();
                (
                    index,
                    unsafe { ptr.as_mut().unwrap().index_mut(index) }
                )
            })
            .drive_unindexed(consumer)
        }
    }

    impl<'a, Data, Idx> IndexedParallelIterator for ParSelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne + Send + Sync,
        Data::Output: Send,
        Idx: Sized + Copy + Sync + Send,
    {
        fn len(&self) -> usize {
            self.index_iter.len()
        }

        fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.index_iter.map(|&index| {
                let ptr = ptr.clone();
                (
                    index,
                    unsafe { ptr.as_mut().unwrap().index_mut(index) }
                )
            })
            .drive(consumer)
        }

        fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
            let ptr: ForceSync<*mut _> = unsafe { ForceSync::new(self.data) };
            self.index_iter.map(|&index| {
                let ptr = ptr.clone();
                (
                    index,
                    unsafe { ptr.as_mut().unwrap().index_mut(index) }
                )
            })
            .with_producer(callback)
        }
    }
}
pub use indexed::*;

impl<'a, Data, Idx> ParSelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>
where
    Data: ?Sized + IndexMut<Idx> + OneToOne + Send + Sync,
    Data::Output: Send,
    Idx: Sized + Copy + Sync + Send,
{
    pub fn indexed(self) -> ParSelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    {
        ParSelectIndicesSliceIterMut {
            data: self.data,
            index_iter: self.index_iter,
            _phantom: Default::default(),
        }
    }
}

#[test]
#[ignore]
fn speed_test()
{
    use rand::prelude::*;
    use std::time::Instant;
    use crate::prelude::*;
    use std::collections::HashSet;

    const DATA_LEN: usize = 100000;
    const INDICES_LEN: usize = 1000;

    let mut rng = rand::thread_rng();
    let mut data: Vec<i32> = Vec::with_capacity(DATA_LEN);
    data.resize_with(DATA_LEN, || rng.gen());

    let mut indices: HashSet<usize> = HashSet::with_capacity(INDICES_LEN);
    while indices.len() < INDICES_LEN
    {
        indices.insert(rng.gen_range(0..DATA_LEN));
    }
    let indices: Vec<usize> = indices.drain().collect();

    println!("Slice vs. Iter indices speed test");
    fn time_iter<'a, Iter>(construct_closure: impl FnOnce() -> Iter)
    where
        Iter: ParallelIterator<Item = &'a mut i32>
    {
        let construct_start = Instant::now();
        let iter = construct_closure();
        let construct_end = Instant::now();
        let construct_duration = construct_end - construct_start;
        println!("Construction duration: {} micros ({} millis)", construct_duration.as_micros(), construct_duration.as_millis());
        
        let iter_start = Instant::now();
        iter.for_each(|val| *val += rand::thread_rng().gen::<i8>() as i32);
        let iter_end = Instant::now();
        let iter_duration = iter_end - iter_start;
        println!("Iteration duration: {} micros ({} millis)", iter_duration.as_micros(), iter_duration.as_millis());

        let total_duration = construct_duration + iter_duration;
        println!("Total duration: {} micros ({} millis)", total_duration.as_micros(), total_duration.as_millis());
    }

    println!("Generic Iterator");
    time_iter(|| data.par_select_with_iter_mut(indices.par_iter().copied()));
    println!();

    println!("Slice Iterator");
    time_iter(|| data.par_select_indices_mut(&indices));
    println!();
}