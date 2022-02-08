mod one_to_one;
pub use one_to_one::*;

mod immutable;
pub use immutable::*;

mod mutable;
pub use mutable::*;

pub(crate) mod iter_type {
    pub struct Unindexed;
    pub struct Indexed;
}

pub mod prelude {
    pub use crate::{ SelectIndices, SelectIndicesMut };
    
    #[cfg(feature = "rayon")]
    pub use crate::{ ParSelectIndices, ParSelectIndicesMut };
}