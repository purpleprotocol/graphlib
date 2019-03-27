// Copyright 2019 Octavian Oncescu

use rand;
use rand::Rng;
use rand::SeedableRng;
use rand_isaac::IsaacRng;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
/// Id of a vertex
pub struct VertexId([u8; 16]); // 128bit

impl VertexId {
    pub fn random() -> VertexId {
        /*
        use rand;
        use rand::Rng;
        use rand_hc::Hc128Rng;
        use rand::SeedableRng;

        let bytes = Hc128Rng::gen::<[u8; 16]>(&mut Hc128Rng::seed_from_u64(6);
        */
        let bytes = IsaacRng::gen::<[u8; 16]>(&mut IsaacRng::seed_from_u64(9));

        VertexId(bytes)
    }
}
