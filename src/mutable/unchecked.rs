use crate::{
    traits::OneToOne,
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
        Iter: Iterator<Item = &'a mut i32>
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
    time_iter(|| data.select_with_iter_mut(indices.iter().cloned()));
    println!();

    println!("Slice Iterator");
    time_iter(|| data.select_indices_mut(&indices));
    println!();

    println!("Unchecked Generic");
    time_iter(|| unsafe { data.select_with_iter_mut_unchecked(indices.iter().cloned()) });
    println!();

    println!("Unchecked Slice");
    time_iter(|| unsafe { data.select_indices_mut_unchecked(&indices) });
    println!();
}