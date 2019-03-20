#[macro_use]
extern crate criterion;

use criterion::Criterion;
use graphlib::*;

// includes benches for :
// 1. new() -> Graph<T>
// 2. with_capacity(capacity: usize) -> Graph<T>
fn bench_create(c: &mut Criterion) {
    c.bench_function("new", |b| b.iter(|| Graph::<usize>::new()));
    c.bench_function("with_capacity", |b| {
        b.iter(|| Graph::<usize>::with_capacity(7))
    });
}

// includes benches for :
// 1. add_edge(&mut self, a: &VertexId, b: &VertexId) -> Result<(), GraphErr>
// 2. add_vertex(&mut self, item: T) -> VertexId
// 3. capacity(&self) -> usize
// 4. edge_count(&self) -> usize
// 5. fetch(&self, id: &VertexId) -> Option<&T>
// 6. fetch_mut(&mut self, id: &VertexId) -> Option<&mut T>
// 7. fold<A>(&self, initial: A, fun: impl Fn(&T, A) -> A) -> A
// 8. has_edge(&self, a: &VertexId, b: &VertexId) -> bool
// 9. is_cyclic(&self) -> bool
// 10.remove(&mut self, id: &VertexId)
// 11.remove_edge(&mut self, a: &VertexId, b: &VertexId)
// 12.reserve(&mut self, additional: usize)
// 13.retain(&mut self, fun: impl Fn(&T) -> bool)
// 14.roots_count(&self) -> usize
// 15.shrink_to_fit(&mut self)
// 16.vertex_count(&self) -> usize
fn bench_others(c: &mut Criterion) {
    c.bench_function("add_edge", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(1);
            let v2 = graph.add_vertex(2);

            graph.add_edge(&v1, &v2);
            graph.add_edge(&v1, &v2);
            graph.add_edge(&v1, &v2);
        })
    });
    c.bench_function("add_vertex", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let _id = graph.add_vertex(1);
        })
    });
    c.bench_function("capacity", |b| {
        b.iter(|| {
            let graph: Graph<usize> = Graph::with_capacity(5);

            let _k = graph.capacity();
        })
    });
    c.bench_function("edge_count", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v2, &v3).unwrap();
            graph.add_edge(&v3, &v4).unwrap();

            let _k = graph.edge_count();
        })
    });
    c.bench_function("fetch", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let id = graph.add_vertex(1);

            let _k = *graph.fetch(&id).unwrap();
        })
    });
    c.bench_function("fetch_mut", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let id = graph.add_vertex(1);

            let _v = graph.fetch_mut(&id).unwrap();
        })
    });
    c.bench_function("fold", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            graph.add_vertex(1);
            graph.add_vertex(2);
            graph.add_vertex(3);

            let _result = graph.fold(0, |v, acc| v + acc);
        })
    });
    c.bench_function("has_edge", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(1);
            let v2 = graph.add_vertex(2);
            let v3 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();

            assert!(graph.has_edge(&v1, &v2));
            assert!(!graph.has_edge(&v2, &v3));
        })
    });
    c.bench_function("is_cyclic", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v2, &v3).unwrap();
            graph.add_edge(&v3, &v4).unwrap();

            let a = graph.is_cyclic();

            graph.add_edge(&v3, &v1);

            let k = graph.is_cyclic();
        })
    });
    c.bench_function("remove", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(1);
            let v2 = graph.add_vertex(2);
            let v3 = graph.add_vertex(3);

            graph.remove(&v2);
            graph.remove(&v1);
            graph.remove(&v3);
        })
    });
    c.bench_function("remove_edge", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v2, &v3).unwrap();
            graph.add_edge(&v3, &v4).unwrap();

            assert_eq!(graph.edge_count(), 3);

            graph.remove_edge(&v2, &v3);
            graph.remove_edge(&v2, &v3);
            graph.remove_edge(&v2, &v3);
        })
    });
    c.bench_function("reserve", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::with_capacity(3);

            graph.add_vertex(0);
            graph.add_vertex(1);
            graph.add_vertex(2);

            graph.reserve(10);
        })
    });
    c.bench_function("retain", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            graph.add_vertex(1);
            graph.add_vertex(2);
            graph.add_vertex(2);
            graph.add_vertex(2);
            graph.add_vertex(3);

            graph.retain(|v| *v != 2);
        })
    });
    c.bench_function("roots_count", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();

            assert_eq!(graph.roots_count(), 1);
        })
    });
    c.bench_function("shrink_to_fit", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::with_capacity(5);

            graph.shrink_to_fit();
        })
    });
    c.bench_function("vertex_count", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            graph.add_vertex(1);
            graph.add_vertex(2);
            graph.add_vertex(3);

            assert_eq!(graph.vertex_count(), 3);
        })
    });
}

// includes benches for :
// 1. dfs(&self) -> Dfs<T>
// 2. bfs(&self) -> Bfs<T>
// 3. vertices(&self) -> VertexIter
// 4. roots(&self) -> VertexIter
fn bench_iterators(c: &mut Criterion) {
    c.bench_function("dfs", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let mut vertices = vec![];

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();

            for v in graph.dfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("bfs", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let mut vertices = vec![];

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);
            let v5 = graph.add_vertex(4);
            let v6 = graph.add_vertex(5);
            let v7 = graph.add_vertex(6);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();
            graph.add_edge(&v1, &v7).unwrap();
            graph.add_edge(&v2, &v5).unwrap();
            graph.add_edge(&v5, &v6).unwrap();

            for v in graph.bfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("vertices", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let mut vertices = vec![];

            let _v1 = graph.add_vertex(0);
            let _v2 = graph.add_vertex(1);
            let _v3 = graph.add_vertex(2);
            let _v4 = graph.add_vertex(3);

            for v in graph.vertices() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("roots", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let mut roots = vec![];

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();

            for v in graph.roots() {
                roots.push(v);
            }
        })
    });
}

// includes benches for :
// 1. in_neighbors(&self, id: &VertexId) -> VertexIter
// 2. in_neighbors_count(&self, id: &VertexId) -> usize
// 3. neighbors(&self, id: &VertexId) -> VertexIter
// 4. neighbors_count(&self, id: &VertexId) -> usize
// 5. out_neighbors(&self, id: &VertexId) -> VertexIter
// 6. out_neighbors_count(&self, id: &VertexId) -> usize
fn bench_neighbor_functions(c: &mut Criterion) {
    c.bench_function("neighbors_count", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();
        })
    });
    c.bench_function("in_neighbors_count", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();
        })
    });
    c.bench_function("out_neighbors_count", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);
            let v5 = graph.add_vertex(4);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();
            graph.add_edge(&v2, &v5).unwrap();
            graph.add_edge(&v2, &v3).unwrap();
        })
    });
    c.bench_function("in_neighbors", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let mut neighbors = vec![];

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();

            for v in graph.in_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("out_neighbors", |b| {
        b.iter(|| {
            use graphlib::Graph;

            let mut graph: Graph<usize> = Graph::new();
            let mut neighbors = vec![];

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();

            for v in graph.out_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("neighbors", |b| {
        b.iter(|| {
            let mut graph: Graph<usize> = Graph::new();
            let mut neighbors = vec![];

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

            graph.add_edge(&v1, &v2).unwrap();
            graph.add_edge(&v3, &v1).unwrap();
            graph.add_edge(&v1, &v4).unwrap();

            for v in graph.neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_create,
    bench_iterators,
    bench_neighbor_functions,
    bench_others
);
criterion_main!(benches);
