use std::path::{Path, PathBuf};

#[cfg(feature = "archives-tar")]
pub mod tar;

#[cfg(feature = "archives-zip")]
pub mod zip;

#[cfg(feature = "archives-7z")]
pub mod sevenz;

#[cfg(any(
    feature = "archives-tar",
    feature = "archives-zip",
    feature = "archives-7z"
))]
pub mod universal;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArchiveEntry {
    /// Relative path of the archive entry.
    pub path: PathBuf,

    /// Size of the archive entry.
    ///
    /// Depending on implementation this could either mean compressed or
    /// uncompressed size.
    pub size: u64
}

pub trait ArchiveExtractionContext {
    type Error;

    /// Get amount of extracted bytes.
    fn current(&self) -> u64;

    /// Get total amount of bytes to extract.
    fn total(&self) -> u64;

    /// Get files extraction progress.
    fn fraction(&self) -> f64 {
        let current = self.current();
        let total = self.total();

        if current == 0 {
            return 0.0;
        }

        if total == 0 {
            return 1.0;
        }

        current as f64 / total as f64
    }

    /// Check if extraction has finished.
    ///
    /// Note that it could fail so this doesn't mean that we've successfully
    /// extracted all the files.
    fn is_finished(&self) -> bool;

    /// Wait until the extraction is completed.
    fn wait(self) -> Result<(), Self::Error>;
}

pub trait Archive {
    type Extractor: ArchiveExtractionContext;
    type Error;

    /// Open archive.
    fn open(path: impl AsRef<Path>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Get list of entries in the archive.
    fn get_entries(&self) -> Result<Vec<ArchiveEntry>, Self::Error>;

    /// Get total size of all the entries in the archive.
    fn total_size(&self) -> Result<u64, Self::Error> {
        Ok(self
            .get_entries()?
            .into_iter()
            .map(|entry| entry.size)
            .sum())
    }

    /// Extract archive's content to a folder, using provided callback to handle
    /// the progress.
    ///
    /// Callback accepts currently extracted amount of bytes, total expected
    /// amount and a diff between calls.
    fn extract(
        &self,
        folder: impl AsRef<Path>,
        progress: impl FnMut(u64, u64, u64) + Send + 'static
    ) -> Result<Self::Extractor, Self::Error>;
}
