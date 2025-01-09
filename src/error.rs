use thiserror::Error;
use zip::result::ZipError;

/// An error that can occur while writing a 3MF file
#[derive(Debug, Error)]
pub enum Error {
    /// I/O error while writing 3MF file
    #[error("I/O error while importing/exporting to 3MF file")]
    Io(#[from] std::io::Error),

    /// Error writing ZIP file (3MF files are ZIP files)
    #[error("Error writing ZIP file (3MF files are ZIP files)")]
    Zip(#[from] ZipError),

    /// Error Deserializing internal 3MF XML structure
    #[error("Deserialization error from xml reading")]
    DeError(#[from] quick_xml::DeError),

    /// Error Serializing internal 3MF XML structure
    #[error("Serialization error from xml writing")]
    SeError(#[from] quick_xml::SeError),
}
