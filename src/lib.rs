#![deny(missing_docs)]

//! Rust-native GraphAr core crate.
//!
//! This stage provides foundational metadata types used across later modules.

/// Unified error definitions used by currently implemented APIs.
pub mod error;
/// Core metadata identifiers and enums used across the crate.
pub mod meta;

pub use crate::error::Error;
pub use crate::error::Result;
pub use crate::meta::AdjListMeta;
pub use crate::meta::AdjListType;
pub use crate::meta::DataType;
pub use crate::meta::EdgeChunkSizes;
pub use crate::meta::EdgeLabel;
pub use crate::meta::EdgeMeta;
pub use crate::meta::EdgeMetaBuilder;
pub use crate::meta::EdgeTriplet;
pub use crate::meta::EdgeTripletRef;
pub use crate::meta::FileType;
pub use crate::meta::GraphMeta;
pub use crate::meta::GraphMetaBuilder;
pub use crate::meta::GraphName;
pub use crate::meta::PropertyCardinality;
pub use crate::meta::PropertyGroupMeta;
pub use crate::meta::PropertyMeta;
pub use crate::meta::PropertyName;
pub use crate::meta::Version;
pub use crate::meta::VertexId;
pub use crate::meta::VertexLabel;
pub use crate::meta::VertexMeta;
pub use crate::meta::VertexMetaBuilder;
pub use crate::meta::validate_graph;
