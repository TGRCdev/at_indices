use rayon::{
    prelude::*,
    iter::{
        plumbing::{ UnindexedConsumer, Consumer },
        Cloned,
    },
    slice::Iter,
};
use std::{
    ops::IndexMut,
    hash::Hash,
};
use crate::OneToOne;
use force_send_sync::Sync as ForceSync;
use std::{
    collections::HashSet,
    sync::Mutex,
    fmt::Debug,
};

pub struct ParSelectIndicesIterMut<'a, Data, Iter>
where
    Data: ?Sized,
    Iter: ParallelIterator,
{
    data: &'a mut Data,
    index_iter: Iter,
    past_indices: Mutex<HashSet<Iter::Item>>,
}

impl<'a, Data, Iter> ParallelIterator for ParSelectIndicesIterMut<'a, Data, Iter>
where
    Iter: ParallelIterator + Sync,
    Iter::Item: Copy + Eq + Hash + Debug,
    Data: ?Sized + IndexMut<Iter::Item> + Sync + Send + OneToOne,
    Data::Output: 'a + Sync + Send + Sized,
{
    type Item = &'a mut Data::Output;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item> {
        let data: ForceSync<*mut Data> = unsafe { ForceSync::new(self.data) };
        let past_indices = self.past_indices;
        self.index_iter
            .map(|index| {
                let mut past_indices = past_indices.lock().unwrap();
                if !past_indices.insert(index)
                {
                    panic!("select_indices_mut was passed duplicate indices!");
                }

                let data: &mut Data = unsafe { &mut *data.clone() };
                &mut data[index]
            })
            .drive_unindexed(consumer)
    }
}

impl<'a, Data, Iter> IndexedParallelIterator for ParSelectIndicesIterMut<'a, Data, Iter>
where
    Iter: IndexedParallelIterator + Sync,
    Iter::Item: Copy + Eq + Hash + Debug,
    Data: ?Sized + IndexMut<Iter::Item> + Sync + Send + OneToOne,
    Data::Output: 'a + Sync + Send + Sized,
{
    fn len(&self) -> usize {
        self.index_iter.len()
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        let data: ForceSync<*mut Data> = unsafe { ForceSync::new(self.data) };
        let past_indices = self.past_indices;
        self.index_iter
            .map(|index| {
                let mut past_indices = past_indices.lock().unwrap();
                if !past_indices.insert(index)
                {
                    panic!("select_indices_mut was passed duplicate indices!");
                }

                let data: &mut Data = unsafe { &mut *data.clone() };
                &mut data[index]
            })
            .drive(consumer)
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        let data: ForceSync<*mut Data> = unsafe { ForceSync::new(self.data) };
        let past_indices = self.past_indices;
        self.index_iter
            .map(|index| {
                let mut past_indices = past_indices.lock().unwrap();
                if !past_indices.insert(index)
                {
                    panic!("select_indices_mut was passed duplicate indices!");
                }

                let data: &mut Data = unsafe { &mut *data.clone() };
                &mut data[index]
            })
            .with_producer(callback)
    }
}

pub trait ParSelectIndicesMut<'a>
where
    Self: 'a
{
    fn par_select_indices_mut<I>(&'a mut self, indices: &'a [I]) -> ParSelectIndicesIterMut<'a, Self, Cloned<Iter<'a, I>>>
    where
        I: Copy + Sync + Send,
        Self: IndexMut<I> + OneToOne,
    {
        ParSelectIndicesIterMut {
            data: self,
            index_iter: indices.par_iter().cloned(),
            past_indices: Default::default(),
        }
    }

    fn par_select_with_iter<Iter>(&'a mut self, index_iter: Iter) -> ParSelectIndicesIterMut<'a, Self, Iter::Iter>
    where
        Iter: IntoParallelIterator<Iter = Iter>,
        Iter::Iter: ParallelIterator,
        Iter::Item: Copy,
        Self: IndexMut<Iter::Item> + OneToOne
    {
        ParSelectIndicesIterMut {
            data: self,
            index_iter: index_iter.into_par_iter(),
            past_indices: Default::default(),
        }
    }
}

impl<'a, D> ParSelectIndicesMut<'a> for D
where
    D: ?Sized + 'a + OneToOne
{}

mod indexed {
    use super::*;

    pub struct ParSelectIndicesIndexedIterMut<'a, Data, Iter>(pub ParSelectIndicesIterMut<'a, Data, Iter>)
    where
        Iter: ParallelIterator,
        Data: ?Sized + OneToOne;
    
    impl<'a, Data, Iter> ParallelIterator for ParSelectIndicesIndexedIterMut<'a, Data, Iter>
    where
        Iter: ParallelIterator + Sync,
        Iter::Item: Copy + Eq + Hash + Debug,
        Data: ?Sized + IndexMut<Iter::Item> + Sync + Send + OneToOne,
        Data::Output: 'a + Sync + Send + Sized,
    {
        type Item = (Iter::Item, &'a mut Data::Output);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item> {
            let data: ForceSync<*mut Data> = unsafe { ForceSync::new(self.0.data) };
            let past_indices = self.0.past_indices;
            self.0.index_iter
            .map(|index| {
                let mut past_indices = past_indices.lock().unwrap();
                if !past_indices.insert(index)
                {
                    panic!("select_indices_mut was passed duplicate indices!");
                }
                
                let data: &mut Data = unsafe { &mut *data.clone() };
                (index, &mut data[index])
            })
            .drive_unindexed(consumer)
        }
    }
}
pub use indexed::*;

impl<'a, D, I> ParSelectIndicesIterMut<'a, D, I>
where
    D: ?Sized + OneToOne,
    I: ParallelIterator,
{
    pub fn indexed(self) -> ParSelectIndicesIndexedIterMut<'a, D, I>
    {
        return ParSelectIndicesIndexedIterMut(self)
    }
}