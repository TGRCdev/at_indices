#![feature(generic_associated_types)]

mod one_to_one;
pub use one_to_one::*;

mod immutable;
pub use immutable::*;

mod mutable;
pub use mutable::*;

pub mod prelude {
    pub use crate::{ SelectIndices, SelectIndicesMut };
}