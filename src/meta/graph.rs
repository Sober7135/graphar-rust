use crate::meta::{EdgeMeta, EdgeTriplet, EdgeTripletRef, GraphName, Version, VertexMeta};

/// Metadata root of a GraphAr graph.
///
/// This type mirrors one graph info artifact:
/// - `name` maps to artifact field `name`.
/// - `prefix` maps to `prefix`.
/// - `version` maps to `version`.
/// - `labels` are graph-level tags.
/// - `vertices` stores vertex entries keyed by artifact `type`.
/// - `edges` stores edge entries keyed by artifact
///   `src_type`/`edge_type`/`dst_type`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphMeta {
    /// Human-readable graph name (`name` in artifacts).
    name: GraphName,
    /// Root path prefix of the graph (`prefix` in artifacts).
    prefix: Box<str>,
    /// GraphAr format version (`version` in artifacts).
    version: Version,
    /// Optional graph-level tags.
    labels: Vec<Box<str>>,
    /// Vertex metadata entries.
    vertices: Vec<VertexMeta>,
    /// Edge metadata entries.
    edges: Vec<EdgeMeta>,
}

impl GraphMeta {
    /// Create graph metadata.
    ///
    /// # Parameters
    ///
    /// `name` maps to artifact field `name`.
    /// `prefix` maps to `prefix`.
    /// `version` maps to `version`.
    /// `labels` are graph-level tags (for dataset/classification metadata),
    /// not vertex type labels.
    /// `vertices` maps to artifact vertex entries whose identity is `type`.
    /// `edges` maps to artifact edge entries whose identity is
    /// `src_type`/`edge_type`/`dst_type`.
    ///
    /// This constructor does not validate metadata invariants.
    /// Call [`Self::validate`] before relying on uniqueness/reference constraints.
    pub fn new(
        name: GraphName,
        prefix: impl Into<Box<str>>,
        version: Version,
        labels: Vec<Box<str>>,
        vertices: Vec<VertexMeta>,
        edges: Vec<EdgeMeta>,
    ) -> Self {
        Self {
            name,
            prefix: prefix.into(),
            version,
            labels,
            vertices,
            edges,
        }
    }

    /// Return graph name.
    pub fn name(&self) -> &GraphName {
        &self.name
    }

    /// Return graph prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Return graph format version.
    pub fn version(&self) -> Version {
        self.version
    }

    /// Return graph-level labels (tags).
    ///
    /// These labels describe the graph itself and are independent from
    /// [`VertexMeta::label`](crate::meta::VertexMeta::label).
    pub fn labels(&self) -> &[Box<str>] {
        &self.labels
    }

    /// Return all vertex metadata entries.
    pub fn vertices(&self) -> &[VertexMeta] {
        &self.vertices
    }

    /// Return all edge metadata entries.
    pub fn edges(&self) -> &[EdgeMeta] {
        &self.edges
    }

    /// Find a vertex metadata entry by label.
    ///
    /// If metadata has not been validated and duplicate labels exist, this
    /// returns the first matching entry.
    pub fn vertex(&self, label: &str) -> Option<&VertexMeta> {
        self.vertices
            .iter()
            .find(|meta| meta.label().as_str() == label)
    }

    /// Find an edge metadata entry by `(src, edge, dst)` triplet.
    ///
    /// These arguments correspond to artifact fields
    /// `src_type`/`edge_type`/`dst_type`.
    ///
    /// If metadata has not been validated and duplicate triplets exist, this
    /// returns the first matching entry.
    pub fn edge(&self, src: &str, edge: &str, dst: &str) -> Option<&EdgeMeta> {
        self.edge_by_triplet_ref(EdgeTripletRef::new(src, edge, dst))
    }

    /// Find an edge metadata entry by [`EdgeTriplet`].
    pub fn edge_by_triplet(&self, triplet: &EdgeTriplet) -> Option<&EdgeMeta> {
        self.edge_by_triplet_ref(triplet.as_triplet_ref())
    }

    /// Find an edge metadata entry by [`EdgeTripletRef`].
    ///
    /// If metadata has not been validated and duplicate triplets exist, this
    /// returns the first matching entry.
    pub fn edge_by_triplet_ref(&self, triplet: EdgeTripletRef<'_>) -> Option<&EdgeMeta> {
        self.edges
            .iter()
            .find(|meta| meta.triplet().as_triplet_ref() == triplet)
    }

    /// Validate graph metadata and return a typed error on failure.
    pub fn validate(&self) -> crate::Result<()> {
        super::validate::validate_graph(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::meta::{
        AdjListMeta, AdjListType, DataType, EdgeChunkSizes, EdgeMeta, EdgeTriplet, EdgeTripletRef,
        FileType, GraphMeta, PropertyCardinality, PropertyGroupMeta, PropertyMeta, Version,
        VertexMeta,
    };

    fn property_group() -> PropertyGroupMeta {
        PropertyGroupMeta::new(
            "",
            FileType::Parquet,
            vec![PropertyMeta::new(
                "id".into(),
                DataType::Int64,
                true,
                false,
                PropertyCardinality::Single,
            )],
        )
    }

    fn vertex(label: &str) -> VertexMeta {
        VertexMeta::new(label.into(), 128, vec![], "", vec![property_group()], None)
    }

    fn edge(src: &str, edge: &str, dst: &str) -> EdgeMeta {
        EdgeMeta::new(
            EdgeTriplet::new(src.into(), edge.into(), dst.into()),
            true,
            EdgeChunkSizes::new(1024, 128, 128),
            "",
            vec![AdjListMeta::new(AdjListType::OrderedBySource, "")],
            vec![property_group()],
            None,
        )
    }

    #[test]
    fn vertex_and_edge_lookup_by_index() {
        let graph = GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("person"), vertex("comment")],
            vec![edge("person", "writes", "comment")],
        );

        assert_eq!(graph.vertex("person").unwrap().label().as_str(), "person");
        assert_eq!(graph.vertex("unknown"), None);

        let edge = graph.edge("person", "writes", "comment").unwrap();
        assert_eq!(edge.edge().as_str(), "writes");
        assert_eq!(graph.edge("person", "likes", "comment"), None);

        let triplet = EdgeTriplet::new("person".into(), "writes".into(), "comment".into());
        assert_eq!(graph.edge_by_triplet(&triplet).unwrap().triplet(), &triplet);
        assert_eq!(
            graph
                .edge_by_triplet_ref(EdgeTripletRef::new("person", "writes", "comment"))
                .unwrap()
                .triplet(),
            &triplet
        );
    }

    #[test]
    fn lookup_returns_first_match_when_not_validated() {
        let first = edge("person", "writes", "comment");
        let second = EdgeMeta::new(
            EdgeTriplet::new("person".into(), "writes".into(), "comment".into()),
            false,
            EdgeChunkSizes::new(2048, 128, 128),
            "writes_alt/",
            vec![AdjListMeta::new(AdjListType::OrderedBySource, "")],
            vec![property_group()],
            None,
        );
        let graph = GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("person"), vertex("comment")],
            vec![first, second],
        );

        let hit = graph.edge("person", "writes", "comment").unwrap();
        assert_eq!(hit.prefix(), "person_writes_comment");
        assert!(graph.validate().is_err());
    }
}
