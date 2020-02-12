// Copyright 2019 Octavian Oncescu

#![cfg_attr(feature = "no_std", feature(alloc))]
#![cfg_attr(feature = "no_std", no_std)]

//! # Graphlib
//! Graphlib is a simple and powerful Rust graph library.
//!
//! ---
//!
//! This library attempts to provide a generic api for building, mutating and iterating over graphs that is similar to that of other data-structures in rust i.e. `Vec`, `HashMap`, `VecDeque`, etc.
//!
//! ### Usage
//! ```rust
//! use graphlib::Graph;
//!
//! let mut graph: Graph<usize> = Graph::new();
//!
//! // Add two vertices to the graph
//! let id1 = graph.add_vertex(1);
//! let id2 = graph.add_vertex(2);
//!
//! // Add an edge between the two vertices
//! graph.add_edge(&id1, &id2);
//!
//! assert_eq!(*graph.fetch(&id1).unwrap(), 1);
//! assert_eq!(*graph.fetch(&id2).unwrap(), 2);
//!
//! // The graph has 2 vertices and one edge at this point
//! assert_eq!(graph.vertex_count(), 2);
//! assert_eq!(graph.edge_count(), 1);
//!
//! // Remove one of the connected vertices
//! graph.remove(&id1);
//!
//! assert_eq!(graph.vertex_count(), 1);
//! assert_eq!(graph.edge_count(), 0);
//! ```

#![allow(mutable_transmutes)]

mod edge;
#[macro_use]
mod macros;
mod graph;
pub mod iterators;
mod vertex_id;

// use global variables to create VertexId::random()
use core::sync::atomic::AtomicUsize;

#[cfg(feature = "dot")]
pub mod dot;

pub use graph::*;
pub use vertex_id::*;

static SEED: AtomicUsize = AtomicUsize::new(0);

use rand;
use rand::Rng;
use rand::SeedableRng;
use rand_core::RngCore;
use rand_isaac::IsaacRng;

use core::sync::atomic::Ordering;

pub(crate) fn gen_bytes() -> [u8; 16] {
    IsaacRng::gen::<[u8; 16]>(&mut IsaacRng::seed_from_u64(IsaacRng::next_u64(
        &mut IsaacRng::seed_from_u64(SEED.fetch_add(1, Ordering::Relaxed) as u64),
    )))
}

#[cfg(feature = "no_std")]
#[macro_use]
extern crate alloc;
