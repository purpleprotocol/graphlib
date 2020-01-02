# Graphlib 
[![Build Status]][travis] [![Discord Badge]][Discord] [![Latest Version]][crates.io] [![Documentation]][docs.rs] 

Graphlib is a simple and powerful Rust graph library. 

---

This library attempts to provide a generic api for building, mutating and iterating over graphs that is similar to that of other data-structures found in Rust i.e. `Vec`, `HashMap`, `VecDeque`, etc. 

### Using Graphlib
```rust
use graphlib::Graph;

let mut graph: Graph<usize> = Graph::new();

// Add two vertices to the graph
let id1 = graph.add_vertex(1);
let id2 = graph.add_vertex(2);

// Add an edge between the two vertices
graph.add_edge(&id1, &id2);

assert_eq!(*graph.fetch(&id1).unwrap(), 1);
assert_eq!(*graph.fetch(&id2).unwrap(), 2);

// The graph has 2 vertices and one edge at this point
assert_eq!(graph.vertex_count(), 2);
assert_eq!(graph.edge_count(), 1);

// Remove one of the connected vertices
graph.remove(&id1);

assert_eq!(graph.vertex_count(), 1);
assert_eq!(graph.edge_count(), 0);
```

### Using without `std`
In `Cargo.toml`:
```toml
[dependencies]
graphlib = { version = "*", features = ["no_std"] }
```

### Contributing
We welcome anyone wishing to contribute to Graphlib! Check out the [issues section][issues] of the repository before starting out.

### License

Graphlib is licensed under the MIT license.

[Build Status]: https://travis-ci.org/purpleprotocol/graphlib.svg?branch=master
[Discord Badge]: https://img.shields.io/discord/435827644915777536.svg
[Discord]: https://discord.gg/eGBzyaA
[travis]: https://travis-ci.org/purpleprotocol/graphlib
[crates.io]: https://crates.io/crates/graphlib
[Latest Version]: https://img.shields.io/crates/v/graphlib.svg
[Documentation]: https://docs.rs/graphlib/badge.svg
[docs.rs]: https://docs.rs/graphlib
[issues]: https://github.com/purpleprotocol/graphlib/issues

