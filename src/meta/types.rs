use std::fmt;

/// GraphAr payload file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum FileType {
    /// CSV file format.
    Csv,
    /// Apache Parquet file format.
    Parquet,
    /// Apache ORC file format.
    Orc,
    /// JSON file format.
    Json,
}

impl FileType {
    /// Return the GraphAr canonical lowercase string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Parquet => "parquet",
            Self::Orc => "orc",
            Self::Json => "json",
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// GraphAr adjacency list layout type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AdjListType {
    /// Unordered edges, partitioned by source id.
    UnorderedBySource,
    /// Unordered edges, partitioned by destination id.
    UnorderedByDest,
    /// Ordered edges by source id.
    OrderedBySource,
    /// Ordered edges by destination id.
    OrderedByDest,
}

impl AdjListType {
    /// Return the GraphAr canonical lowercase string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnorderedBySource => "unordered_by_source",
            Self::UnorderedByDest => "unordered_by_dest",
            Self::OrderedBySource => "ordered_by_source",
            Self::OrderedByDest => "ordered_by_dest",
        }
    }

    /// Return whether this layout is ordered.
    pub const fn is_ordered(self) -> bool {
        match self {
            Self::OrderedBySource | Self::OrderedByDest => true,
            Self::UnorderedBySource | Self::UnorderedByDest => false,
        }
    }

    /// Return the alignment key used by this layout: `"src"` or `"dst"`.
    pub const fn aligned_by(self) -> &'static str {
        match self {
            Self::OrderedBySource | Self::UnorderedBySource => "src",
            Self::OrderedByDest | Self::UnorderedByDest => "dst",
        }
    }
}

impl fmt::Display for AdjListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// GraphAr info format version, rendered as `gar/vN`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32);

impl Version {
    /// Canonical GraphAr V1.
    pub const V1: Self = Self(1);

    /// Create a version from an integer value.
    ///
    /// # Panics
    ///
    /// Panics if `value == 0`.
    pub const fn new(value: u32) -> Self {
        assert!(value > 0, "version must be greater than 0");
        Self(value)
    }

    /// Try to create a version from an integer value.
    pub fn try_new(value: u32) -> crate::Result<Self> {
        if value == 0 {
            return Err(crate::Error::invalid_version(value.to_string()));
        }
        Ok(Self(value))
    }

    /// Return the integer value of this version.
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::V1
    }
}

impl TryFrom<u32> for Version {
    type Error = crate::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<Version> for u32 {
    fn from(value: Version) -> Self {
        value.0
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gar/v{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn file_type_roundtrip() {
        for (value, expected) in [
            (FileType::Csv, "csv"),
            (FileType::Parquet, "parquet"),
            (FileType::Orc, "orc"),
            (FileType::Json, "json"),
        ] {
            assert_eq!(value.to_string(), expected);
        }
    }

    #[test]
    fn adj_list_type_roundtrip() {
        for (value, expected, ordered, aligned_by) in [
            (
                AdjListType::UnorderedBySource,
                "unordered_by_source",
                false,
                "src",
            ),
            (
                AdjListType::UnorderedByDest,
                "unordered_by_dest",
                false,
                "dst",
            ),
            (
                AdjListType::OrderedBySource,
                "ordered_by_source",
                true,
                "src",
            ),
            (AdjListType::OrderedByDest, "ordered_by_dest", true, "dst"),
        ] {
            assert_eq!(value.to_string(), expected);
            assert_eq!(value.is_ordered(), ordered);
            assert_eq!(value.aligned_by(), aligned_by);
        }
    }

    #[test]
    fn version_roundtrip() {
        assert_eq!(Version::V1.value(), 1);
        assert_eq!(Version::V1.to_string(), "gar/v1");
    }

    #[test]
    fn public_type_enums_are_send_sync() {
        assert_send_sync::<FileType>();
        assert_send_sync::<AdjListType>();
        assert_send_sync::<Version>();
    }

    #[test]
    fn version_default_is_v1() {
        assert_eq!(Version::default(), Version::V1);
    }

    #[test]
    #[should_panic(expected = "version must be greater than 0")]
    fn version_new_rejects_zero() {
        let _ = Version::new(0);
    }

    #[test]
    fn version_try_from_is_fallible() {
        assert_eq!(Version::try_from(1).unwrap(), Version::V1);

        let err = Version::try_from(0).unwrap_err();
        match err {
            crate::Error::InvalidVersion { value } => assert_eq!(value.as_ref(), "0"),
            _ => panic!("unexpected error variant"),
        }
    }
}
