#[cfg(feature = "network")]
pub mod network;

#[cfg(feature = "archives")]
pub mod archives;

#[cfg(feature = "hashes")]
pub mod hashes;

#[cfg(any(
    feature = "network",
    feature = "archives",
    feature = "hashes"
))]
pub mod export {
    //! Re-exports of core library dependencies.

    #[cfg(feature = "network")]
    pub mod network {
        //! Re-exports of the `network` feature dependencies.

        pub use tokio;
        pub use reqwest;
    }

    #[cfg(feature = "hashes")]
    pub mod hashes {
        //! Re-exports of the `hashes` feature dependencies.

        #[cfg(feature = "hashes-seahash")]
        pub use seahash;

        #[cfg(feature = "hashes-crc32")]
        pub use crc32fast as crc32;

        #[cfg(feature = "hashes-crc32c")]
        pub use crc32c;

        #[cfg(feature = "hashes-xxh")]
        pub use xxhash_rust as xxh;

        #[cfg(feature = "hashes-md5")]
        pub use md5;

        #[cfg(feature = "hashes-sha1")]
        pub use sha1;

        #[cfg(feature = "hashes-sha2")]
        pub use sha2;

        #[cfg(feature = "hashes-sha3")]
        pub use sha3;

        #[cfg(feature = "hashes-blake2")]
        pub use blake2;

        #[cfg(feature = "hashes-blake3")]
        pub use blake3;
    }
}
