mod producer;

mod iter;
pub use iter::*;

mod iter_mut;
pub use iter_mut::*;

use std::ops::{ Index, IndexMut };

/// Seek asynchronously through a shared slice with a list of indices.
/// 
/// SelectIndicesPar provides a [ParallelIterator](https://docs.rs/rayon/1.5.1/rayon/iter/trait.ParallelIterator.html)
/// that can split a contiguous, immutable slice of objects
/// (`&[T]`) into individual, shared references (`&T`).
pub trait SelectIndicesPar<'a, T: 'a + Index<I, Output = O> + ?Sized + Sync, I: Clone + Sync, O: 'a + Sync>
{
    /// Creates a [ParallelIterator](https://docs.rs/rayon/1.5.1/rayon/iter/trait.ParallelIterator.html)
    /// on the slice that seeks through and returns
    /// references to each element within the given set of indices
    /// 
    /// # Panics
    /// This function will panic if any index is out of bounds,
    /// or if there are any duplicate indices.
    /// 
    /// # Safety
    /// The iterator returned by this method is guaranteed to give out unique,
    /// shared references to the elements referenced by `indices`, and these
    /// references can only be used while the original slice is not dropped.
    fn par_select_indices(&'a self, indices: &'a [I]) -> SelectIndicesIterPar<'a, T, I, O>;
    
    /// Creates a [ParallelIterator](https://docs.rs/rayon/1.5.1/rayon/iter/trait.ParallelIterator.html)
    /// on the slice that seeks through and returns
    /// references to each element within the given set of indices
    /// 
    /// # Safety
    /// This method is safe as long as the indices passed are in-bounds and
    /// do not have duplicates. Violating either of these will cause undefined
    /// behavior.
    unsafe fn par_select_indices_unchecked(&'a self, indices: &'a [I]) -> SelectIndicesIterPar<'a, T, I, O>;
}

/// Seek asynchronously through an exclusive slice with a list of indices.
/// 
/// SelectIndicesParMut provides a [ParallelIterator](https://docs.rs/rayon/1.5.1/rayon/iter/trait.ParallelIterator.html)
/// that can split a contiguous, mutable slice of objects (`&mut [T]`)
/// into individual, exclusive references (`&mut T`).
/// 
/// The lifetimes of the references are linked to the original slice,
/// so the iterator cannot produce dangling references.
/// ```compile_fail
/// # use select_indices::prelude::*;
/// let refs: Vec<&mut i8>; // Vec of mutable elements within 'data'
/// 
/// {
///     let mut data = vec![1, 2, 3, 4, 5];
/// 
///     // Collect mutable references
///     refs = data.par_select_indices_mut(&[0,1,2]).collect();
/// }
/// 
/// // Compiler error: 'data' was dropped, references are invalid
/// refs.into_iter().for_each(|x| *x += 1); 
/// ```
/// 
/// Mutable references collected from the iterator also cannot
/// violate the borrow checker.
/// 
/// ```compile_fail
/// # use select_indices::prelude::*;
/// let mut refs: Vec<&mut i8>;
/// let mut data = vec![1, 2, 3, 4, 5];
/// {
///     // first mutable borrow
///     refs = data.par_select_indices_mut(&[0, 1, 2, 3, 4]).collect();
/// }
/// 
/// data.sort(); // Compiler error: second mutable borrow
/// data[4] = 9; // Compiler error: second mutable borrow
/// *refs[4] = 65; // first borrow used here
/// ```
///
/// Lastly, collected references cannot violate the borrow checker
/// when sent to other threads (as far as I can tell).
/// ```compile_fail
/// # use select_indices::prelude::*;
/// # use std::thread;
/// let mut data = [1,2,3,4,5];
/// 
/// {
///     let mut refs: Vec<&mut i8> = data.par_select_indices_mut(&[3,4]).collect();
/// 
///     // refs is moved out of this thread
///     thread::spawn(move || *refs[0] = 99);
/// }
/// 
/// data[3] = 57; // Compile error: Assignment to borrowed data
/// ```
pub trait SelectIndicesParMut<'a, T: 'a + IndexMut<I, Output = O> + ?Sized + Send, I: Clone + Sync, O: 'a + Send>
{
    /// Creates a [ParallelIterator](https://docs.rs/rayon/1.5.1/rayon/iter/trait.ParallelIterator.html)
    /// on the slice that seeks through and returns
    /// references to each element within the given set of indices
    /// 
    /// # Panics
    /// This function will panic if any index is out of bounds,
    /// or if there are any duplicate indices.
    /// 
    /// # Safety
    /// The iterator returned by this method is guaranteed to give out unique,
    /// exclusive references to the elements referenced by `indices`, and these
    /// references can only be used while the original slice is not dropped.
    fn par_select_indices_mut(&'a mut self, indices: &'a [I]) -> SelectIndicesIterMutPar<'a, T, I, O>;
    
    /// Creates a [ParallelIterator](https://docs.rs/rayon/1.5.1/rayon/iter/trait.ParallelIterator.html)
    /// on the slice that seeks through and returns
    /// references to each element within the given set of indices
    /// 
    /// # Safety
    /// This method is safe as long as the indices passed are in-bounds and
    /// do not have duplicates. Violating either of these will cause undefined
    /// behavior and possibly create multiple exclusive references.
    unsafe fn par_select_indices_mut_unchecked(&'a mut self, indices: &'a [I]) -> SelectIndicesIterMutPar<'a, T, I, O>;
}

pub mod prelude {
    pub use super::{ SelectIndicesPar, SelectIndicesParMut };
}