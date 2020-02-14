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

    macro_rules! with_capacity {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| b.iter(|| Graph::<usize>::with_capacity($x)));
        };
    }
    with_capacity!("with_capacity_10", 10);
    with_capacity!("with_capacity_100", 100);
    with_capacity!("with_capacity_500", 500);
    with_capacity!("with_capacity_1000", 1000);
    #[cfg(feature = "sbench")]
    with_capacity!("with_capacity_m", 10_000_000);
}

// includes benches for :
// 1. dfs(&self) -> Dfs<T>
// 2. bfs(&self) -> Bfs<T>
// 3. topo(&self) -> Topo<T>
// 4. vertices(&self) -> VertexIter
// 5. roots(&self) -> VertexIter
fn bench_iterators(c: &mut Criterion) {
    macro_rules! dfs {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut vertices = vec![];

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
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
        };
    }
    dfs!("dfs_10", 10);
    dfs!("dfs_100", 100);
    dfs!("dfs_500", 500);
    dfs!("dfs_1000", 1000);
    #[cfg(feature = "sbench")]
    dfs!("dfs_m", 10_000_000);

    macro_rules! bfs {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut vertices = vec![];

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
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
        };
    }
    bfs!("bfs_10", 10);
    bfs!("bfs_100", 100);
    bfs!("bfs_500", 500);
    bfs!("bfs_1000", 1000);
    #[cfg(feature = "sbench")]
    bfs!("bfs_m", 10_000_000);

    macro_rules! topo {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut vertices = vec![];

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
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
        };
    }
    topo!("topo_10", 10);
    topo!("topo_100", 100);
    topo!("topo_500", 500);
    topo!("topo_1000", 1000);
    #[cfg(feature = "sbench")]
    topo!("topo_m", 10_000_000);

    macro_rules! vertices {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut vertices = vec![];

                for i in 1..=$x {
                    graph.add_vertex(i);
                }

                b.iter(|| {
                    for v in graph.vertices() {
                        vertices.push(v);
                    }
                })
            });
        };
    }
    vertices!("vertices_10", 10);
    vertices!("vertices_100", 100);
    vertices!("vertices_500", 500);
    vertices!("vertices_1000", 1000);
    #[cfg(feature = "sbench")]
    vertices!("vertices_m", 10_000_000);

    macro_rules! roots {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut roots = vec![];

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
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
        };
    }

    roots!("roots_10", 10);
    roots!("roots_100", 100);
    roots!("roots_500", 500);
    roots!("roots_1000", 1000);
    #[cfg(feature = "sbench")]
    roots!("roots_m", 10_000_000);
}

// includes benches for :
// 1. in_neighbors(&self, id: &VertexId) -> VertexIter
// 2. in_neighbors_count(&self, id: &VertexId) -> usize
// 3. neighbors(&self, id: &VertexId) -> VertexIter
// 4. neighbors_count(&self, id: &VertexId) -> usize
// 5. out_neighbors(&self, id: &VertexId) -> VertexIter
// 6. out_neighbors_count(&self, id: &VertexId) -> usize
fn bench_neighbor_functions(c: &mut Criterion) {
    macro_rules! neighbors_count {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
                    let v2 = graph.add_vertex(i);
                    graph.add_edge(&v1, &v2);
                    v1 = v2.clone();
                }

                b.iter(|| {
                    let _k = graph.neighbors_count(&v1);
                })
            });
        };
    }
    neighbors_count!("neighbors_count_10", 10);
    neighbors_count!("neighbors_count_100", 100);
    neighbors_count!("neighbors_count_500", 500);
    neighbors_count!("neighbors_count_1000", 1000);
    #[cfg(feature = "sbench")]
    neighbors_count!("neighbors_count_m", 10_000_000);

    macro_rules! in_neighbors_count {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
                    let v2 = graph.add_vertex(i);
                    graph.add_edge(&v1, &v2);
                    v1 = v2.clone();
                }
                b.iter(|| {
                    let _k = graph.in_neighbors_count(&v1);
                })
            });
        };
    }
    in_neighbors_count!("in_neighbors_count_10", 10);
    in_neighbors_count!("in_neighbors_count_100", 100);
    in_neighbors_count!("in_neighbors_count_500", 500);
    in_neighbors_count!("in_neighbors_count_1000", 1000);
    #[cfg(feature = "sbench")]
    in_neighbors_count!("in_neighbors_count_m", 10_000_000);

    macro_rules! out_neighbors_count {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
                    let v2 = graph.add_vertex(i);
                    graph.add_edge(&v1, &v2);
                    v1 = v2.clone();
                }
                b.iter(|| {
                    let _k = graph.out_neighbors_count(&v1);
                })
            });
        };
    }

    out_neighbors_count!("out_neighbors_count_10", 10);
    out_neighbors_count!("out_neighbors_count_100", 100);
    out_neighbors_count!("out_neighbors_count_500", 500);
    out_neighbors_count!("out_neighbors_count_1000", 1000);
    #[cfg(feature = "sbench")]
    out_neighbors_count!("out_neighbors_count_m", 10_000_000);

    macro_rules! in_neighbors {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut neighbors = vec![];
                let mut graph: Graph<usize> = Graph::new();

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
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
        };
    }
    in_neighbors!("in_neighbors_10", 10);
    in_neighbors!("in_neighbors_100", 100);
    in_neighbors!("in_neighbors_500", 500);
    in_neighbors!("in_neighbors_1000", 1000);
    #[cfg(feature = "sbench")]
    in_neighbors!("in_neighbors_m", 10_000_000);

    macro_rules! out_neighbors {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut neighbors = vec![];

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
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
        };
    }

    out_neighbors!("out_neighbors_10", 10);
    out_neighbors!("out_neighbors_100", 100);
    out_neighbors!("out_neighbors_500", 500);
    out_neighbors!("out_neighbors_1000", 1000);
    #[cfg(feature = "sbench")]
    out_neighbors!("out_neighbors_m", 10_000_000);

    macro_rules! neighbors {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut neighbors = vec![];

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
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
        };
    }
    neighbors!("neighbors_10", 10);
    neighbors!("neighbors_100", 100);
    neighbors!("neighbors_500", 500);
    neighbors!("neighbors_1000", 1000);
    #[cfg(feature = "sbench")]
    neighbors!("neighbors_m", 10_000_000);
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
    macro_rules! add_edge {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                b.iter(|| {
                    let mut v1 = graph.add_vertex(0);

                    for i in 1..=$x {
                        let v2 = graph.add_vertex(i);
                        graph.add_edge(&v1, &v2);
                        v1 = v2.clone();
                    }
                })
            });
        };
    }
    add_edge!("add_edge_10", 10);
    add_edge!("add_edge_100", 100);
    add_edge!("add_edge_500", 500);
    add_edge!("add_edge_1000", 1000);
    #[cfg(feature = "sbench")]
    add_edge!("add_edge_m", 10_000_000);

    macro_rules! add_edge_cycle_check {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                b.iter(|| {
                    let mut v1 = graph.add_vertex(0);

                    for i in 1..=$x {
                        let v2 = graph.add_vertex(i);
                        graph.add_edge_check_cycle(&v1, &v2);
                        v1 = v2.clone();
                    }
                })
            });
        };
    }
    add_edge_cycle_check!("add_edge_cycle_check_10", 10);
    add_edge_cycle_check!("add_edge_cycle_check_100", 100);
    add_edge_cycle_check!("add_edge_cycle_check_500", 500);
    add_edge_cycle_check!("add_edge_cycle_check_1000", 1000);
    #[cfg(feature = "sbench")]
    add_edge_cycle_check!("add_edge_cycle_check_m", 10_000_000);

    macro_rules! add_vertex {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                b.iter(|| {
                    for i in 1..=$x {
                        graph.add_vertex(i);
                    }
                })
            });
        };
    }
    add_vertex!("add_vertex_10", 10);
    add_vertex!("add_vertex_100", 100);
    add_vertex!("add_vertex_500", 500);
    add_vertex!("add_vertex_1000", 1000);
    #[cfg(feature = "sbench")]
    add_vertex!("add_vertex_m", 10_000_000);

    macro_rules! capacity {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let graph: Graph<usize> = Graph::with_capacity($x);
                b.iter(|| {
                    let _k = graph.capacity();
                })
            });
        };
    }

    capacity!("capacity_10", 10);
    capacity!("capacity_100", 100);
    capacity!("capacity_500", 500);
    capacity!("capacity_1000", 1000);
    #[cfg(feature = "sbench")]
    capacity!("capacity_m", 10_000_000);

    macro_rules! edge_count {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
                    let v2 = graph.add_vertex(i);
                    graph.add_edge(&v1, &v2);
                    v1 = v2.clone();
                }
                b.iter(|| {
                    let _k = graph.edge_count();
                })
            });
        };
    }
    edge_count!("edge_count_10", 10);
    edge_count!("edge_count_100", 100);
    edge_count!("edge_count_500", 500);
    edge_count!("edge_count_1000", 1000);
    #[cfg(feature = "sbench")]
    edge_count!("edge_count_m", 10_000_000);

    macro_rules! fetch {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                for i in 1..$x {
                    graph.add_vertex(i);
                }
                let id = graph.add_vertex($x);
                b.iter(|| {
                    let _k = *graph.fetch(&id).unwrap();
                })
            });
        };
    }
    fetch!("fetch_10", 10);
    fetch!("fetch_100", 100);
    fetch!("fetch_500", 500);
    fetch!("fetch_1000", 1000);
    #[cfg(feature = "sbench")]
    fetch!("fetch_m", 10_000_000);

    macro_rules! fetch_mut {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                for i in 1..$x {
                    graph.add_vertex(i);
                }
                let id = graph.add_vertex($x);
                b.iter(|| {
                    let _v = graph.fetch_mut(&id).unwrap();
                })
            });
        };
    }
    fetch_mut!("fetch_mut_10", 10);
    fetch_mut!("fetch_mut_100", 100);
    fetch_mut!("fetch_mut_500", 500);
    fetch_mut!("fetch_mut_1000", 1000);
    #[cfg(feature = "sbench")]
    fetch_mut!("fetch_mut_m", 10_000_000);

    macro_rules! fold {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                for i in 1..=$x {
                    graph.add_vertex(i);
                }

                b.iter(|| {
                    let _result = graph.fold(0, |v, acc| v + acc);
                })
            });
        };
    }
    fold!("fold_10", 10);
    fold!("fold_100", 100);
    fold!("fold_500", 500);
    fold!("fold_1000", 1000);
    #[cfg(feature = "sbench")]
    fold!("fold_m", 10_000_000);

    macro_rules! has_edge {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                let v1 = graph.add_vertex(1);
                let v2 = graph.add_vertex(2);

                for i in 3..=$x {
                    graph.add_vertex(i);
                }
                let v3 = graph.add_vertex(3);

                graph.add_edge(&v1, &v2).unwrap();
                b.iter(|| {
                    let _k = graph.has_edge(&v1, &v2);
                    let _l = graph.has_edge(&v2, &v3);
                })
            });
        };
    }
    has_edge!("has_edge_10", 10);
    has_edge!("has_edge_100", 100);
    has_edge!("has_edge_500", 500);
    has_edge!("has_edge_1000", 1000);
    #[cfg(feature = "sbench")]
    has_edge!("has_edge_m", 10_000_000);

    macro_rules! is_cyclic {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                let v0 = graph.add_vertex(0);
                let mut v1 = graph.add_vertex(1);
                let mut v2 = graph.add_vertex(2);
                graph.add_edge(&v0, &v1);
                graph.add_edge(&v1, &v2);
                for i in 4..=$x {
                    v1 = v2.clone();
                    v2 = graph.add_vertex(i);
                    graph.add_edge(&v1, &v2);
                }
                graph.add_edge(&v2, &v0);
                b.iter(|| {
                    let _k = graph.is_cyclic();
                })
            });
        };
    }
    is_cyclic!("is_cyclic_10", 10);
    is_cyclic!("is_cyclic_100", 100);
    is_cyclic!("is_cyclic_500", 500);
    is_cyclic!("is_cyclic_1000", 1000);
    #[cfg(feature = "sbench")]
    is_cyclic!("is_cyclic_m", 10_000_000);

    macro_rules! remove {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                b.iter(|| {
                    for i in 1..=$x {
                        let v1 = graph.add_vertex(i);
                        graph.remove(&v1);
                    }
                })
            });
        };
    }
    remove!("remove_10", 10);
    remove!("remove_100", 100);
    remove!("remove_500", 500);
    remove!("remove_1000", 1000);
    #[cfg(feature = "sbench")]
    remove!("remove_m", 10_000_000);

    macro_rules! remove_edge {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                b.iter(|| {
                    let mut v1 = graph.add_vertex(0);

                    for i in 1..=$x {
                        let v2 = graph.add_vertex(i);
                        graph.add_edge(&v1, &v2);
                        v1 = v2.clone();
                        graph.remove_edge(&v1, &v2);
                    }
                })
            });
        };
    }

    remove_edge!("remove_edge_10", 10);
    remove_edge!("remove_edge_100", 100);
    remove_edge!("remove_edge_500", 500);
    remove_edge!("remove_edge_1000", 1000);
    #[cfg(feature = "sbench")]
    remove_edge!("remove_edge_m", 10_000_000);

    macro_rules! reserve {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::with_capacity($x);

                for i in 1..=$x {
                    graph.add_vertex(i);
                }

                b.iter(|| {
                    graph.reserve($x);
                })
            });
        };
    }

    reserve!("reserve_10", 10);
    reserve!("reserve_100", 100);
    reserve!("reserve_500", 500);
    reserve!("reserve_1000", 1000);
    #[cfg(feature = "sbench")]
    reserve!("reserve_m", 10_000_000);

    macro_rules! retain {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                for i in 1..=$x {
                    graph.add_vertex(i);
                }
                b.iter(|| {
                    graph.retain(|v| *v != 2);
                })
            });
        };
    }

    retain!("retain_10", 10);
    retain!("retain_100", 100);
    retain!("retain_500", 500);
    retain!("retain_1000", 1000);
    #[cfg(feature = "sbench")]
    retain!("retain_m", 10_000_000);

    macro_rules! roots_count {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();
                let mut v1 = graph.add_vertex(0);

                for i in 1..=$x {
                    let v2 = graph.add_vertex(i);
                    graph.add_edge(&v1, &v2);
                    v1 = v2.clone();
                }
                b.iter(|| {
                    let _k = graph.roots_count();
                })
            });
        };
    }
    roots_count!("roots_count_10", 10);
    roots_count!("roots_count_100", 100);
    roots_count!("roots_count_500", 500);
    roots_count!("roots_count_1000", 1000);
    #[cfg(feature = "sbench")]
    roots_count!("roots_count_m", 10_000_000);

    macro_rules! shrink_to_fit {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::with_capacity($x);

                b.iter(|| {
                    graph.shrink_to_fit();
                })
            });
        };
    }
    shrink_to_fit!("shrink_to_fit_10", 10);
    shrink_to_fit!("shrink_to_fit_100", 100);
    shrink_to_fit!("shrink_to_fit_500", 500);
    shrink_to_fit!("shrink_to_fit_1000", 1000);
    #[cfg(feature = "sbench")]
    shrink_to_fit!("shrink_to_fit_m", 10_000_000);

    macro_rules! vertex_count {
        ($str: tt ,$x: expr) => {
            c.bench_function($str, |b| {
                let mut graph: Graph<usize> = Graph::new();

                for i in 1..=$x {
                    graph.add_vertex(i);
                }
                b.iter(|| {
                    let _k = graph.vertex_count();
                })
            });
        };
    }
    vertex_count!("vertex_count_10", 10);
    vertex_count!("vertex_count_100", 100);
    vertex_count!("vertex_count_500", 500);
    vertex_count!("vertex_count_1000", 1000);
    #[cfg(feature = "sbench")]
    vertex_count!("vertex_count_m", 10_000_000);
}

criterion_group!(
    benches,
    bench_create,
    bench_iterators,
    bench_neighbor_functions,
    bench_others
);

criterion_main!(benches);
