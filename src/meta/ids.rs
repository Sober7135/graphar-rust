use std::fmt;

/// Internal vertex identifier used by GraphAr.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexId(u64);

impl VertexId {
    /// Create a vertex identifier from a raw `u64`.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the raw numeric value.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for VertexId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<VertexId> for u64 {
    fn from(value: VertexId) -> Self {
        value.0
    }
}

impl fmt::Display for VertexId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! string_newtype {
    ($name:ident) => {
        #[doc = concat!("A `", stringify!($name), "` string newtype.")]
        #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(Box<str>);

        impl $name {
            #[doc = "Create a value from an owned or borrowed string."]
            pub fn new(value: impl Into<Box<str>>) -> Self {
                Self(value.into())
            }

            #[doc = "Borrow as `&str`."]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value.into_boxed_str())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl From<Box<str>> for $name {
            fn from(value: Box<str>) -> Self {
                Self(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

string_newtype!(GraphName);
string_newtype!(VertexLabel);
string_newtype!(EdgeLabel);
string_newtype!(PropertyName);

/// A borrowed edge identifier triplet `(src, edge, dst)`.
///
/// This view avoids allocation when only borrowed labels are available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeTripletRef<'a> {
    src: &'a str,
    edge: &'a str,
    dst: &'a str,
}

impl<'a> EdgeTripletRef<'a> {
    /// Create a borrowed triplet from source label, edge label, and destination label.
    pub const fn new(src: &'a str, edge: &'a str, dst: &'a str) -> Self {
        Self { src, edge, dst }
    }

    /// Return the source vertex label.
    pub const fn src(self) -> &'a str {
        self.src
    }

    /// Return the edge label.
    pub const fn edge(self) -> &'a str {
        self.edge
    }

    /// Return the destination vertex label.
    pub const fn dst(self) -> &'a str {
        self.dst
    }
}

/// A canonical edge identifier triplet `(src, edge, dst)`.
///
/// In artifacts, it maps to `src_type`/`edge_type`/`dst_type`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeTriplet {
    src: VertexLabel,
    edge: EdgeLabel,
    dst: VertexLabel,
}

impl EdgeTriplet {
    /// Create a triplet from source label, edge label, and destination label.
    pub fn new(src: VertexLabel, edge: EdgeLabel, dst: VertexLabel) -> Self {
        Self { src, edge, dst }
    }

    /// Return the source vertex label.
    pub fn src(&self) -> &VertexLabel {
        &self.src
    }

    /// Return the edge label.
    pub fn edge(&self) -> &EdgeLabel {
        &self.edge
    }

    /// Return the destination vertex label.
    pub fn dst(&self) -> &VertexLabel {
        &self.dst
    }

    /// Borrow this triplet as an [`EdgeTripletRef`].
    pub fn as_triplet_ref(&self) -> EdgeTripletRef<'_> {
        EdgeTripletRef::from(self)
    }
}

impl<'a> From<&'a EdgeTriplet> for EdgeTripletRef<'a> {
    fn from(value: &'a EdgeTriplet) -> Self {
        Self::new(
            value.src().as_str(),
            value.edge().as_str(),
            value.dst().as_str(),
        )
    }
}

impl fmt::Display for EdgeTripletRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}_{}", self.src, self.edge, self.dst)
    }
}

impl fmt::Display for EdgeTriplet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}_{}", self.src, self.edge, self.dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn vertex_id_roundtrip() {
        let id = VertexId::new(42);
        assert_eq!(id.get(), 42);
        assert_eq!(u64::from(id), 42);
        assert_eq!(id.to_string(), "42");
    }

    #[test]
    fn string_newtypes_display_and_conversion() {
        let graph = GraphName::from("social");
        let vertex = VertexLabel::from("person");
        let edge = EdgeLabel::from(String::from("knows"));
        let property = PropertyName::new("name");

        assert_eq!(graph.as_str(), "social");
        assert_eq!(vertex.to_string(), "person");
        assert_eq!(edge.as_ref(), "knows");
        assert_eq!(property.as_str(), "name");
    }

    #[test]
    fn edge_triplet_display() {
        let triplet = EdgeTriplet::new("person".into(), "knows".into(), "person".into());
        assert_eq!(triplet.to_string(), "person_knows_person");
        assert_eq!(triplet.src().as_str(), "person");
        assert_eq!(triplet.edge().as_str(), "knows");
        assert_eq!(triplet.dst().as_str(), "person");
    }

    #[test]
    fn edge_triplet_ref_roundtrip() {
        let triplet = EdgeTriplet::new("person".into(), "knows".into(), "person".into());
        let triplet_ref = triplet.as_triplet_ref();
        assert_eq!(triplet_ref.src(), "person");
        assert_eq!(triplet_ref.edge(), "knows");
        assert_eq!(triplet_ref.dst(), "person");
        assert_eq!(triplet_ref.to_string(), "person_knows_person");
    }

    #[test]
    fn public_id_types_are_send_sync() {
        assert_send_sync::<VertexId>();
        assert_send_sync::<GraphName>();
        assert_send_sync::<VertexLabel>();
        assert_send_sync::<EdgeLabel>();
        assert_send_sync::<PropertyName>();
        assert_send_sync::<EdgeTriplet>();
        assert_send_sync::<EdgeTripletRef<'static>>();
    }
}
