#[macro_use]
extern crate criterion;

use criterion::Criterion;
use graphlib::Graph;

fn bench_create(c: &mut Criterion) {
    c.bench_function("new", |b| b.iter(|| Graph::<usize>::new()));
    c.bench_function("with_capacity", |b| {
        b.iter(|| Graph::<usize>::with_capacity(7))
    });
}

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

            let v1 = graph.add_vertex(0);
            let v2 = graph.add_vertex(1);
            let v3 = graph.add_vertex(2);
            let v4 = graph.add_vertex(3);

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
    bench_neighbor_functions
);
criterion_main!(benches);
