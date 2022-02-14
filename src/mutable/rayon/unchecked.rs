use crate::{
    prelude::*,
    mutable::iter::ParSelectIndicesUncheckedMutIter,
    indexed_type::{ Unindexed, Indexed },
};
use rayon::{
    prelude::*,
    iter::plumbing::{ Consumer, UnindexedConsumer },
};
use force_send_sync::Sync as ForceSync;

mod unindexed {
    use super::*;

    impl<'a, Data, Indices> ParallelIterator for ParSelectIndicesUncheckedMutIter<'a, Data, Indices, Unindexed>
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
        Data: Send + OneToOne<Indices::Item>,
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
        Data: Send + OneToOne<Indices::Item>,
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
        Data: Send + OneToOne<Indices::Item>,
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

#[allow(unused_macros)]
macro_rules! time_iter {
    ($constructor:expr) => {{
        use std::time::Duration;
        const TEST_COUNT: u32 = 1000;

        let mut construct_avg = Duration::from_secs(0);
        let mut construct_min = Duration::from_secs(u64::MAX);
        let mut construct_max = Duration::from_secs(u64::MIN);

        let mut iter_avg = Duration::from_secs(0);
        let mut iter_min = Duration::from_secs(u64::MAX);
        let mut iter_max = Duration::from_secs(u64::MIN);

        fn print_durations(avg: Duration, min: Duration, max: Duration)
        {
            println!("\tMinimum: {} micros ({} millis)", min.as_micros(), (avg.as_micros() as f64) / 1000.0);
            println!("\tMaximum: {} micros ({} millis)", max.as_micros(), (max.as_micros() as f64) / 1000.0);
            println!("\tAverage: {} micros ({} millis)", avg.as_micros(), (avg.as_micros() as f64) / 1000.0);
        }

        for _test in 0..TEST_COUNT
        {
            let construct_start = Instant::now();
            let iter = $constructor;
            let construct_end = Instant::now();
            let construct_duration = construct_end - construct_start;

            construct_avg += construct_duration;
            construct_min = construct_min.min(construct_duration);
            construct_max = construct_max.max(construct_duration);

            let iter_start = Instant::now();
            iter.for_each(|val| *val += rand::thread_rng().gen::<i8>() as i32);
            let iter_end = Instant::now();
            let iter_duration = iter_end - iter_start;

            iter_avg += iter_duration;
            iter_min = iter_min.min(iter_duration);
            iter_max = iter_max.max(iter_duration);
        }

        construct_avg /= TEST_COUNT;
        iter_avg /= TEST_COUNT;

        println!("Construction durations:");
        print_durations(construct_avg, construct_min, construct_max);
        
        println!("Iteration durations:");
        print_durations(iter_avg, iter_min, iter_max);
        
        println!("Total durations:");
        print_durations(
            construct_avg + iter_avg,
            construct_min + iter_min,
            construct_max + iter_max,
        );
    }}
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
    println!();

    println!("Generic Iterator");
    time_iter!(data.par_select_with_iter_mut(indices.par_iter().cloned()));
    println!();

    println!("Slice Iterator");
    time_iter!(data.par_select_indices_mut(&indices));
    println!();

    println!("Unchecked Generic");
    time_iter!(unsafe { data.par_select_with_iter_mut_unchecked(indices.par_iter().cloned()) });
    println!();

    println!("Unchecked Slice");
    time_iter!(unsafe { data.par_select_indices_mut_unchecked(&indices) });
    println!();
}