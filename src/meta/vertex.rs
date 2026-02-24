use std::collections::HashSet;

use crate::meta::{PropertyGroupMeta, Version, VertexLabel};

/// Metadata for one vertex type.
///
/// This type mirrors one vertex info artifact:
/// - `label` maps to artifact field `type`.
/// - `chunk_size` maps to `chunk_size`.
/// - `labels` are optional vertex-level tags.
/// - `prefix` maps to `prefix`.
/// - `property_groups` maps to `property_groups`.
/// - `version` maps to optional field `version`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexMeta {
    /// Canonical vertex type name (`type` in artifacts).
    label: VertexLabel,
    /// Number of vertices per chunk (`chunk_size` in artifacts).
    chunk_size: usize,
    /// Optional vertex-level tags.
    labels: Vec<Box<str>>,
    /// Path prefix used to resolve vertex data files (`prefix` in artifacts).
    prefix: Box<str>,
    /// Property-group definitions for this vertex type.
    property_groups: Vec<PropertyGroupMeta>,
    /// Optional per-vertex metadata format version (`version` in artifacts).
    version: Option<Version>,
}

impl VertexMeta {
    /// Create a vertex metadata object.
    ///
    /// # Parameters
    ///
    /// `label` maps to artifact field `type`.
    /// `chunk_size` maps to artifact field `chunk_size`.
    /// `labels` are optional vertex-level tags.
    /// `prefix` maps to artifact field `prefix`.
    /// If `prefix` is empty, the default prefix is `{label}`.
    /// `property_groups` maps to artifact field `property_groups`.
    /// `version` maps to optional artifact field `version`.
    ///
    /// This constructor does not validate metadata invariants.
    /// Call `validate` on the parent graph metadata for full semantic checks.
    pub fn new(
        label: VertexLabel,
        chunk_size: usize,
        labels: Vec<Box<str>>,
        prefix: impl Into<Box<str>>,
        property_groups: Vec<PropertyGroupMeta>,
        version: Option<Version>,
    ) -> Self {
        let mut prefix = prefix.into();
        if prefix.is_empty() {
            prefix = label.as_str().into();
        }

        Self {
            label,
            chunk_size,
            labels,
            prefix,
            property_groups,
            version,
        }
    }

    /// Return the vertex type identifier (`type` in artifacts).
    pub fn label(&self) -> &VertexLabel {
        &self.label
    }

    /// Return vertex chunk size.
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Return optional vertex-level tags.
    pub fn labels(&self) -> &[Box<str>] {
        &self.labels
    }

    /// Return path prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Return property groups.
    pub fn property_groups(&self) -> &[PropertyGroupMeta] {
        &self.property_groups
    }

    /// Return optional per-vertex metadata format version.
    pub fn version(&self) -> Option<Version> {
        self.version
    }

    /// Validate semantic constraints of this vertex metadata entry.
    ///
    /// Validation checks:
    /// - non-empty `label`
    /// - `chunk_size > 0`
    /// - non-empty `prefix`
    /// - no duplicate property names across all property groups
    pub(crate) fn validate(&self, path: &str) -> crate::Result<()> {
        // Basic identity and storage constraints.
        if self.label.as_str().is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.label must not be empty"
            )));
        }
        if self.chunk_size == 0 {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.chunk_size must be > 0"
            )));
        }
        if self.prefix.is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.prefix must not be empty"
            )));
        }

        // Enforce global property-name uniqueness across all groups.
        let mut property_unique = HashSet::new();
        for (group_idx, group) in self.property_groups.iter().enumerate() {
            group.validate(&format!("{path}.property_groups[{group_idx}]"))?;
            for property in group.properties() {
                if !property_unique.insert(property.name().as_str()) {
                    return Err(crate::Error::invalid_metadata(format!(
                        "{path}.property_groups has duplicate property `{}`",
                        property.name().as_str()
                    )));
                }
            }
        }

        Ok(())
    }
}
