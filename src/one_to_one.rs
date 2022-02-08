/// This trait is implemented for types whose `Index`/`IndexMut`
/// implementations guarantee that each index is associated with
/// only one output object. This is required to ensure thread
/// safety and XOR mutability rules with `SelectIndices`.
/// 
/// # Safety
/// By declaring this trait for an Index or IndexMut type, you
/// are asserting that there is one, and only one, output for
/// every index. If this assertion is untrue, then using
/// it for select_indices iterators that require it WILL
/// break XOR mutability and cause undefined behavior.
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