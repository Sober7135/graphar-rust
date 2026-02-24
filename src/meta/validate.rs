use std::collections::HashSet;

use crate::meta::GraphMeta;

/// Validate graph metadata and return the first actionable error.
pub fn validate_graph(graph: &GraphMeta) -> crate::Result<()> {
    // Graph-level identity and root storage prefix must be present.
    if graph.name().as_str().is_empty() {
        return Err(crate::Error::invalid_metadata(
            "graph.name must not be empty",
        ));
    }
    if graph.prefix().is_empty() {
        return Err(crate::Error::invalid_metadata(
            "graph.prefix must not be empty",
        ));
    }

    let mut vertex_labels = HashSet::with_capacity(graph.vertices().len());
    for (idx, vertex) in graph.vertices().iter().enumerate() {
        vertex.validate(&format!("graph.vertices[{idx}]"))?;
        // Enforce one metadata entry per vertex type label.
        if !vertex_labels.insert(vertex.label().as_str()) {
            return Err(crate::Error::invalid_metadata(format!(
                "graph.vertices has duplicate label `{}`",
                vertex.label().as_str()
            )));
        }
    }

    let mut edge_triplets = HashSet::with_capacity(graph.edges().len());
    for (idx, edge) in graph.edges().iter().enumerate() {
        edge.validate(&format!("graph.edges[{idx}]"))?;
        // Edge endpoints must reference defined vertex types.
        if !vertex_labels.contains(edge.src().as_str()) {
            return Err(crate::Error::invalid_metadata(format!(
                "graph.edges[{idx}].src references undefined vertex label `{}`",
                edge.src().as_str()
            )));
        }
        if !vertex_labels.contains(edge.dst().as_str()) {
            return Err(crate::Error::invalid_metadata(format!(
                "graph.edges[{idx}].dst references undefined vertex label `{}`",
                edge.dst().as_str()
            )));
        }

        // Enforce one metadata entry per edge triplet.
        let triplet = edge.triplet();
        if !edge_triplets.insert(triplet.clone()) {
            return Err(crate::Error::invalid_metadata(format!(
                "graph.edges has duplicate triplet `{triplet}`"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::meta::{
        AdjListMeta, AdjListType, DataType, EdgeChunkSizes, EdgeMeta, EdgeTriplet, FileType,
        GraphMeta, PropertyCardinality, PropertyGroupMeta, PropertyMeta, Version, VertexMeta,
    };

    fn pg() -> PropertyGroupMeta {
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

    fn vertex(label: &str, chunk_size: usize) -> VertexMeta {
        VertexMeta::new(label.into(), chunk_size, vec![], "", vec![pg()], None)
    }

    fn edge(src: &str, e: &str, dst: &str, chunk_size: usize) -> EdgeMeta {
        EdgeMeta::new(
            EdgeTriplet::new(src.into(), e.into(), dst.into()),
            true,
            EdgeChunkSizes::new(chunk_size, 16, 16),
            "",
            vec![AdjListMeta::new(AdjListType::OrderedBySource, "")],
            vec![pg()],
            None,
        )
    }

    fn minimal_graph() -> GraphMeta {
        GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("person", 128)],
            vec![edge("person", "knows", "person", 1024)],
        )
    }

    #[test]
    fn validate_succeeds_for_minimal_graph() {
        let graph = minimal_graph();
        assert!(super::validate_graph(&graph).is_ok());
    }

    #[test]
    fn duplicate_vertex_label_is_invalid() {
        let graph = GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("person", 128), vertex("person", 256)],
            vec![edge("person", "knows", "person", 1024)],
        );
        let err = super::validate_graph(&graph).unwrap_err();
        match err {
            crate::Error::InvalidMetadata { message } => {
                assert!(message.contains("duplicate label"));
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn duplicate_edge_triplet_is_invalid() {
        let graph = GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("person", 128)],
            vec![
                edge("person", "knows", "person", 1024),
                edge("person", "knows", "person", 2048),
            ],
        );
        let err = super::validate_graph(&graph).unwrap_err();
        match err {
            crate::Error::InvalidMetadata { message } => {
                assert!(message.contains("duplicate triplet"));
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn triplets_with_underscores_do_not_collide() {
        // Guard against key-collision bugs from naive string concatenation.
        // Distinct triplets like `(a_b, c, d)` and `(a, b_c, d)` must remain
        // distinct when checking uniqueness.
        let graph = GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("a_b", 128), vertex("a", 128), vertex("d", 128)],
            vec![edge("a_b", "c", "d", 1024), edge("a", "b_c", "d", 1024)],
        );
        assert!(super::validate_graph(&graph).is_ok());
    }

    #[test]
    fn edge_with_undefined_endpoint_is_invalid() {
        let graph = GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("person", 128)],
            vec![edge("person", "knows", "ghost", 1024)],
        );
        let err = super::validate_graph(&graph).unwrap_err();
        match err {
            crate::Error::InvalidMetadata { message } => {
                assert!(message.contains("references undefined vertex label"));
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn zero_chunk_size_is_invalid() {
        let graph = GraphMeta::new(
            "social".into(),
            "graphs/social/",
            Version::V1,
            vec![],
            vec![vertex("person", 0)],
            vec![edge("person", "knows", "person", 1024)],
        );
        let err = super::validate_graph(&graph).unwrap_err();
        match err {
            crate::Error::InvalidMetadata { message } => {
                assert!(message.contains("chunk_size must be > 0"));
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn default_prefixes_match_expected_shape() {
        let vertex = vertex("person", 128);
        assert_eq!(vertex.prefix(), "person");

        let edge = edge("person", "knows", "person", 1024);
        assert_eq!(edge.prefix(), "person_knows_person");

        let group = pg();
        assert_eq!(group.prefix(), "id/");
    }
}
