use crate::meta::{
    AdjListMeta, EdgeChunkSizes, EdgeMeta, EdgeTriplet, GraphMeta, GraphName, PropertyGroupMeta,
    Version, VertexLabel, VertexMeta,
};

/// Builder for [`GraphMeta`].
///
/// This builder mirrors fields in [`GraphMeta`] and keeps write-time defaults:
/// - `version` defaults to [`Version::V1`]
/// - `labels` are graph-level tags
/// - `vertices` and `edges` are appended in insertion order
#[derive(Debug, Clone)]
pub struct GraphMetaBuilder {
    /// Graph name (`name` in artifacts).
    name: GraphName,
    /// Graph prefix (`prefix` in artifacts).
    prefix: Box<str>,
    /// Graph format version (`version` in artifacts).
    version: Version,
    /// Optional graph-level tags.
    labels: Vec<Box<str>>,
    /// Vertex metadata entries.
    vertices: Vec<VertexMeta>,
    /// Edge metadata entries.
    edges: Vec<EdgeMeta>,
}

impl GraphMetaBuilder {
    /// Create a graph metadata builder.
    ///
    /// `name` maps to artifact field `name`.
    /// `prefix` maps to artifact field `prefix`.
    pub fn new(name: GraphName, prefix: impl Into<Box<str>>) -> Self {
        Self {
            name,
            prefix: prefix.into(),
            version: Version::V1,
            labels: Vec::new(),
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Set graph format version.
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    /// Set graph-level labels (tags), not vertex type labels.
    pub fn with_labels(mut self, labels: Vec<Box<str>>) -> Self {
        self.labels = labels;
        self
    }

    /// Add one vertex metadata entry.
    pub fn add_vertex(mut self, vertex: VertexMeta) -> Self {
        self.vertices.push(vertex);
        self
    }

    /// Add one edge metadata entry.
    pub fn add_edge(mut self, edge: EdgeMeta) -> Self {
        self.edges.push(edge);
        self
    }

    /// Build graph metadata and validate it.
    pub fn build(self) -> crate::Result<GraphMeta> {
        let meta = self.build_unchecked();
        meta.validate()?;
        Ok(meta)
    }

    /// Build graph metadata without validation.
    pub fn build_unchecked(self) -> GraphMeta {
        GraphMeta::new(
            self.name,
            self.prefix,
            self.version,
            self.labels,
            self.vertices,
            self.edges,
        )
    }
}

/// Builder for [`VertexMeta`].
///
/// This builder constructs one vertex entry whose identity maps to artifact
/// field `type`.
#[derive(Debug, Clone)]
pub struct VertexMetaBuilder {
    /// Vertex type identifier (`type` in artifacts).
    label: VertexLabel,
    /// Vertex chunk size (`chunk_size` in artifacts).
    chunk_size: usize,
    /// Optional vertex-level tags.
    labels: Vec<Box<str>>,
    /// Vertex prefix (`prefix` in artifacts).
    prefix: Box<str>,
    /// Property-group definitions (`property_groups` in artifacts).
    property_groups: Vec<PropertyGroupMeta>,
    /// Optional per-vertex metadata format version (`version` in artifacts).
    version: Option<Version>,
}

impl VertexMetaBuilder {
    /// Create a vertex metadata builder.
    ///
    /// `label` maps to artifact field `type`.
    /// `chunk_size` maps to artifact field `chunk_size`.
    pub fn new(label: VertexLabel, chunk_size: usize) -> Self {
        Self {
            label,
            chunk_size,
            labels: Vec::new(),
            prefix: "".into(),
            property_groups: Vec::new(),
            version: None,
        }
    }

    /// Set vertex labels.
    pub fn with_labels(mut self, labels: Vec<Box<str>>) -> Self {
        self.labels = labels;
        self
    }

    /// Set vertex prefix.
    pub fn with_prefix(mut self, prefix: impl Into<Box<str>>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Add one property group.
    pub fn add_property_group(mut self, property_group: PropertyGroupMeta) -> Self {
        self.property_groups.push(property_group);
        self
    }

    /// Set optional per-vertex metadata format version.
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Build vertex metadata and validate it.
    pub fn build(self) -> crate::Result<VertexMeta> {
        let meta = self.build_unchecked();
        meta.validate("vertex")?;
        Ok(meta)
    }

    /// Build vertex metadata without validation.
    pub fn build_unchecked(self) -> VertexMeta {
        VertexMeta::new(
            self.label,
            self.chunk_size,
            self.labels,
            self.prefix,
            self.property_groups,
            self.version,
        )
    }
}

/// Builder for [`EdgeMeta`].
///
/// This builder constructs one edge entry whose identity maps to
/// `src_type`/`edge_type`/`dst_type` in artifacts.
#[derive(Debug, Clone)]
pub struct EdgeMetaBuilder {
    /// Edge identity triplet.
    triplet: EdgeTriplet,
    /// Directed flag (`directed` in artifacts).
    directed: bool,
    /// Edge/src/dst chunk sizes.
    chunk_sizes: EdgeChunkSizes,
    /// Edge prefix (`prefix` in artifacts).
    prefix: Box<str>,
    /// Adjacency-list layouts (`adj_lists` in artifacts).
    adj_lists: Vec<AdjListMeta>,
    /// Edge property groups (`property_groups` in artifacts).
    property_groups: Vec<PropertyGroupMeta>,
    /// Optional per-edge metadata format version (`version` in artifacts).
    version: Option<Version>,
}

impl EdgeMetaBuilder {
    /// Create an edge metadata builder.
    ///
    /// `triplet` maps to artifact fields `src_type`/`edge_type`/`dst_type`.
    /// `chunk_sizes` maps to artifact fields
    /// `edge_chunk_size`/`src_chunk_size`/`dst_chunk_size`.
    /// `adj_lists` maps to artifact field `adj_lists`.
    pub fn new(
        triplet: EdgeTriplet,
        chunk_sizes: EdgeChunkSizes,
        adj_lists: Vec<AdjListMeta>,
    ) -> Self {
        Self {
            triplet,
            directed: true,
            chunk_sizes,
            prefix: "".into(),
            adj_lists,
            property_groups: Vec::new(),
            version: None,
        }
    }

    /// Set whether the edge is directed.
    pub fn with_directed(mut self, directed: bool) -> Self {
        self.directed = directed;
        self
    }

    /// Set edge prefix.
    pub fn with_prefix(mut self, prefix: impl Into<Box<str>>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Add one adjacency-list layout.
    pub fn add_adj_list(mut self, adj_list: AdjListMeta) -> Self {
        self.adj_lists.push(adj_list);
        self
    }

    /// Add one property group.
    pub fn add_property_group(mut self, property_group: PropertyGroupMeta) -> Self {
        self.property_groups.push(property_group);
        self
    }

    /// Set optional per-edge metadata format version.
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Build edge metadata and validate it.
    pub fn build(self) -> crate::Result<EdgeMeta> {
        let meta = self.build_unchecked();
        meta.validate("edge")?;
        Ok(meta)
    }

    /// Build edge metadata without validation.
    pub fn build_unchecked(self) -> EdgeMeta {
        EdgeMeta::new(
            self.triplet,
            self.directed,
            self.chunk_sizes,
            self.prefix,
            self.adj_lists,
            self.property_groups,
            self.version,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::meta::{
        AdjListMeta, AdjListType, DataType, EdgeChunkSizes, EdgeTriplet, FileType,
        GraphMetaBuilder, PropertyCardinality, PropertyGroupMeta, PropertyMeta, Version,
        VertexMetaBuilder,
    };

    fn property_group(name: &str) -> PropertyGroupMeta {
        PropertyGroupMeta::new(
            "",
            FileType::Parquet,
            vec![PropertyMeta::new(
                name.into(),
                DataType::Int64,
                false,
                false,
                PropertyCardinality::Single,
            )],
        )
    }

    #[test]
    fn edge_builder_defaults_behave_as_documented() {
        let edge = crate::meta::EdgeMetaBuilder::new(
            EdgeTriplet::new("person".into(), "knows".into(), "person".into()),
            EdgeChunkSizes::new(1024, 128, 128),
            vec![AdjListMeta::new(AdjListType::OrderedBySource, "")],
        )
        .build()
        .expect("edge builder with required fields should be valid");

        assert!(edge.directed());
        assert_eq!(edge.prefix(), "person_knows_person");
        assert_eq!(edge.version(), None);
    }

    #[test]
    fn builder_defaults_behave_as_documented() {
        let vertex = VertexMetaBuilder::new("person".into(), 128)
            .add_property_group(property_group("id"))
            .build()
            .expect("vertex builder default should be valid");
        assert_eq!(vertex.prefix(), "person");
        assert_eq!(vertex.version(), None);

        let graph = GraphMetaBuilder::new("social".into(), "graphs/social/")
            .add_vertex(vertex.clone())
            .add_edge(
                crate::meta::EdgeMetaBuilder::new(
                    EdgeTriplet::new("person".into(), "knows".into(), "person".into()),
                    EdgeChunkSizes::new(1024, 128, 128),
                    vec![AdjListMeta::new(AdjListType::OrderedBySource, "")],
                )
                .add_property_group(property_group("weight"))
                .build()
                .expect("edge should be valid"),
            )
            .build()
            .expect("graph builder default should be valid");

        assert_eq!(graph.version(), Version::V1);
    }
}
