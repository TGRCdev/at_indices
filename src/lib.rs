//! This crate provides multiple iterators for multiple shared/exclusive 
//! references within slices, Vecs and other contiguous data structures.
//! The two basic functions it adds are [select_indices](SelectIndices::select_indices)
//! and [select_indices_mut](SelectIndicesMut::select_indices_mut), which let you provide a list of
//! indices into a given slice, and get back an iterator over each shared
//! or exclusive reference with those indices.
//! 
//! If the `rayon` feature is enabled, 
//! [par_select_indices](rayon::SelectIndicesPar::par_select_indices) and 
//! [par_select_indices_mut](rayon::SelectIndicesParMut::par_select_indices_mut) 
//! are also provided for use with [rayon](https://docs.rs/rayon). The latter is extremely 
//! useful for efficiently mutating a `Vec` in multi-threaded contexts.
//! 
//! # Examples
//! ```
//! # fn main() {
//! use select_indices::prelude::*;
//! 
//! struct BankAccount {
//!     pub name: String,
//!     pub balance: f32,
//! }
//! 
//! let mut vec: Vec<BankAccount> = vec![
//!     BankAccount { name: "Joey Bag o' Donuts".to_string(), balance: 4.27 },
//!     BankAccount { name: "Henry Howard Roosevelt".to_string(), balance: 83.20 },
//!     BankAccount { name: "Jenny Jenson".to_string(), balance: 54.32 },
//!     BankAccount { name: "The Dude".to_string(), balance: -134.01 },
//!     // Assume there's like 300 of these
//! ];
//! 
//! vec.select_indices_mut(&[1, 3]).for_each(|account| {
//!     account.balance -= 20.00;
//!     println!("{} now has ${}", account.name, account.balance);
//! });
//! # }
//! ```

mod data;
use data::*;

mod iter;
pub use iter::*;

mod iter_mut;
pub use iter_mut::*;

/// Seek through a shared slice with a list of indices.
/// 
/// SelectIndices provides an iterator that can split a contiguous,
/// immutable slice of objects (`&[T]`) into individual, shared references (`&T`).
pub trait SelectIndices<'a>
{
    type SliceType: Sized;

    /// Creates an iterator on the slice that seeks through and returns
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
    fn select_indices(&'a self, indices: &'a [usize]) -> SelectIndicesIter<Self::SliceType>;

    /// Creates an iterator on the slice that seeks through and returns
    /// references to each element within the given set of indices
    /// 
    /// # Safety
    /// This method is safe as long as the indices passed are in-bounds and
    /// do not have duplicates. Violating either of these will cause undefined
    /// behavior.
    unsafe fn select_indices_unchecked(&'a self, indices: &'a [usize]) -> SelectIndicesIter<Self::SliceType>;
}

/// Seek through an exclusive slice with a list of indices.
/// 
/// SelectIndicesMut provides an iterator that can split a contiguous,
/// mutable slice of objects (`&mut [T]`) into individual, exclusive references (`&mut T`)
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
///     refs = data.select_indices_mut(&[0,1,2]).collect();
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
///     refs = data.select_indices_mut(&[0, 1, 2, 3, 4]).collect();
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
///     let mut refs: Vec<&mut i8> = data.select_indices_mut(&[3,4]).collect();
/// 
///     // refs is moved out of this thread
///     thread::spawn(move || *refs[0] = 99);
/// }
/// 
/// data[3] = 57; // Compile error: Assignment to borrowed data
/// ```
pub trait SelectIndicesMut<'a>
{
    type SliceType: Sized;

    /// Creates an iterator on the slice that seeks through and returns
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
    fn select_indices_mut(&'a mut self, indices: &'a [usize]) -> SelectIndicesIterMut<Self::SliceType>;

    /// Creates an iterator on the slice that seeks through and returns
    /// references to each element within the given set of indices
    /// 
    /// # Safety
    /// This method is safe as long as the indices passed are in-bounds and
    /// do not have duplicates. Violating either of these will cause undefined
    /// behavior and possibly create multiple exclusive references.
    unsafe fn select_indices_mut_unchecked(&'a mut self, indices: &'a [usize]) -> SelectIndicesIterMut<Self::SliceType>;
}

mod slice;
/// Additional traits and iterators for use with [rayon](https://docs.rs/rayon)
#[cfg(feature = "rayon")]
pub mod rayon;

pub mod prelude {
    pub use super::{SelectIndices, SelectIndicesMut};

    #[cfg(feature = "rayon")]
    pub use crate::rayon::prelude::*;
}