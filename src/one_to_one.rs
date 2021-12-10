/// This trait is implemented for types whose `Index`/`IndexMut`
/// implementations guarantee that each index is associated with
/// only one output object. This is required to ensure thread
/// safety and XOR mutability rules with `SelectIndices`.
pub unsafe trait OneToOne {}

unsafe impl<T> OneToOne for [T] {}
unsafe impl<T, const N: usize> OneToOne for [T; N] {}

#[cfg(feature = "ndarray")]
mod ndarray {
    use super::OneToOne;
    use ndarray::{
        prelude::*,
        Dimension, RawData,
    };

    unsafe impl<S, D> OneToOne for ArrayBase<S, D>
    where
        S: RawData,
        D: Dimension,
    {}
}