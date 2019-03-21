// Copyright 2019 Octavian Oncescu

use crate::vertex_id::VertexId;
use std::sync::Arc;
use std::hash::Hasher;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Edge {
    inbound: Arc<VertexId>,
    outbound: Arc<VertexId>,
    weight: f32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.inbound == other.inbound && self.outbound == other.outbound
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inbound.hash(state);
        self.outbound.hash(state);
    }
}

impl Edge {
    pub fn new(outbound: Arc<VertexId>, inbound: Arc<VertexId>) -> Edge {
        Edge {
            inbound,
            outbound,
            weight: 0.0,
        }
    }

    /// Returns true if the given vertex ids are the
    /// inbound and outbound vertices of the edge.
    pub fn matches(&self, a: &VertexId, b: &VertexId) -> bool {
        a == self.outbound.as_ref() && b == self.inbound.as_ref()
    }

    /// Returns true if either the inbound or outbound
    /// vertex is matching the given `VertexId`.
    pub fn matches_any(&self, id: &VertexId) -> bool {
        id == self.inbound.as_ref() || id == self.outbound.as_ref()
    }
}
