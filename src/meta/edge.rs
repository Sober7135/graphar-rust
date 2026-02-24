use std::collections::HashSet;

use crate::meta::{AdjListType, EdgeLabel, EdgeTriplet, PropertyGroupMeta, Version, VertexLabel};

/// Metadata for one adjacency-list layout under an edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjListMeta {
    kind: AdjListType,
    prefix: Box<str>,
}

impl AdjListMeta {
    /// Create an adjacency-list metadata object.
    ///
    /// If `prefix` is empty, the default prefix is `{kind}/`.
    pub fn new(kind: AdjListType, prefix: impl Into<Box<str>>) -> Self {
        let mut prefix = prefix.into();
        if prefix.is_empty() {
            prefix = format!("{}/", kind.as_str()).into_boxed_str();
        }
        Self { kind, prefix }
    }

    /// Return adjacency-list type.
    pub fn kind(&self) -> AdjListType {
        self.kind
    }

    /// Return adjacency-list path prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub(crate) fn validate(&self, path: &str) -> crate::Result<()> {
        // Adjacency-list prefix is required for deterministic path generation.
        if self.prefix.is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.prefix must not be empty"
            )));
        }
        Ok(())
    }
}

/// Metadata for one edge type triplet.
///
/// `triplet` maps to artifact fields `src_type`/`edge_type`/`dst_type`.
/// `version` maps to optional field `version`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeMeta {
    /// Edge identity triplet.
    triplet: EdgeTriplet,
    /// Whether this edge type is directed (`directed` in artifacts).
    directed: bool,
    /// Edge/src/dst chunk sizes.
    chunk_sizes: EdgeChunkSizes,
    /// Path prefix for this edge type (`prefix` in artifacts).
    prefix: Box<str>,
    /// Adjacency-list layout definitions (`adj_lists` in artifacts).
    adj_lists: Vec<AdjListMeta>,
    /// Edge property-group definitions (`property_groups` in artifacts).
    property_groups: Vec<PropertyGroupMeta>,
    /// Optional per-edge metadata format version (`version` in artifacts).
    version: Option<Version>,
}

/// Chunk size triple used by one edge metadata entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeChunkSizes {
    edge: usize,
    src: usize,
    dst: usize,
}

impl EdgeChunkSizes {
    /// Create chunk sizes for edge/src/dst partitions.
    pub const fn new(edge: usize, src: usize, dst: usize) -> Self {
        Self { edge, src, dst }
    }

    /// Return edge chunk size.
    pub const fn edge(self) -> usize {
        self.edge
    }

    /// Return source vertex chunk size.
    pub const fn src(self) -> usize {
        self.src
    }

    /// Return destination vertex chunk size.
    pub const fn dst(self) -> usize {
        self.dst
    }
}

impl EdgeMeta {
    /// Create an edge metadata object.
    ///
    /// `triplet` maps to artifact fields `src_type`/`edge_type`/`dst_type`.
    /// If `prefix` is empty, the default prefix is `{src}_{edge}_{dst}`.
    /// `version` maps to optional artifact field `version`.
    ///
    /// This constructor does not validate metadata invariants.
    /// Call `validate` on the parent graph metadata for full semantic checks.
    pub fn new(
        triplet: EdgeTriplet,
        directed: bool,
        chunk_sizes: EdgeChunkSizes,
        prefix: impl Into<Box<str>>,
        adj_lists: Vec<AdjListMeta>,
        property_groups: Vec<PropertyGroupMeta>,
        version: Option<Version>,
    ) -> Self {
        let mut prefix = prefix.into();
        if prefix.is_empty() {
            prefix = triplet.to_string().into_boxed_str();
        }

        Self {
            triplet,
            directed,
            chunk_sizes,
            prefix,
            adj_lists,
            property_groups,
            version,
        }
    }

    /// Return source vertex label.
    pub fn src(&self) -> &VertexLabel {
        self.triplet.src()
    }

    /// Return edge label.
    pub fn edge(&self) -> &EdgeLabel {
        self.triplet.edge()
    }

    /// Return destination vertex label.
    pub fn dst(&self) -> &VertexLabel {
        self.triplet.dst()
    }

    /// Return edge type triplet (`src_type`, `edge_type`, `dst_type`).
    pub fn triplet(&self) -> &EdgeTriplet {
        &self.triplet
    }

    /// Return all chunk sizes used by this edge.
    pub fn chunk_sizes(&self) -> EdgeChunkSizes {
        self.chunk_sizes
    }

    /// Return whether this edge is directed.
    pub fn directed(&self) -> bool {
        self.directed
    }

    /// Return edge chunk size.
    pub fn edge_chunk_size(&self) -> usize {
        self.chunk_sizes.edge()
    }

    /// Return source vertex chunk size.
    pub fn src_chunk_size(&self) -> usize {
        self.chunk_sizes.src()
    }

    /// Return destination vertex chunk size.
    pub fn dst_chunk_size(&self) -> usize {
        self.chunk_sizes.dst()
    }

    /// Return path prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Return configured adjacency-list layouts.
    pub fn adj_lists(&self) -> &[AdjListMeta] {
        &self.adj_lists
    }

    /// Return property groups.
    pub fn property_groups(&self) -> &[PropertyGroupMeta] {
        &self.property_groups
    }

    /// Return optional per-edge metadata format version.
    pub fn version(&self) -> Option<Version> {
        self.version
    }

    pub(crate) fn validate(&self, path: &str) -> crate::Result<()> {
        // Basic identity constraints for edge triplet.
        if self.triplet.src().as_str().is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.src must not be empty"
            )));
        }
        if self.triplet.edge().as_str().is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.edge must not be empty"
            )));
        }
        if self.triplet.dst().as_str().is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.dst must not be empty"
            )));
        }

        // Chunk sizing must be strictly positive for all dimensions.
        if self.chunk_sizes.edge() == 0 {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.edge_chunk_size must be > 0"
            )));
        }
        if self.chunk_sizes.src() == 0 {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.src_chunk_size must be > 0"
            )));
        }
        if self.chunk_sizes.dst() == 0 {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.dst_chunk_size must be > 0"
            )));
        }
        if self.prefix.is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.prefix must not be empty"
            )));
        }

        // At least one adjacency-list layout is required for edge data access.
        if self.adj_lists.is_empty() {
            return Err(crate::Error::invalid_metadata(format!(
                "{path}.adj_lists must not be empty"
            )));
        }

        // Enforce one definition per adjacency-list type.
        let mut adj_type_unique = HashSet::with_capacity(self.adj_lists.len());
        for (adj_idx, adj) in self.adj_lists.iter().enumerate() {
            adj.validate(&format!("{path}.adj_lists[{adj_idx}]"))?;
            if !adj_type_unique.insert(adj.kind()) {
                return Err(crate::Error::invalid_metadata(format!(
                    "{path}.adj_lists has duplicate type `{}`",
                    adj.kind().as_str()
                )));
            }
        }

        // Property names must be unique across all groups.
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
