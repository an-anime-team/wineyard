#[cfg(feature = "network")]
pub use reqwest;

#[cfg(feature = "network")]
pub mod network;

#[cfg(feature = "archives")]
pub mod archives;

pub mod prelude {
    #[cfg(feature = "archives")]
    pub use super::archives::{
        Archive,
        ArchiveExtractionContext
    };
}
