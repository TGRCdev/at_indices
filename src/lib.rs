#![feature(generic_associated_types)]

mod immutable;
pub use immutable::*;

pub mod prelude {
    pub use crate::SelectIndices;
}