use std::{
    slice::Iter,
    ops::{ IndexMut, Deref },
    marker::PhantomData,
};
use crate::OneToOne;

pub struct Unindexed;
pub struct Indexed;

pub struct SelectIndicesSliceIterMut<'a, Data, Idx, IndexedType>
where
    Data: ?Sized + IndexMut<Idx> + OneToOne,
    Idx: Sized + Copy,
{
    pub(crate) data: &'a mut Data,
    pub(crate) index_iter: Iter<'a, Idx>,
    pub(crate) _phantom: PhantomData<IndexedType>,
}

mod unindexed {
    use super::*;

    impl<'a, Data, Idx> Iterator for SelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne,
        Idx: Sized + Copy,
    {
        type Item = &'a mut Data::Output;

        fn next(&mut self) -> Option<Self::Item> {
            self.index_iter.next().map(|&index| {
                let ptr: *mut _ = self.data;
                unsafe { ptr.as_mut().unwrap().index_mut(index) }
            })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            return self.index_iter.size_hint()
        }
    }

    impl<'a, Data, Idx> DoubleEndedIterator for SelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne,
        Idx: Sized + Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.index_iter.next_back().map(|&index| {
                let ptr: *mut _ = self.data;
                unsafe { ptr.as_mut().unwrap().index_mut(index) }
            })
        }
    }

    impl<'a, Data, Idx> ExactSizeIterator for SelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne,
        Idx: Sized + Copy,
    {}
}
pub use unindexed::*;

mod indexed {
    use super::*;

    impl<'a, Data, Idx> Iterator for SelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne,
        Idx: Sized + Copy,
    {
        type Item = (Idx, &'a mut Data::Output);

        fn next(&mut self) -> Option<Self::Item> {
            self.index_iter.next().map(|index| {
                let ptr: *mut _ = self.data;
                (
                    *index.deref(),
                    unsafe { ptr.as_mut().unwrap().index_mut(*index.deref()) }
                )
            })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.index_iter.size_hint()
        }
    }

    impl<'a, Data, Idx> DoubleEndedIterator for SelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne,
        Idx: Sized + Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.index_iter.next_back().map(|index| {
                let ptr: *mut _ = self.data;
                (
                    *index.deref(),
                    unsafe { ptr.as_mut().unwrap().index_mut(*index.deref()) }
                )
            })
        }
    }

    impl<'a, Data, Idx> ExactSizeIterator for SelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne,
        Idx: Sized + Copy,
    {}

    impl<'a, Data, Idx> From<SelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>> for SelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    where
        Data: ?Sized + IndexMut<Idx> + OneToOne,
        Idx: Sized + Copy,
    {
        fn from(iter: SelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>) -> Self {
            SelectIndicesSliceIterMut {
                data: iter.data,
                index_iter: iter.index_iter,
                _phantom: Default::default(),
            }
        }
    }
}
pub use indexed::*;

#[test]
#[ignore]
fn speed_test()
{
    use rand::prelude::*;
    use std::time::Instant;
    use crate::mutable::SelectIndicesMut;
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
}

impl<'a, Data, Idx> SelectIndicesSliceIterMut<'a, Data, Idx, Unindexed>
where
    Data: ?Sized + IndexMut<Idx> + OneToOne,
    Idx: Sized + Copy,
{
    pub fn indexed(self) -> SelectIndicesSliceIterMut<'a, Data, Idx, Indexed>
    {
        self.into()
    }
}