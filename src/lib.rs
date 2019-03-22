// Copyright 2019 Octavian Oncescu

#![cfg_attr(feature = "no_std", feature(alloc))]
#![cfg_attr(feature = "no_std", no_std)]

//! # Graphlib
//! Graphlib is a simple and powerful rust library for the graph data-structure.
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

mod edge;
#[macro_use]
mod macros;
mod graph;
pub mod iterators;
mod vertex_id;

// use global variables to create VertexId::random()
use core::sync::atomic::AtomicUsize;

#[cfg(feature="dot")]
pub mod dot;

pub use graph::*;
pub use vertex_id::*;

static SEED: AtomicUsize = AtomicUsize::new(0);
