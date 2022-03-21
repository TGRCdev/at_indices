//! ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/TGRCdev/select_indices/main)
//! [![crates.io](https://img.shields.io/crates/v/select_indices.svg)](https://crates.io/crates/select_indices)
//! [![docs.rs](https://img.shields.io/docsrs/select_indices)](https://docs.rs/select_indices)
//! ![Crates.io](https://img.shields.io/crates/l/select_indices)
//!
//! `select_indices` is a crate that provides iterators for seeking through a slice with a pre-made list of indices. It can simplify the readability of code and, in some cases, increase performance.
//!
//! [Documentation](https://docs.rs/select_indices)<br>
//!
//! ```rust
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
//! ```
//!
//! There is also a `rayon` feature flag that provides ParallelIterator versions of `select_indices` iterators. In certain cases, these iterators can greatly improve performance over other methods of slice iteration.

mod immutable;

mod mutable;

pub(crate) mod indexed_type {
    pub struct Unindexed;
    pub struct Indexed;
}

pub(crate) mod iter_type {
    pub struct Sequential;
    #[cfg(feature = "rayon")]
    pub struct Parallel;
}

pub use crate::{
    immutable::traits::SelectIndices,
    mutable::traits::{
        OneToOne,
        SelectIndicesMut,
    },
};

#[cfg(feature = "rayon")]
pub use crate::{
    immutable::traits::ParSelectIndices,
    mutable::traits::ParSelectIndicesMut,
};

pub mod prelude {
    pub use super::*;
}