//! Core metadata identifiers and enums.

mod builders;
mod data_type;
mod edge;
mod graph;
mod ids;
mod property;
mod types;
mod validate;
mod vertex;

pub use builders::{EdgeMetaBuilder, GraphMetaBuilder, VertexMetaBuilder};
pub use data_type::DataType;
pub use edge::{AdjListMeta, EdgeChunkSizes, EdgeMeta};
pub use graph::GraphMeta;
pub use ids::{
    EdgeLabel, EdgeTriplet, EdgeTripletRef, GraphName, PropertyName, VertexId, VertexLabel,
};
pub use property::{PropertyCardinality, PropertyGroupMeta, PropertyMeta};
pub use types::{AdjListType, FileType, Version};
pub use validate::validate_graph;
pub use vertex::VertexMeta;
