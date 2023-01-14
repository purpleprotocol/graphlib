// Copyright 2019 Octavian Oncescu

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Id of a vertex
// pub struct VertexId([u8; 16]); // 128bit
pub struct VertexId(u32);

impl core::fmt::Debug for VertexId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "VertexId({})", self.0)
    }
}

impl core::convert::AsRef<VertexId> for VertexId {
    fn as_ref(&self) -> &VertexId {
        &self
    }
}

impl VertexId {

    /// This is an unsafe function and generally should not be used!
    /// It's made public for use largely in test contexts.
    /// Otherwise you may risk creating two identical VertexId's in a graph
    /// Use Graph::add_vertex(...) instead
    pub fn new(val: u32) -> Self {
        Self{0: val}
    }

    pub(crate) fn val(&self) -> u32 {
        self.0
    }
}
