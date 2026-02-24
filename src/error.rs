/// Unified error type for currently implemented core APIs.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Invalid GraphAr file type string.
    #[error("invalid file type: {value}")]
    InvalidFileType {
        /// Original parse input.
        value: Box<str>,
    },
    /// Invalid GraphAr adjacency list type string.
    #[error("invalid adj list type: {value}")]
    InvalidAdjListType {
        /// Original parse input.
        value: Box<str>,
    },
    /// Invalid GraphAr version string.
    #[error("invalid version: {value}")]
    InvalidVersion {
        /// Original parse input.
        value: Box<str>,
    },
    /// Invalid GraphAr metadata content.
    #[error("invalid metadata: {message}")]
    InvalidMetadata {
        /// Actionable message that points to the invalid field or invariant.
        message: Box<str>,
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

    /// Create an `InvalidMetadata` error.
    pub fn invalid_metadata(message: impl Into<Box<str>>) -> Self {
        Self::InvalidMetadata {
            message: message.into(),
        }
    }
}

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
        assert_eq!(
            Error::invalid_metadata("graph.vertices[0].chunk_size must be > 0").to_string(),
            "invalid metadata: graph.vertices[0].chunk_size must be > 0"
        );
    }

    #[test]
    fn error_is_send_sync() {
        assert_send_sync::<Error>();
    }
}
