use std::ops::IndexMut;

/// This trait makes a number of guarantees to make sure that
/// multiple mutable index operations do not break XOR mutability
/// or cause undefined behavior.
/// 
/// # Safety
/// By declaring this trait for an `IndexMut` type, you
/// are asserting the following guarantees for your type:
/// 
/// 1. For every valid index, there is only one output.
/// 2. Invalid indices should panic and not return multiple references
///     to one object.
/// 2. When mutably indexed, the type never reads any other
///     indexable objects (`HashMap`s are not OneToOne because of this).
/// 3. When mutably indexed, the type does not mutate itself. It
///     should only return a mutable index to an object held by the
///     type.
pub unsafe trait OneToOne<Idx> : IndexMut<Idx> {}

unsafe impl<T> OneToOne<usize> for [T] {}
unsafe impl<T, const N: usize> OneToOne<usize> for [T; N] {}
unsafe impl<T> OneToOne<usize> for Vec<T> {}

#[cfg(feature = "ndarray")]
mod ndarray {
    use super::OneToOne;
    use ::ndarray::{
        prelude::*,
        Dimension, DataMut,
        NdIndex
    };

    unsafe impl<S, D, I> OneToOne<I> for ArrayBase<S, D>
    where
        S: DataMut,
        D: Dimension,
        I: NdIndex<D>,
    {}
}