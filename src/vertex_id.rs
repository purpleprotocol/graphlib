// Copyright 2019 Octavian Oncescu

use rand::Rng;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct VertexId([u8; 16]); // 128bit

impl VertexId {
    pub fn random() -> VertexId {
        let bytes = rand::thread_rng().gen::<[u8; 16]>();
        VertexId(bytes)
    }
}
