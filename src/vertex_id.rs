// Copyright 2019 Octavian Oncescu

use rand;
use rand::Rng;
use rand::SeedableRng;
use rand_core::RngCore;
use rand_isaac::IsaacRng;

use core::sync::atomic::Ordering;
use super::SEED;

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