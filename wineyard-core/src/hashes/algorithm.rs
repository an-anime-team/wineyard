use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    #[cfg(feature = "hashes-seahash")]
    Seahash,

    #[cfg(feature = "hashes-crc32")]
    Crc32,

    #[cfg(feature = "hashes-crc32c")]
    Crc32c,

    #[cfg(feature = "hashes-siphash")]
    Siphash_1_3_64,

    #[cfg(feature = "hashes-siphash")]
    Siphash_1_3_128,

    #[cfg(feature = "hashes-siphash")]
    Siphash_2_4_64,

    #[cfg(feature = "hashes-siphash")]
    Siphash_2_4_128,

    #[cfg(feature = "hashes-xxh")]
    Xxh32,

    #[cfg(feature = "hashes-xxh")]
    Xxh64,

    #[cfg(feature = "hashes-xxh")]
    Xxh3_64,

    #[cfg(feature = "hashes-xxh")]
    Xxh3_128,

    #[cfg(feature = "hashes-md5")]
    Md5,

    #[cfg(feature = "hashes-sha1")]
    Sha1,

    #[cfg(feature = "hashes-sha2")]
    Sha2_224,

    #[cfg(feature = "hashes-sha2")]
    Sha2_256,

    #[cfg(feature = "hashes-sha2")]
    Sha2_384,

    #[cfg(feature = "hashes-sha2")]
    Sha2_512,

    #[cfg(feature = "hashes-sha2")]
    Sha2_512_224,

    #[cfg(feature = "hashes-sha2")]
    Sha2_512_256
}

impl HashAlgorithm {
    pub const fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "hashes-seahash")]
            Self::Seahash => "seahash",

            #[cfg(feature = "hashes-crc32")]
            Self::Crc32 => "crc32",

            #[cfg(feature = "hashes-crc32c")]
            Self::Crc32c => "crc32c",

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_64 => "siphash-1-3-64",

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_1_3_128 => "siphash-1-3-128",

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_64 => "siphash-2-4-64",

            #[cfg(feature = "hashes-siphash")]
            Self::Siphash_2_4_128 => "siphash-2-4-128",

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh32 => "xxh32",

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh64 => "xxh64",

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_64 => "xxh3-64",

            #[cfg(feature = "hashes-xxh")]
            Self::Xxh3_128 => "xxh3-128",

            #[cfg(feature = "hashes-md5")]
            Self::Md5 => "md5",

            #[cfg(feature = "hashes-sha1")]
            Self::Sha1 => "sha1",

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_224 => "sha2-224",

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_256 => "sha2-256",

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_384 => "sha2-384",

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512 => "sha2-512",

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_224 => "sha2-512/224",

            #[cfg(feature = "hashes-sha2")]
            Self::Sha2_512_256 => "sha2-512/256"
        }
    }
}

impl FromStr for HashAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "hashes-seahash")]
            "seahash" => Ok(Self::Seahash),

            #[cfg(feature = "hashes-crc32")]
            "crc32" => Ok(Self::Crc32),

            #[cfg(feature = "hashes-crc32c")]
            "crc32c" => Ok(Self::Crc32c),

            #[cfg(feature = "hashes-siphash")]
            "siphash-1-3-64" | "siphash-1-3" => Ok(Self::Siphash_1_3_64),

            #[cfg(feature = "hashes-siphash")]
            "siphash-1-3-128" => Ok(Self::Siphash_1_3_128),

            #[cfg(feature = "hashes-siphash")]
            "siphash-2-4-64" | "siphash-2-4" | "siphash" => Ok(Self::Siphash_2_4_64),

            #[cfg(feature = "hashes-siphash")]
            "siphash-2-4-128" => Ok(Self::Siphash_2_4_128),

            #[cfg(feature = "hashes-xxh")]
            "xxh32" | "xxh-32" => Ok(Self::Xxh32),

            #[cfg(feature = "hashes-xxh")]
            "xxh64" | "xxh-64" | "xxh" => Ok(Self::Xxh64),

            #[cfg(feature = "hashes-xxh")]
            "xxh3-64" | "xxh3" => Ok(Self::Xxh3_64),

            #[cfg(feature = "hashes-xxh")]
            "xxh3-128" => Ok(Self::Xxh3_128),

            #[cfg(feature = "hashes-md5")]
            "md5" => Ok(Self::Md5),

            #[cfg(feature = "hashes-sha1")]
            "sha1" => Ok(Self::Sha1),

            #[cfg(feature = "hashes-sha2")]
            "sha2-224" => Ok(Self::Sha2_224),

            #[cfg(feature = "hashes-sha2")]
            "sha2-256" | "sha2" => Ok(Self::Sha2_256),

            #[cfg(feature = "hashes-sha2")]
            "sha2-384" => Ok(Self::Sha2_384),

            #[cfg(feature = "hashes-sha2")]
            "sha2-512" => Ok(Self::Sha2_512),

            #[cfg(feature = "hashes-sha2")]
            "sha2-512/224" | "sha2-512-224" => Ok(Self::Sha2_512_224),

            #[cfg(feature = "hashes-sha2")]
            "sha2-512/256" | "sha2-512-256" => Ok(Self::Sha2_512_256),

            _ => Err(format!("unsupported hash algorithm: {s}"))
        }
    }
}

impl std::fmt::Display for HashAlgorithm {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
