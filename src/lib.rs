pub mod immutable;

pub mod mutable;

pub(crate) mod indexed_type {
    pub struct Unindexed;
    pub struct Indexed;
}

pub(crate) mod iter_type {
    pub struct Sequential;
    #[cfg(feature = "rayon")]
    pub struct Parallel;
}

pub mod prelude {
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
}