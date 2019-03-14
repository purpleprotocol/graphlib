## Graphlib 
Graphlib is a simple and powerful rust library for the graph data-structure that is optimized for high churn environments (where the graph mutates often). It provides a simple api for manipulating and for interacting with graphs.

### Usage
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