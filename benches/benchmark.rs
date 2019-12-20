#[macro_use]
extern crate criterion;

use criterion::Criterion;
use graphlib::*;

// use `cargo bench --features sbench` for benching with GraphCapacity of 10_000_000

// includes benches for :
// 1. new() -> Graph<T>
// 2. with_capacity(capacity: usize) -> Graph<T>
fn bench_create(c: &mut Criterion) {
    c.bench_function("new", |b| b.iter(Graph::<usize>::new));

    c.bench_function("with_capacity_10", |b| {
        b.iter(|| Graph::<usize>::with_capacity(10))
    });
    c.bench_function("with_capacity_100", |b| {
        b.iter(|| Graph::<usize>::with_capacity(100))
    });
    c.bench_function("with_capacity_500", |b| {
        b.iter(|| Graph::<usize>::with_capacity(500))
    });
    c.bench_function("with_capacity_1000", |b| {
        b.iter(|| Graph::<usize>::with_capacity(1000))
    });
    #[cfg(feature = "sbench")]
    c.bench_function("with_capacity_m", |b| {
        b.iter(|| Graph::<usize>::with_capacity(10_000_000))
    });
}

// includes benches for :
// 1. dfs(&self) -> Dfs<T>
// 2. bfs(&self) -> Bfs<T>
// 3. topo(&self) -> Topo<T>
// 4. vertices(&self) -> VertexIter
// 5. roots(&self) -> VertexIter
fn bench_iterators(c: &mut Criterion) {
    c.bench_function("dfs_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.dfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("dfs_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.dfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("dfs_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.dfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("dfs_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.dfs() {
                vertices.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("dfs_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.dfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("bfs_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.bfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("bfs_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.bfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("bfs_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.bfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("bfs_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.bfs() {
                vertices.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("bfs_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.bfs() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("topo_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.topo() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("topo_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.topo() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("topo_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.topo() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("topo_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.topo() {
                vertices.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("topo_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.topo() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("vertices_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        for i in 1..=10 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            for v in graph.vertices() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("vertices_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        for i in 1..=100 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            for v in graph.vertices() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("vertices_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        for i in 1..=500 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            for v in graph.vertices() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("vertices_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        for i in 1..=1000 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            for v in graph.vertices() {
                vertices.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("vertices_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        for i in 1..=10_000_000 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            for v in graph.vertices() {
                vertices.push(v);
            }
        })
    });

    c.bench_function("roots_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut roots = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.roots() {
                roots.push(v);
            }
        })
    });

    c.bench_function("roots_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut roots = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.roots() {
                roots.push(v);
            }
        })
    });

    c.bench_function("roots_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut roots = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.roots() {
                roots.push(v);
            }
        })
    });

    c.bench_function("roots_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut roots = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.roots() {
                roots.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("roots_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut roots = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
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
    c.bench_function("neighbors_count_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            let _k = graph.neighbors_count(&v1);
        })
    });

    c.bench_function("neighbors_count_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            let _k = graph.neighbors_count(&v1);
        })
    });

    c.bench_function("neighbors_count_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            let _k = graph.neighbors_count(&v1);
        })
    });

    c.bench_function("neighbors_count_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            let _k = graph.neighbors_count(&v1);
        })
    });

    #[cfg(feature = "sbench")]
    c.bench_function("neighbors_count_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            let _k = graph.neighbors_count(&v1);
        })
    });

    c.bench_function("in_neighbors_count_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.in_neighbors_count(&v1);
        })
    });

    c.bench_function("in_neighbors_count_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.in_neighbors_count(&v1);
        })
    });

    c.bench_function("in_neighbors_count_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.in_neighbors_count(&v1);
        })
    });

    c.bench_function("in_neighbors_count_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.in_neighbors_count(&v1);
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("in_neighbors_count_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.in_neighbors_count(&v1);
        })
    });

    c.bench_function("out_neighbors_count_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.out_neighbors_count(&v1);
        })
    });
    c.bench_function("out_neighbors_count_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.out_neighbors_count(&v1);
        })
    });
    c.bench_function("out_neighbors_count_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.out_neighbors_count(&v1);
        })
    });
    c.bench_function("out_neighbors_count_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.out_neighbors_count(&v1);
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("out_neighbors_count_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.out_neighbors_count(&v1);
        })
    });
    c.bench_function("in_neighbors_10", |b| {
        let mut neighbors = vec![];
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.in_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("in_neighbors_100", |b| {
        let mut neighbors = vec![];
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.in_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("in_neighbors_500", |b| {
        let mut neighbors = vec![];
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.in_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("in_neighbors_1000", |b| {
        let mut neighbors = vec![];
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.in_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("in_neighbors_m", |b| {
        let mut neighbors = vec![];
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.in_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("out_neighbors_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.out_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });

    c.bench_function("out_neighbors_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.out_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("out_neighbors_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.out_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("out_neighbors_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.out_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("out_neighbors_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            for v in graph.out_neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("neighbors_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("neighbors_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });

    c.bench_function("neighbors_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    c.bench_function("neighbors_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.neighbors(&v1) {
                neighbors.push(v);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("neighbors_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut neighbors = vec![];

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }

        b.iter(|| {
            for v in graph.neighbors(&v1) {
                neighbors.push(v);
            }
        })
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
    c.bench_function("add_edge_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=10 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
            }
        })
    });

    c.bench_function("add_edge_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=100 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
            }
        })
    });
    c.bench_function("add_edge_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=500 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
            }
        })
    });
    c.bench_function("add_edge_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=1000 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("add_edge_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=10_000_000 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
            }
        })
    });
    c.bench_function("add_vertex_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=10 {
                graph.add_vertex(i);
            }
        })
    });

    c.bench_function("add_vertex_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=100 {
                graph.add_vertex(i);
            }
        })
    });

    c.bench_function("add_vertex_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=500 {
                graph.add_vertex(i);
            }
        })
    });
    c.bench_function("add_vertex_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=1000 {
                graph.add_vertex(i);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("add_vertex_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=10_000_000 {
                graph.add_vertex(i);
            }
        })
    });

    c.bench_function("capacity_10", |b| {
        let graph: Graph<usize> = Graph::with_capacity(10);
        b.iter(|| {
            let _k = graph.capacity();
        })
    });
    c.bench_function("capacity_100", |b| {
        let graph: Graph<usize> = Graph::with_capacity(100);
        b.iter(|| {
            let _k = graph.capacity();
        })
    });
    c.bench_function("capacity_500", |b| {
        let graph: Graph<usize> = Graph::with_capacity(500);
        b.iter(|| {
            let _k = graph.capacity();
        })
    });

    c.bench_function("capacity_1000", |b| {
        let graph: Graph<usize> = Graph::with_capacity(1000);
        b.iter(|| {
            let _k = graph.capacity();
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("capacity_m", |b| {
        let graph: Graph<usize> = Graph::with_capacity(10_000_000);
        b.iter(|| {
            let _k = graph.capacity();
        })
    });

    c.bench_function("edge_count_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.edge_count();
        })
    });
    c.bench_function("edge_count_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.edge_count();
        })
    });

    c.bench_function("edge_count_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.edge_count();
        })
    });
    c.bench_function("edge_count_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.edge_count();
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("edge_count_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.edge_count();
        })
    });
    c.bench_function("fetch_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..10 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(10);
        b.iter(|| {
            let _k = *graph.fetch(&id).unwrap();
        })
    });
    c.bench_function("fetch_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..100 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(100);
        b.iter(|| {
            let _k = *graph.fetch(&id).unwrap();
        })
    });
    c.bench_function("fetch_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..500 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(500);
        b.iter(|| {
            let _k = *graph.fetch(&id).unwrap();
        })
    });
    c.bench_function("fetch_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..1000 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(1000);
        b.iter(|| {
            let _k = *graph.fetch(&id).unwrap();
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("fetch_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..10_000_000 {
            graph.add_vertex(i);
        }
        let mut id = graph.add_vertex(10_000_000);
        b.iter(|| {
            let _k = *graph.fetch(&id).unwrap();
        })
    });

    c.bench_function("fetch_mut_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        for i in 1..10 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(10);
        b.iter(|| {
            let _v = graph.fetch_mut(&id).unwrap();
        })
    });
    c.bench_function("fetch_mut_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        for i in 1..100 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(100);
        b.iter(|| {
            let _v = graph.fetch_mut(&id).unwrap();
        })
    });
    c.bench_function("fetch_mut_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        for i in 1..500 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(500);
        b.iter(|| {
            let _v = graph.fetch_mut(&id).unwrap();
        })
    });
    c.bench_function("fetch_mut_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        for i in 1..1000 {
            graph.add_vertex(i);
        }
        let id = graph.add_vertex(1000);
        b.iter(|| {
            let _v = graph.fetch_mut(&id).unwrap();
        })
    });

    #[cfg(feature = "sbench")]
    c.bench_function("fetch_mut_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        for i in 1..10_000_000 {
            graph.add_vertex(i);
        }
        let mut id = graph.add_vertex(10_000_000);
        b.iter(|| {
            let _v = graph.fetch_mut(&id).unwrap();
        })
    });
    c.bench_function("fold_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=10 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            let _result = graph.fold(0, |v, acc| v + acc);
        })
    });
    c.bench_function("fold_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=100 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            let _result = graph.fold(0, |v, acc| v + acc);
        })
    });
    c.bench_function("fold_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=500 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            let _result = graph.fold(0, |v, acc| v + acc);
        })
    });
    c.bench_function("fold_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=1000 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            let _result = graph.fold(0, |v, acc| v + acc);
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("fold_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=10_000_000 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            let _result = graph.fold(0, |v, acc| v + acc);
        })
    });
    c.bench_function("has_edge_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);

        for i in 3..=10 {
            graph.add_vertex(i);
        }
        let v3 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        b.iter(|| {
            let _k = graph.has_edge(&v1, &v2);
            let _l = graph.has_edge(&v2, &v3);
        })
    });
    c.bench_function("has_edge_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);

        for i in 3..=100 {
            graph.add_vertex(i);
        }
        let v3 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        b.iter(|| {
            let _k = graph.has_edge(&v1, &v2);
            let _l = graph.has_edge(&v2, &v3);
        })
    });

    c.bench_function("has_edge_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);

        for i in 3..=500 {
            graph.add_vertex(i);
        }
        let v3 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        b.iter(|| {
            let _k = graph.has_edge(&v1, &v2);
            let _l = graph.has_edge(&v2, &v3);
        })
    });

    c.bench_function("has_edge_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);

        for i in 3..=1000 {
            graph.add_vertex(i);
        }
        let v3 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        b.iter(|| {
            let _k = graph.has_edge(&v1, &v2);
            let _l = graph.has_edge(&v2, &v3);
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("has_edge_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);

        for i in 3..=10_000_000 {
            graph.add_vertex(i);
        }
        let v3 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        b.iter(|| {
            let _k = graph.has_edge(&v1, &v2);
            let _l = graph.has_edge(&v2, &v3);
        })
    });
    c.bench_function("is_cyclic_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v0 = graph.add_vertex(0);
        let mut v1 = graph.add_vertex(1);
        let mut v2 = graph.add_vertex(2);
        graph.add_edge(&v0, &v1);
        graph.add_edge(&v1, &v2);
        for i in 4..=10 {
            v1 = v2.clone();
            v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
        }
        graph.add_edge(&v2, &v0);
        b.iter(|| {
            let _k = graph.is_cyclic();
        })
    });
    c.bench_function("is_cyclic_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v0 = graph.add_vertex(0);
        let mut v1 = graph.add_vertex(1);
        let mut v2 = graph.add_vertex(2);
        graph.add_edge(&v0, &v1);
        graph.add_edge(&v1, &v2);
        for i in 4..=100 {
            v1 = v2.clone();
            v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
        }
        graph.add_edge(&v2, &v0);
        b.iter(|| {
            let _k = graph.is_cyclic();
        })
    });
    c.bench_function("is_cyclic_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v0 = graph.add_vertex(0);
        let mut v1 = graph.add_vertex(1);
        let mut v2 = graph.add_vertex(2);
        graph.add_edge(&v0, &v1);
        graph.add_edge(&v1, &v2);
        for i in 4..=500 {
            v1 = v2.clone();
            v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
        }
        graph.add_edge(&v2, &v0);
        b.iter(|| {
            let _k = graph.is_cyclic();
        })
    });
    c.bench_function("is_cyclic_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v0 = graph.add_vertex(0);
        let mut v1 = graph.add_vertex(1);
        let mut v2 = graph.add_vertex(2);
        graph.add_edge(&v0, &v1);
        graph.add_edge(&v1, &v2);
        for i in 4..=1000 {
            v1 = v2.clone();
            v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
        }
        graph.add_edge(&v2, &v0);
        b.iter(|| {
            let _k = graph.is_cyclic();
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("is_cyclic_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        let v0 = graph.add_vertex(0);
        let mut v1 = graph.add_vertex(1);
        let mut v2 = graph.add_vertex(2);
        graph.add_edge(&v0, &v1);
        graph.add_edge(&v1, &v2);
        for i in 4..=10_000_000 {
            v1 = v2.clone();
            v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
        }
        graph.add_edge(&v2, &v0);
        b.iter(|| {
            let _k = graph.is_cyclic();
        })
    });
    c.bench_function("remove_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=10 {
                let v1 = graph.add_vertex(i);
                graph.remove(&v1);
            }
        })
    });
    c.bench_function("remove_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=100 {
                let v1 = graph.add_vertex(i);
                graph.remove(&v1);
            }
        })
    });
    c.bench_function("remove_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=500 {
                let v1 = graph.add_vertex(i);
                graph.remove(&v1);
            }
        })
    });
    c.bench_function("remove_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=1000 {
                let v1 = graph.add_vertex(i);
                graph.remove(&v1);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("remove_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            for i in 1..=10_000_000 {
                let v1 = graph.add_vertex(i);
                graph.remove(&v1);
            }
        })
    });
    c.bench_function("remove_edge_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=10 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
                graph.remove_edge(&v1, &v2);
            }
        })
    });
    c.bench_function("remove_edge_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=100 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
                graph.remove_edge(&v1, &v2);
            }
        })
    });

    c.bench_function("remove_edge_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=500 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
                graph.remove_edge(&v1, &v2);
            }
        })
    });

    c.bench_function("remove_edge_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=1000 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
                graph.remove_edge(&v1, &v2);
            }
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("remove_edge_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        b.iter(|| {
            let mut v1 = graph.add_vertex(0);

            for i in 1..=10_000_000 {
                let v2 = graph.add_vertex(i);
                graph.add_edge(&v1, &v2);
                v1 = v2.clone();
                graph.remove_edge(&v1, &v2);
            }
        })
    });

    c.bench_function("reserve_10", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(10);

        for i in 1..=10 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            graph.reserve(10);
        })
    });
    c.bench_function("reserve_100", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(100);

        for i in 1..=100 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            graph.reserve(100);
        })
    });
    c.bench_function("reserve_500", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(500);

        for i in 1..=500 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            graph.reserve(500);
        })
    });
    c.bench_function("reserve_1000", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(1000);

        for i in 1..=1000 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            graph.reserve(1000);
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("reserve_m", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(10_000_000);

        for i in 1..=10_000_000 {
            graph.add_vertex(i);
        }

        b.iter(|| {
            graph.reserve(10_000_000);
        })
    });
    c.bench_function("retain_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=10 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            graph.retain(|v| *v != 2);
        })
    });
    c.bench_function("retain_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=100 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            graph.retain(|v| *v != 2);
        })
    });
    c.bench_function("retain_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=500 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            graph.retain(|v| *v != 2);
        })
    });
    c.bench_function("retain_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=1000 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            graph.retain(|v| *v != 2);
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("retain_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=10_000_000 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            graph.retain(|v| *v != 2);
        })
    });
    c.bench_function("roots_count_10", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut v1 = graph.add_vertex(0);

        for i in 1..=10 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.roots_count();
        })
    });
    c.bench_function("roots_count_100", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut v1 = graph.add_vertex(0);

        for i in 1..=100 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.roots_count();
        })
    });
    c.bench_function("roots_count_500", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut v1 = graph.add_vertex(0);

        for i in 1..=500 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.roots_count();
        })
    });
    c.bench_function("roots_count_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut v1 = graph.add_vertex(0);

        for i in 1..=1000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.roots_count();
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("roots_count_m", |b| {
        let mut graph: Graph<usize> = Graph::new();
        let mut v1 = graph.add_vertex(0);

        for i in 1..=10_000_000 {
            let v2 = graph.add_vertex(i);
            graph.add_edge(&v1, &v2);
            v1 = v2.clone();
        }
        b.iter(|| {
            let _k = graph.roots_count();
        })
    });

    c.bench_function("shrink_to_fit_10", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(10);

        b.iter(|| {
            graph.shrink_to_fit();
        })
    });
    c.bench_function("shrink_to_fit_100", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(100);

        b.iter(|| {
            graph.shrink_to_fit();
        })
    });
    c.bench_function("shrink_to_fit_500", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(500);

        b.iter(|| {
            graph.shrink_to_fit();
        })
    });
    c.bench_function("shrink_to_fit_1000", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(1000);

        b.iter(|| {
            graph.shrink_to_fit();
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("shrink_to_fit_m", |b| {
        let mut graph: Graph<usize> = Graph::with_capacity(10_000_000);

        b.iter(|| {
            graph.shrink_to_fit();
        })
    });
    c.bench_function("vertex_count_10", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=10 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            let _k = graph.vertex_count();
        })
    });
    c.bench_function("vertex_count_100", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=100 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            let _k = graph.vertex_count();
        })
    });
    c.bench_function("vertex_count_500", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=500 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            let _k = graph.vertex_count();
        })
    });
    c.bench_function("vertex_count_1000", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=1000 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            let _k = graph.vertex_count();
        })
    });
    #[cfg(feature = "sbench")]
    c.bench_function("vertex_count_m", |b| {
        let mut graph: Graph<usize> = Graph::new();

        for i in 1..=10_000_000 {
            graph.add_vertex(i);
        }
        b.iter(|| {
            let _k = graph.vertex_count();
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
