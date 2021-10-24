// Copyright 2019 Octavian Oncescu

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Id of a vertex
pub struct VertexId([u8; 16]); // 128bit

impl core::fmt::Debug for VertexId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut buff: [u8; 32] = [0 as u8; 32];
        let _ = hex::encode_to_slice(self.0, &mut buff);
        let s = core::str::from_utf8(&buff).unwrap();
        write!(f, "VertexId({})", s)
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
