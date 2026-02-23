use std::{fmt, str::FromStr};

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

impl FromStr for FileType {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "csv" => Ok(Self::Csv),
            "parquet" => Ok(Self::Parquet),
            "orc" => Ok(Self::Orc),
            "json" => Ok(Self::Json),
            _ => Err(crate::Error::invalid_file_type(value.trim())),
        }
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

impl FromStr for AdjListType {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "unordered_by_source" => Ok(Self::UnorderedBySource),
            "unordered_by_dest" => Ok(Self::UnorderedByDest),
            "ordered_by_source" => Ok(Self::OrderedBySource),
            "ordered_by_dest" => Ok(Self::OrderedByDest),
            _ => Err(crate::Error::invalid_adj_list_type(value.trim())),
        }
    }
}

/// GraphAr info format version, rendered as `gar/vN`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32);

impl Version {
    /// Canonical GraphAr V1.
    pub const V1: Self = Self(1);

    /// Create a version from an integer value.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Return the integer value of this version.
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Self::new(value)
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

impl FromStr for Version {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = value.trim().to_ascii_lowercase();
        let Some(number) = normalized.strip_prefix("gar/v") else {
            return Err(crate::Error::invalid_version(value.trim()));
        };

        let parsed = number
            .parse::<u32>()
            .map_err(|_| crate::Error::invalid_version(value.trim()))?;
        if parsed == 0 {
            return Err(crate::Error::invalid_version(value.trim()));
        }
        Ok(Self(parsed))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn file_type_roundtrip() {
        for (raw, expected) in [
            ("csv", FileType::Csv),
            ("parquet", FileType::Parquet),
            ("orc", FileType::Orc),
            ("json", FileType::Json),
        ] {
            let parsed = FileType::from_str(raw).unwrap();
            assert_eq!(parsed, expected);
            assert_eq!(parsed.to_string(), raw);
        }
    }

    #[test]
    fn file_type_parse_invalid() {
        let err = FileType::from_str("tsv").unwrap_err();
        match err {
            crate::Error::InvalidFileType { value } => assert_eq!(value.as_ref(), "tsv"),
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn adj_list_type_roundtrip() {
        for (raw, expected, ordered, aligned_by) in [
            (
                "unordered_by_source",
                AdjListType::UnorderedBySource,
                false,
                "src",
            ),
            (
                "unordered_by_dest",
                AdjListType::UnorderedByDest,
                false,
                "dst",
            ),
            (
                "ordered_by_source",
                AdjListType::OrderedBySource,
                true,
                "src",
            ),
            ("ordered_by_dest", AdjListType::OrderedByDest, true, "dst"),
        ] {
            let parsed = AdjListType::from_str(raw).unwrap();
            assert_eq!(parsed, expected);
            assert_eq!(parsed.to_string(), raw);
            assert_eq!(parsed.is_ordered(), ordered);
            assert_eq!(parsed.aligned_by(), aligned_by);
        }
    }

    #[test]
    fn adj_list_type_parse_invalid() {
        let err = AdjListType::from_str("ordered_src").unwrap_err();
        match err {
            crate::Error::InvalidAdjListType { value } => assert_eq!(value.as_ref(), "ordered_src"),
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn version_roundtrip() {
        let version = Version::from_str("gar/v1").unwrap();
        assert_eq!(version, Version::V1);
        assert_eq!(version.value(), 1);
        assert_eq!(version.to_string(), "gar/v1");
    }

    #[test]
    fn version_parse_invalid() {
        for raw in ["v1", "gar/v0", "gar/vx"] {
            let err = Version::from_str(raw).unwrap_err();
            match err {
                crate::Error::InvalidVersion { value } => assert_eq!(value.as_ref(), raw),
                _ => panic!("unexpected error variant"),
            }
        }
    }

    #[test]
    fn public_type_enums_are_send_sync() {
        assert_send_sync::<FileType>();
        assert_send_sync::<AdjListType>();
        assert_send_sync::<Version>();
    }
}
