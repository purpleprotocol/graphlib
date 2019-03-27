// Copyright 2019 Octavian Oncescu

use rand;
use rand::Rng;
use rand::SeedableRng;
use rand_core::RngCore;
use rand_isaac::IsaacRng;

use core::sync::atomic::Ordering;

use super::SEED;

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

        //bytes = IsaacRng::gen::<[u8; 16]>(&mut IsaacRng::seed_from_u64(9));

        let bytes = IsaacRng::gen::<[u8; 16]>(&mut IsaacRng::seed_from_u64(IsaacRng::next_u64(
            &mut IsaacRng::seed_from_u64(SEED.load(Ordering::Relaxed) as u64),
        )));

        // change global variable to create new random the next time
        SEED.fetch_add(1, Ordering::Relaxed);

        VertexId(bytes)
    }
}
