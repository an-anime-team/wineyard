#[allow(unused_imports)]
use std::io::Write;

use super::algorithm::HashAlgorithm;

#[allow(non_camel_case_types)]
pub enum Hasher {
    #[cfg(feature = "hashes-seahash")]
    Seahash(seahash::SeaHasher),

    #[cfg(feature = "hashes-crc32")]
    Crc32(crc32fast::Hasher),

    #[cfg(feature = "hashes-crc32c")]
    Crc32c(crc32c::Crc32cHasher),

    #[cfg(feature = "hashes-siphash")]
    Siphash_1_3_64(siphasher::sip::SipHasher13),

    #[cfg(feature = "hashes-siphash")]
    Siphash_1_3_128(siphasher::sip128::SipHasher13),

    #[cfg(feature = "hashes-siphash")]
    Siphash_2_4_64(siphasher::sip::SipHasher24),

    #[cfg(feature = "hashes-siphash")]
    Siphash_2_4_128(siphasher::sip128::SipHasher24),

    #[cfg(feature = "hashes-xxh")]
    Xxh32(xxhash_rust::xxh32::Xxh32),

    #[cfg(feature = "hashes-xxh")]
    Xxh64(xxhash_rust::xxh64::Xxh64),

    #[cfg(feature = "hashes-xxh")]
    Xxh3_64(xxhash_rust::xxh3::Xxh3),

    #[cfg(feature = "hashes-xxh")]
    Xxh3_128(xxhash_rust::xxh3::Xxh3),

    #[cfg(feature = "hashes-md5")]
    Md5(md5::Md5),

    #[cfg(feature = "hashes-sha1")]
    Sha1(sha1::Sha1),

    #[cfg(feature = "hashes-sha2")]
    Sha2_224(sha2::Sha224),

    #[cfg(feature = "hashes-sha2")]
    Sha2_256(sha2::Sha256),

    #[cfg(feature = "hashes-sha2")]
    Sha2_384(sha2::Sha384),

    #[cfg(feature = "hashes-sha2")]
    Sha2_512(sha2::Sha512),

    #[cfg(feature = "hashes-sha2")]
    Sha2_512_224(sha2::Sha512_224),

    #[cfg(feature = "hashes-sha2")]
    Sha2_512_256(sha2::Sha512_256)
}

impl Hasher {
    /// Create new hasher from the algorithm.
    pub fn new(algorithm: impl Into<HashAlgorithm>) -> Self {
        match algorithm.into() {
            #[cfg(feature = "hashes-seahash")]
            HashAlgorithm::Seahash => Self::Seahash(Default::default()),

            #[cfg(feature = "hashes-crc32")]
            HashAlgorithm::Crc32 => Self::Crc32(Default::default()),

            #[cfg(feature = "hashes-crc32c")]
            HashAlgorithm::Crc32c => Self::Crc32c(Default::default()),

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_1_3_64 => Self::Siphash_1_3_64(Default::default()),

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_1_3_128 => Self::Siphash_1_3_128(Default::default()),

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_2_4_64 => Self::Siphash_2_4_64(Default::default()),

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_2_4_128 => Self::Siphash_2_4_128(Default::default()),

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh32 => Self::Xxh32(Default::default()),

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh64 => Self::Xxh64(Default::default()),

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh3_64 => Self::Xxh3_64(Default::default()),

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh3_128 => Self::Xxh3_128(Default::default()),

            #[cfg(feature = "hashes-md5")]
            HashAlgorithm::Md5 => Self::Md5(Default::default()),

            #[cfg(feature = "hashes-sha1")]
            HashAlgorithm::Sha1 => Self::Sha1(Default::default()),

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_224 => Self::Sha2_224(Default::default()),

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_256 => Self::Sha2_256(Default::default()),

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_384 => Self::Sha2_384(Default::default()),

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_512 => Self::Sha2_512(Default::default()),

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_512_224 => Self::Sha2_512_224(Default::default()),

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_512_256 => Self::Sha2_512_256(Default::default())
        }
    }

    /// Create new hasher from the algorithm and seed bytes.
    ///
    /// Big endian seeds are expected. If seed is different from the algorithm's
    /// seed size - it will be automatically converted to appropriate size.
    ///
    /// For algorithms with no native seed support it will be written to the
    /// hasher itself, basically prepending to the input data.
    pub fn with_seed(
        algorithm: impl Into<HashAlgorithm>,
        seed: impl AsRef<[u8]>
    ) -> Self {
        fn get_seed<const SIZE: usize>(seed: impl AsRef<[u8]>) -> [u8; SIZE] {
            let mut output = [0; SIZE];

            let seed = seed.as_ref();
            let len = seed.len();

            for i in 0..len {
                output[i % SIZE] ^= seed[i];
            }

            output
        }

        match algorithm.into() {
            #[cfg(feature = "hashes-seahash")]
            HashAlgorithm::Seahash => {
                let seed = get_seed::<32>(seed);

                let mut k1 = [0; 8];
                let mut k2 = [0; 8];
                let mut k3 = [0; 8];
                let mut k4 = [0; 8];

                k1.copy_from_slice(&seed[0..8]);
                k2.copy_from_slice(&seed[8..16]);
                k3.copy_from_slice(&seed[16..24]);
                k4.copy_from_slice(&seed[24..32]);

                let hasher = seahash::SeaHasher::with_seeds(
                    u64::from_be_bytes(k1),
                    u64::from_be_bytes(k2),
                    u64::from_be_bytes(k3),
                    u64::from_be_bytes(k4)
                );

                Self::Seahash(hasher)
            }

            #[cfg(feature = "hashes-crc32")]
            HashAlgorithm::Crc32 => {
                let hasher = crc32fast::Hasher::new_with_initial(
                    u32::from_be_bytes(get_seed(seed))
                );

                Self::Crc32(hasher)
            }

            #[cfg(feature = "hashes-crc32c")]
            HashAlgorithm::Crc32c => {
                let hasher = crc32c::Crc32cHasher::new(
                    u32::from_be_bytes(get_seed(seed))
                );

                Self::Crc32c(hasher)
            }

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_1_3_64 => {
                let hasher = siphasher::sip::SipHasher13::new_with_key(&get_seed(seed));

                Self::Siphash_1_3_64(hasher)
            }

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_1_3_128 => {
                let hasher = siphasher::sip128::SipHasher13::new_with_key(&get_seed(seed));

                Self::Siphash_1_3_128(hasher)
            }

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_2_4_64 => {
                let hasher = siphasher::sip::SipHasher24::new_with_key(&get_seed(seed));

                Self::Siphash_2_4_64(hasher)
            }

            #[cfg(feature = "hashes-siphash")]
            HashAlgorithm::Siphash_2_4_128 => {
                let hasher = siphasher::sip128::SipHasher24::new_with_key(&get_seed(seed));

                Self::Siphash_2_4_128(hasher)
            }

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh32 => {
                let hasher = xxhash_rust::xxh32::Xxh32::new(
                    u32::from_be_bytes(get_seed(seed))
                );

                Self::Xxh32(hasher)
            }

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh64 => {
                let hasher = xxhash_rust::xxh64::Xxh64::new(
                    u64::from_be_bytes(get_seed(seed))
                );

                Self::Xxh64(hasher)
            }

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh3_64 => {
                let hasher = xxhash_rust::xxh3::Xxh3::with_seed(
                    u64::from_be_bytes(get_seed(seed))
                );

                Self::Xxh3_64(hasher)
            }

            #[cfg(feature = "hashes-xxh")]
            HashAlgorithm::Xxh3_128 => {
                let hasher = xxhash_rust::xxh3::Xxh3::with_seed(
                    u64::from_be_bytes(get_seed(seed))
                );

                Self::Xxh3_128(hasher)
            }

            #[cfg(feature = "hashes-md5")]
            HashAlgorithm::Md5 => {
                use md5::Digest;

                let hasher = md5::Md5::new_with_prefix(seed);

                Self::Md5(hasher)
            }

            #[cfg(feature = "hashes-sha1")]
            HashAlgorithm::Sha1 => {
                use sha1::Digest;

                let hasher = sha1::Sha1::new_with_prefix(seed);

                Self::Sha1(hasher)
            }

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_224 => {
                use sha2::Digest;

                let hasher = sha2::Sha224::new_with_prefix(seed);

                Self::Sha2_224(hasher)
            }

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_256 => {
                use sha2::Digest;

                let hasher = sha2::Sha256::new_with_prefix(seed);

                Self::Sha2_256(hasher)
            }

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_384 => {
                use sha2::Digest;

                let hasher = sha2::Sha384::new_with_prefix(seed);

                Self::Sha2_384(hasher)
            }

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_512 => {
                use sha2::Digest;

                let hasher = sha2::Sha512::new_with_prefix(seed);

                Self::Sha2_512(hasher)
            }

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_512_224 => {
                use sha2::Digest;

                let hasher = sha2::Sha512_224::new_with_prefix(seed);

                Self::Sha2_512_224(hasher)
            }

            #[cfg(feature = "hashes-sha2")]
            HashAlgorithm::Sha2_512_256 => {
                use sha2::Digest;

                let hasher = sha2::Sha512_256::new_with_prefix(seed);

                Self::Sha2_512_256(hasher)
            }
        }
    }

    /// Get hash algorithm from the current hasher.
    pub const fn algorithm(&self) -> HashAlgorithm {
        match self {
            #[cfg(feature = "hashes-seahash")]
            Self::Seahash(_) => HashAlgorithm::Seahash,

            #[cfg(feature = "hashes-crc32")]
            Self::Crc32(_) => HashAlgorithm::Crc32,

            #[cfg(feature = "hashes-crc32c")]
            Self::Crc32c(_) => HashAlgorithm::Crc32c,

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_64(_) => HashAlgorithm::Siphash_1_3_64,

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_128(_) => HashAlgorithm::Siphash_1_3_128,

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_64(_) => HashAlgorithm::Siphash_2_4_64,

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_128(_) => HashAlgorithm::Siphash_2_4_128,

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh32(_) => HashAlgorithm::Xxh32,

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh64(_) => HashAlgorithm::Xxh64,

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_64(_) => HashAlgorithm::Xxh3_64,

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_128(_) => HashAlgorithm::Xxh3_128,

            #[cfg(feature = "hashes-md5")]
            Self::Md5(_) => HashAlgorithm::Md5,

            #[cfg(feature = "hashes-sha1")]
            Self::Sha1(_) => HashAlgorithm::Sha1,

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_224(_) => HashAlgorithm::Sha2_224,

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_256(_) => HashAlgorithm::Sha2_256,

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_384(_) => HashAlgorithm::Sha2_384,

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512(_) => HashAlgorithm::Sha2_512,

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_224(_) => HashAlgorithm::Sha2_512_224,

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_256(_) => HashAlgorithm::Sha2_512_256
        }
    }

    /// Calculate hash from all the written bytes. Some implementations consume
    /// entire buffer while others allow to continue using it. If buffer is not
    /// consumed - the hasher struct will be returned as `Some` in the pair.
    pub fn finalize(self) -> (Box<[u8]>, Option<Self>) {
        match self {
            #[cfg(feature = "hashes-seahash")]
            Self::Seahash(hasher) => {
                use std::hash::Hasher;

                let hash = Box::new(hasher.finish().to_be_bytes());

                (hash, Some(Self::Seahash(hasher)))
            }

            #[cfg(feature = "hashes-crc32")]
            Self::Crc32(hasher) => {
                (Box::new(hasher.finalize().to_be_bytes()), None)
            }

            // The actual output is u32, but hasher API forced devs to return u64.
            // We don't strip any meaningful hash bits here.
            #[cfg(feature = "hashes-crc32c")]
            Self::Crc32c(hasher) => {
                use std::hash::Hasher;

                let hash = Box::new((hasher.finish() as u32).to_be_bytes());

                (hash, Some(Self::Crc32c(hasher)))
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_64(hasher) => {
                use std::hash::Hasher;

                let hash = Box::new(hasher.finish().to_be_bytes());

                (hash, Some(Self::Siphash_1_3_64(hasher)))
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_128(hasher) => {
                use siphasher::sip128::Hasher128;

                let hash = Box::new(hasher.finish128().as_bytes());

                (hash, Some(Self::Siphash_1_3_128(hasher)))
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_64(hasher) => {
                use std::hash::Hasher;

                let hash = Box::new(hasher.finish().to_be_bytes());

                (hash, Some(Self::Siphash_2_4_64(hasher)))
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_128(hasher) => {
                use siphasher::sip128::Hasher128;

                let hash = Box::new(hasher.finish128().as_bytes());

                (hash, Some(Self::Siphash_2_4_128(hasher)))
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh32(hasher) => {
                let hash = Box::new(hasher.digest().to_be_bytes());

                (hash, Some(Self::Xxh32(hasher)))
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh64(hasher) => {
                use std::hash::Hasher;

                let hash = Box::new(hasher.finish().to_be_bytes());

                (hash, Some(Self::Xxh64(hasher)))
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_64(hasher) => {
                let hash = Box::new(hasher.digest().to_be_bytes());

                (hash, Some(Self::Xxh3_64(hasher)))
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_128(hasher) => {
                let hash = Box::new(hasher.digest128().to_be_bytes());

                (hash, Some(Self::Xxh3_128(hasher)))
            }

            #[cfg(feature = "hashes-md5")]
            Self::Md5(hasher) => {
                use md5::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }

            #[cfg(feature = "hashes-sha1")]
            Self::Sha1(hasher) => {
                use sha1::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_224(hasher) => {
                use sha2::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_256(hasher) => {
                use sha2::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_384(hasher) => {
                use sha2::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512(hasher) => {
                use sha2::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_224(hasher) => {
                use sha2::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_256(hasher) => {
                use sha2::Digest;

                let hash = hasher.finalize()
                    .to_vec()
                    .into_boxed_slice();

                (hash, None)
            }
        }
    }

    /// Calculate hash of the given bytes slice.
    pub fn hash(mut self, buf: impl AsRef<[u8]>) -> std::io::Result<Box<[u8]>> {
        self.write_all(buf.as_ref())?;
        self.flush()?;

        Ok(self.finalize().0)
    }
}

impl std::fmt::Debug for Hasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "hashes-seahash")]
            Self::Seahash(_) => f.debug_struct("Hasher")
                .field("inner", &"Seahash" as &dyn std::fmt::Debug)
                .finish(),

            #[cfg(feature = "hashes-crc32")]
            Self::Crc32(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-crc32c")]
            Self::Crc32c(_) => f.debug_struct("Hasher")
                .field("inner", &"Crc32c" as &dyn std::fmt::Debug)
                .finish(),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_64(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_128(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_64(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_128(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh32(_) => f.debug_struct("Hasher")
                .field("inner", &"Xxh32" as &dyn std::fmt::Debug)
                .finish(),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh64(_) => f.debug_struct("Hasher")
                .field("inner", &"Xxh64" as &dyn std::fmt::Debug)
                .finish(),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_64(_) => f.debug_struct("Hasher")
                .field("inner", &"Xxh3_64" as &dyn std::fmt::Debug)
                .finish(),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_128(_) => f.debug_struct("Hasher")
                .field("inner", &"Xxh3_128" as &dyn std::fmt::Debug)
                .finish(),

            #[cfg(feature = "hashes-md5")]
            Self::Md5(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-sha1")]
            Self::Sha1(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_224(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_256(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_384(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_224(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_256(hasher) => f.debug_struct("Hasher")
                .field("inner", hasher)
                .finish()
        }
    }
}

impl std::fmt::Display for Hasher {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.algorithm().fmt(f)
    }
}

impl Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            #[cfg(feature = "hashes-seahash")]
            Self::Seahash(hasher) => {
                use std::hash::Hasher;

                hasher.write(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-crc32")]
            Self::Crc32(hasher) => {
                hasher.update(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-crc32c")]
            Self::Crc32c(hasher) => {
                use std::hash::Hasher;

                hasher.write(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_64(hasher) => {
                use std::hash::Hasher;

                hasher.write(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_128(hasher) => {
                use std::hash::Hasher;

                hasher.write(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_64(hasher) => {
                use std::hash::Hasher;

                hasher.write(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_128(hasher) => {
                use std::hash::Hasher;

                hasher.write(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh32(hasher) => {
                hasher.update(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh64(hasher) => {
                hasher.update(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_64(hasher) => {
                hasher.update(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_128(hasher) => {
                hasher.update(buf);

                Ok(buf.len())
            }

            #[cfg(feature = "hashes-md5")]
            Self::Md5(hasher) => hasher.write(buf),

            #[cfg(feature = "hashes-sha1")]
            Self::Sha1(hasher) => hasher.write(buf),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_224(hasher) => hasher.write(buf),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_256(hasher) => hasher.write(buf),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_384(hasher) => hasher.write(buf),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512(hasher) => hasher.write(buf),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_224(hasher) => hasher.write(buf),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_256(hasher) => hasher.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            #[cfg(feature = "hashes-seahash")]
            Self::Seahash(_) => Ok(()),

            #[cfg(feature = "hashes-crc32")]
            Self::Crc32(_) => Ok(()),

            #[cfg(feature = "hashes-crc32c")]
            Self::Crc32c(_) => Ok(()),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_64(_) => Ok(()),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_128(_) => Ok(()),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_64(_) => Ok(()),

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_128(_) => Ok(()),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh32(_) => Ok(()),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh64(_) => Ok(()),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_64(_) => Ok(()),

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_128(_) => Ok(()),

            #[cfg(feature = "hashes-md5")]
            Self::Md5(hasher) => hasher.flush(),

            #[cfg(feature = "hashes-sha1")]
            Self::Sha1(hasher) => hasher.flush(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_224(hasher) => hasher.flush(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_256(hasher) => hasher.flush(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_384(hasher) => hasher.flush(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512(hasher) => hasher.flush(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_224(hasher) => hasher.flush(),

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_256(hasher) => hasher.flush()
        }
    }
}
