// Copyright 2019 Octavian Oncescu

// Need to set the no_std attribute to avoid
// compile_error! { "ink! only supports compilation as `std` or `no_std` + `wasm32-unknown`" }
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

// # Graphlib
// Graphlib is a simple and powerful Rust graph library.
//
// ---
//
// This library attempts to provide a generic api for building, mutating and
// iterating over graphs that is similar to that of other data-structures
// in rust i.e. `Vec`, `HashMap`, `VecDeque`, etc.
//

mod edge;
#[macro_use]
mod macros;
mod graph;
pub mod iterators;
mod vertex_id;

#[cfg(feature = "dot")]
pub mod dot;

pub use graph::*;
pub use vertex_id::*;
