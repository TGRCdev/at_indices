#![feature(generic_associated_types)]

mod immutable;
pub use immutable::*;

mod mutable;
pub use mutable::*;

pub mod prelude {
    pub use crate::{ SelectIndices, SelectIndicesMut };
}