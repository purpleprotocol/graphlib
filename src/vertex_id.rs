// Copyright 2019 Octavian Oncescu

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Id of a vertex
pub struct VertexId([u8; 16]); // 128bit

impl core::fmt::Debug for VertexId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "VertexId({})", hex::encode(self.0))
    }
}

impl core::convert::AsRef<VertexId> for VertexId {
    fn as_ref(&self) -> &VertexId {
        &self
    }
}

impl VertexId {
    pub fn random() -> VertexId {
        VertexId(super::gen_bytes())
    }

    pub fn bytes(&self) -> &[u8; 16] {
        &self.0
    }
}
