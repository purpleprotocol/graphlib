// Copyright 2019 Octavian Oncescu

use crate::vertex_id::VertexId;
#[cfg(not(feature = "no_std"))]
use std::hash::Hash;
#[cfg(not(feature = "no_std"))]
use std::hash::Hasher;

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
use core::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
/// Edge internal struct
pub struct Edge {
    inbound: VertexId,
    outbound: VertexId,
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
    pub fn new(outbound: VertexId, inbound: VertexId) -> Edge {
        Edge {
            inbound,
            outbound,
        }
    }

    /// Returns true if the given vertex ids are the
    /// inbound and outbound vertices of the edge.
    pub(crate) fn matches(&self, a: &VertexId, b: &VertexId) -> bool {
        a == &self.outbound && b == &self.inbound
    }

    /// Returns true if either the inbound or outbound
    /// vertex is matching the given `VertexId`.
    pub(crate) fn matches_any(&self, id: &VertexId) -> bool {
        id == &self.inbound || id == &self.outbound
    }

    /// Returns the inbound VertexId
    pub(crate) fn inbound(&self) -> &VertexId {
        &self.inbound
    }

    /// Returns the inbound VertexId
    pub(crate) fn outbound(&self) -> &VertexId {
        &self.outbound
    }
}
