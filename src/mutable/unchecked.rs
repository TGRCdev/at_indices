use crate::{
    prelude::*,
    indexed_type::{ Indexed, Unindexed },
};
use super::iter::SeqSelectIndicesUncheckedMutIter;

mod unindexed {
    use super::*;

    impl<'a, Data, Indices> Iterator for SeqSelectIndicesUncheckedMutIter<'a, Data, Indices, Unindexed>
    where
        Data: ?Sized + OneToOne<Indices::Item>,
        Data::Output: 'a,
        Indices: Iterator,
        Indices::Item: Sized + Copy,
    {
        type Item = &'a mut Data::Output;

        fn next(&mut self) -> Option<Self::Item> {
            self.indices.next().map(|index| {
                let ptr: *mut _ = self.data;
                unsafe { ptr.as_mut().unwrap().index_mut(index) }
            })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.indices.size_hint()
        }
    }

    impl<'a, Data, Indices> DoubleEndedIterator for SeqSelectIndicesUncheckedMutIter<'a, Data, Indices, Unindexed>
    where
        Data: ?Sized + OneToOne<Indices::Item>,
        Data::Output: 'a,
        Indices: DoubleEndedIterator,
        Indices::Item: Sized + Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.indices.next_back().map(|index| {
                let ptr: *mut _ = self.data;
                unsafe { ptr.as_mut().unwrap().index_mut(index) }
            })
        }
    }

    impl<'a, Data, Indices> ExactSizeIterator for SeqSelectIndicesUncheckedMutIter<'a, Data, Indices, Unindexed>
    where
        Data: ?Sized + OneToOne<Indices::Item>,
        Data::Output: 'a,
        Indices: Iterator,
        Indices::Item: Sized + Copy,
    {}
}
mod indexed {
    use super::*;

    impl<'a, Data, Indices> Iterator for SeqSelectIndicesUncheckedMutIter<'a, Data, Indices, Indexed>
    where
        Data: ?Sized + OneToOne<Indices::Item>,
        Data::Output: 'a,
        Indices: Iterator,
        Indices::Item: Sized + Copy,
    {
        type Item = (Indices::Item, &'a mut Data::Output);

        fn next(&mut self) -> Option<Self::Item> {
            self.indices.next().map(|index| {
                let ptr: *mut _ = self.data;
                (
                    index,
                    unsafe { ptr.as_mut().unwrap().index_mut(index) }
                )
            })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.indices.size_hint()
        }
    }

    impl<'a, Data, Indices> DoubleEndedIterator for SeqSelectIndicesUncheckedMutIter<'a, Data, Indices, Indexed>
    where
        Data: ?Sized + OneToOne<Indices::Item>,
        Data::Output: 'a,
        Indices: DoubleEndedIterator,
        Indices::Item: Sized + Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.indices.next_back().map(|index| {
                let ptr: *mut _ = self.data;
                (
                    index,
                    unsafe { ptr.as_mut().unwrap().index_mut(index) }
                )
            })
        }
    }

    impl<'a, Data, Indices> ExactSizeIterator for SeqSelectIndicesUncheckedMutIter<'a, Data, Indices, Indexed>
    where
        Data: ?Sized + OneToOne<Indices::Item>,
        Data::Output: 'a,
        Indices: ExactSizeIterator,
        Indices::Item: Sized + Copy,
    {}
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
    use crate::*;
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
    time_iter!(data.select_with_iter_mut(indices.iter().cloned()));
    println!();

    println!("Slice Iterator");
    time_iter!(data.select_indices_mut(&indices));
    println!();

    println!("Unchecked Generic");
    time_iter!(unsafe { data.select_with_iter_mut_unchecked(indices.iter().cloned()) });
    println!();

    println!("Unchecked Slice");
    time_iter!(unsafe { data.select_indices_mut_unchecked(&indices) });
    println!();
}