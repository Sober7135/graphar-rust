use std::{error::Error as StdError, fmt};

/// Unified error type for currently implemented core APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Invalid GraphAr file type string.
    InvalidFileType {
        /// Original parse input.
        value: Box<str>,
    },
    /// Invalid GraphAr adjacency list type string.
    InvalidAdjListType {
        /// Original parse input.
        value: Box<str>,
    },
    /// Invalid GraphAr version string.
    InvalidVersion {
        /// Original parse input.
        value: Box<str>,
    },
}

impl Error {
    /// Create an `InvalidFileType` error.
    pub fn invalid_file_type(value: impl Into<Box<str>>) -> Self {
        Self::InvalidFileType {
            value: value.into(),
        }
    }

    /// Create an `InvalidAdjListType` error.
    pub fn invalid_adj_list_type(value: impl Into<Box<str>>) -> Self {
        Self::InvalidAdjListType {
            value: value.into(),
        }
    }

    /// Create an `InvalidVersion` error.
    pub fn invalid_version(value: impl Into<Box<str>>) -> Self {
        Self::InvalidVersion {
            value: value.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFileType { value } => write!(f, "invalid file type: {value}"),
            Self::InvalidAdjListType { value } => write!(f, "invalid adj list type: {value}"),
            Self::InvalidVersion { value } => write!(f, "invalid version: {value}"),
        }
    }
}

impl StdError for Error {}

/// Crate-wide result alias.
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn error_display() {
        assert_eq!(
            Error::invalid_file_type("tsv").to_string(),
            "invalid file type: tsv"
        );
        assert_eq!(
            Error::invalid_adj_list_type("ordered_src").to_string(),
            "invalid adj list type: ordered_src"
        );
        assert_eq!(
            Error::invalid_version("gar/v0").to_string(),
            "invalid version: gar/v0"
        );
    }

    #[test]
    fn error_is_send_sync() {
        assert_send_sync::<Error>();
    }
}
