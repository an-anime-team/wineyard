use std::path::{Path, PathBuf};

use super::{
    Archive,
    ArchiveExtractionContext,
    ArchiveEntry
};

#[cfg(feature = "archives-tar")]
use super::tar::TarArchive;

#[cfg(feature = "archives-zip")]
use super::zip::ZipArchive;

#[cfg(feature = "archives-7z")]
use super::sevenz::{
    SevenzArchive,
    SevenzArchiveError
};

const FORMATS: &[(ArchiveFormat, &[&str])] = &[
    #[cfg(feature = "archives-tar")]
    (ArchiveFormat::Tar, &[
        ".tar",
        ".tar.xz",
        ".tar.gz",
        ".tar.bz2",
        ".tar.zst",
        ".tar.zstd",
        ".txz",
        ".tgz",
        ".tbz2",
        ".tzst",
        ".tzstd"
    ]),

    #[cfg(feature = "archives-zip")]
    (ArchiveFormat::Zip, &[
        ".zip"
    ]),

    #[cfg(feature = "archives-7z")]
    (ArchiveFormat::Sevenz, &[
        ".7z",
        ".7z.001",
        ".zip.001"
    ])
];

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[cfg(feature = "archives-7z")]
    #[error("7z package is not installed")]
    SevenzNotAvailable,

    #[error("unsupported archive format: {0:?}")]
    UnsupportedFormat(PathBuf),

    #[error("failed to extract archive: {0}")]
    ExtractionError(&'static str)
}

#[cfg(feature = "archives-7z")]
impl From<SevenzArchiveError> for Error {
    #[inline]
    fn from(value: SevenzArchiveError) -> Self {
        match value {
            SevenzArchiveError::Io(err) => Self::Io(err),
            SevenzArchiveError::SevenzNotAvailable => Self::SevenzNotAvailable
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchiveFormat {
    #[cfg(feature = "archives-tar")]
    Tar,

    #[cfg(feature = "archives-zip")]
    Zip,

    #[cfg(feature = "archives-7z")]
    Sevenz
}

impl ArchiveFormat {
    /// Assume archive format from the fs path.
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let path = path.as_ref()
            .as_os_str()
            .to_string_lossy();

        for (format, exts) in FORMATS {
            for ext in exts.iter() {
                if path.ends_with(ext) {
                    return Some(*format);
                }
            }
        }

        None
    }
}

/// Get list of entries of an archive.
///
/// Utility function that automatically predicts the format of the archive and
/// needed struct to process it.
pub fn get_entries(path: impl AsRef<Path>) -> Result<Vec<ArchiveEntry>, Error> {
    let path = path.as_ref();

    let entries = match ArchiveFormat::from_path(path) {
        #[cfg(feature = "archives-tar")]
        Some(ArchiveFormat::Tar) => TarArchive::open(path)
            .and_then(|archive| archive.get_entries())?,

        #[cfg(feature = "archives-zip")]
        Some(ArchiveFormat::Zip) => ZipArchive::open(path)
            .and_then(|archive| archive.get_entries())?,

        #[cfg(feature = "archives-7z")]
        Some(ArchiveFormat::Sevenz) => SevenzArchive::open(path)
            .and_then(|archive| archive.get_entries())?,

        None => return Err(Error::UnsupportedFormat(path.to_path_buf()))
    };

    Ok(entries)
}

/// Get total size of files in the archive.
///
/// Utility function that automatically predicts the format of the archive and
/// needed struct to process it.
pub fn get_total_size(path: impl AsRef<Path>) -> Result<u64, Error> {
    let path = path.as_ref();

    let total_size = match ArchiveFormat::from_path(path) {
        #[cfg(feature = "archives-tar")]
        Some(ArchiveFormat::Tar) => TarArchive::open(path)
            .and_then(|archive| archive.total_size())?,

        #[cfg(feature = "archives-zip")]
        Some(ArchiveFormat::Zip) => ZipArchive::open(path)
            .and_then(|archive| archive.total_size())?,

        #[cfg(feature = "archives-7z")]
        Some(ArchiveFormat::Sevenz) => SevenzArchive::open(path)
            .and_then(|archive| archive.total_size())?,

        None => return Err(Error::UnsupportedFormat(path.to_path_buf()))
    };

    Ok(total_size)
}

/// Extract files from the archive.
///
/// Utility function that automatically predicts the format of the archive and
/// needed struct to process it.
///
/// This function will freeze the current thread until the archive is fully
/// extracted.
pub fn extract(
    path: impl AsRef<Path>,
    folder: impl AsRef<Path>,
    progress: impl FnMut(u64, u64, u64) + Send + 'static
) -> Result<(), Error> {
    let path = path.as_ref();

    match ArchiveFormat::from_path(path) {
        #[cfg(feature = "archives-tar")]
        Some(ArchiveFormat::Tar) => {
            TarArchive::open(path)
                .and_then(|archive| archive.extract(folder, progress))?
                .wait()
                .map_err(Error::ExtractionError)?;
        }

        #[cfg(feature = "archives-zip")]
        Some(ArchiveFormat::Zip) => {
            ZipArchive::open(path)
                .and_then(|archive| archive.extract(folder, progress))?
                .wait()
                .map_err(Error::ExtractionError)?;
        }

        #[cfg(feature = "archives-7z")]
        Some(ArchiveFormat::Sevenz) => {
            SevenzArchive::open(path)
                .and_then(|archive| archive.extract(folder, progress))?
                .wait()
                .map_err(Error::ExtractionError)?;
        }

        None => return Err(Error::UnsupportedFormat(path.to_path_buf()))
    }

    Ok(())
}
