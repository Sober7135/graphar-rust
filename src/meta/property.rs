use crate::meta::{DataType, FileType, PropertyName};

/// Cardinality for one property in metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyCardinality {
    /// Exactly one value per record.
    Single,
    /// Ordered multi-values per record.
    List,
    /// Unordered unique multi-values per record.
    Set,
}

/// Metadata for one property column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyMeta {
    name: PropertyName,
    data_type: DataType,
    is_primary: bool,
    is_nullable: bool,
    cardinality: PropertyCardinality,
}

impl PropertyMeta {
    /// Create a property metadata object.
    ///
    /// Primary keys are always treated as non-nullable.
    pub fn new(
        name: PropertyName,
        data_type: DataType,
        is_primary: bool,
        is_nullable: bool,
        cardinality: PropertyCardinality,
    ) -> Self {
        Self {
            name,
            data_type,
            is_primary,
            is_nullable: !is_primary && is_nullable,
            cardinality,
        }
    }

    /// Return property name.
    pub fn name(&self) -> &PropertyName {
        &self.name
    }

    /// Return logical data type name.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// Return whether this property is a primary key.
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    /// Return whether this property is nullable.
    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }

    /// Return property cardinality.
    pub fn cardinality(&self) -> PropertyCardinality {
        self.cardinality
    }
}

/// Metadata for one property group.
///
/// A property group stores a list of properties in one file series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyGroupMeta {
    prefix: Box<str>,
    file_type: FileType,
    properties: Vec<PropertyMeta>,
}

impl PropertyGroupMeta {
    /// Create a property group metadata object.
    ///
    /// If `prefix` is empty, the default prefix is generated from joined
    /// property names with `'_'` and a trailing `'/'`.
    pub fn new(
        prefix: impl Into<Box<str>>,
        file_type: FileType,
        properties: Vec<PropertyMeta>,
    ) -> Self {
        let mut prefix = prefix.into();
        if prefix.is_empty() && !properties.is_empty() {
            let joined = properties
                .iter()
                .map(|p| p.name().as_str())
                .collect::<Vec<_>>()
                .join("_");
            prefix = format!("{joined}/").into_boxed_str();
        }

        Self {
            prefix,
            file_type,
            properties,
        }
    }

    /// Return the path prefix of this property group.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Return the file type of this property group.
    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    /// Return all properties in this group.
    pub fn properties(&self) -> &[PropertyMeta] {
        &self.properties
    }

    /// Validate property-group invariants and return the first actionable error.
    ///
    /// Validation checks:
    /// - non-empty `prefix`
    /// - reserved prefix `offset` is rejected
    /// - non-empty property names
    /// - CSV currently only supports `Single` cardinality
    ///
    /// Note: duplicate property names are validated at `VertexMeta`/`EdgeMeta`
    /// scope so the uniqueness rule can apply across groups, not only inside one
    /// group.
    pub(crate) fn validate(&self, path: &str) -> crate::Result<()> {
        // Prefix must be present because it participates in artifact path layout.
        if self.prefix.is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.prefix must not be empty"
            )));
        }
        // `offset/` is reserved by edge offset chunks in GraphAr path layout.
        if self
            .prefix
            .trim_end_matches('/')
            .eq_ignore_ascii_case("offset")
        {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.prefix `offset` is reserved"
            )));
        }

        // Validate per-property constraints.
        for (idx, property) in self.properties.iter().enumerate() {
            if property.name().as_str().is_empty() {
                return Err(crate::Error::invalid_metadata(format!(
                    "{path}.properties[{idx}].name must not be empty"
                )));
            }
            // TODO:
            // CSV encoding currently assumes one scalar value per row/column cell,
            // so list/set cardinalities are rejected for now.
            if self.file_type == FileType::Csv
                && property.cardinality() != PropertyCardinality::Single
            {
                return Err(crate::Error::invalid_metadata(format!(
                    "{path}.properties[{idx}] csv does not support non-single cardinality"
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::meta::{DataType, FileType, PropertyCardinality, PropertyGroupMeta, PropertyMeta};

    fn property(name: &str) -> PropertyMeta {
        PropertyMeta::new(
            name.into(),
            DataType::Int64,
            false,
            false,
            PropertyCardinality::Single,
        )
    }

    #[test]
    fn reject_reserved_offset_prefix() {
        // `offset` is reserved for edge offset chunks. Allowing property-group
        // prefixes to use it can collide with topology/offset path layout.
        for prefix in ["offset", "offset/"] {
            let group = PropertyGroupMeta::new(prefix, FileType::Parquet, vec![property("id")]);
            let err = group.validate("property_group").unwrap_err();
            match err {
                crate::Error::InvalidMetadata { message } => {
                    assert!(message.contains("prefix `offset` is reserved"));
                }
                _ => panic!("unexpected error variant"),
            }
        }
    }
}
