mod one_to_one;

mod traits;

mod immutable;

mod mutable;

pub(crate) mod indexed_type {
    pub struct Unindexed;
    pub struct Indexed;
}

pub(crate) mod iter_type {
    pub struct Sequential;
    pub struct Parallel;
}

pub mod prelude {
    pub use crate::traits::{
        SelectIndices,
        SelectIndicesMut,
    };

    #[cfg(feature = "rayon")]
    pub use crate::traits::{
        ParSelectIndices,
        ParSelectIndicesMut
    };
}