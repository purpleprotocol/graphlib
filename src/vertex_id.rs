// Copyright 2019 Octavian Oncescu

use rand::Rng;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct VertexId([u8; 12]);

impl VertexId {
    pub fn random() -> VertexId {
        let bytes = rand::thread_rng().gen::<[u8; 12]>();
        VertexId(bytes)
    }
}
