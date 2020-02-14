// Copyright 2019 Chakrapani Gautam

use crate::graph::{Graph, GraphErr};
use crate::iterators::owning_iterator::OwningIterator;
use crate::iterators::vertices::VertexIter;
use crate::vertex_id::VertexId;

use hashbrown::HashMap;
use hashbrown::HashSet;

#[cfg(not(feature = "no_std"))]
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
    f32,
    fmt::Debug,
    iter,
};

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::boxed::Box;
#[cfg(feature = "no_std")]
use alloc::collections::{binary_heap::BinaryHeap, vec_deque::VecDeque};

#[cfg(feature = "no_std")]
use core::{cmp::Ordering, f32, fmt::Debug, iter};

#[derive(PartialEq, Debug)]
struct VertexMeta {
    id: VertexId,
    distance: f32,
}

impl Eq for VertexMeta {}

impl PartialOrd for VertexMeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for VertexMeta {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Debug)]
/// Dijkstra Single-source Shortest Path Iterator
pub struct Dijkstra<'a, T> {
    source: &'a VertexId,
    iterable: &'a Graph<T>,
    iterator: VecDeque<VertexId>,
    distances: HashMap<VertexId, f32>,
    previous: HashMap<VertexId, Option<VertexId>>,
}

impl<'a, T> Dijkstra<'a, T> {
    pub fn new(graph: &'a Graph<T>, src: &'a VertexId) -> Result<Dijkstra<'a, T>, GraphErr> {
        if graph.fetch(src).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        for edge in graph.edges() {
            if let Some(w) = graph.weight(edge.1, edge.0) {
                if w < 0.0 {
                    return Err(GraphErr::InvalidWeight);
                }
            }
        }

        let mut instance = Dijkstra {
            source: src,
            iterable: graph,
            iterator: VecDeque::with_capacity(graph.vertex_count()),
            distances: HashMap::with_capacity(graph.vertex_count()),
            previous: HashMap::with_capacity(graph.vertex_count()),
        };

        instance.calc_distances();

        Ok(instance)
    }

    pub fn set_source(&mut self, vert: &'a VertexId) -> Result<(), GraphErr> {
        if self.iterable.fetch(vert).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        self.source = vert;
        self.distances.clear();
        self.previous.clear();
        self.calc_distances();

        Ok(())
    }

    pub fn get_path_to(mut self, vert: &'a VertexId) -> Result<VertexIter, GraphErr> {
        if self.iterable.fetch(vert).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        if self.previous.contains_key(vert) {
            let mut cur_vert = Some(vert);
            self.iterator.clear();

            while cur_vert.is_some() {
                self.iterator.push_front(*cur_vert.unwrap());

                match self.previous.get(cur_vert.unwrap()) {
                    Some(v) => cur_vert = v.as_ref(),
                    None => cur_vert = None,
                }
            }

            return Ok(VertexIter(Box::new(OwningIterator::new(self.iterator))));
        }

        Ok(VertexIter(Box::new(iter::empty())))
    }

    pub fn get_distance(&mut self, vert: &'a VertexId) -> Result<f32, GraphErr> {
        if self.iterable.fetch(vert).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        if self.distances.contains_key(vert) {
            return Ok(*self.distances.get(vert).unwrap());
        }

        Ok(f32::MAX)
    }

    fn calc_distances(&mut self) {
        let mut visited: HashSet<VertexId> = HashSet::with_capacity(self.iterable.vertex_count());
        let mut vertex_pq: BinaryHeap<VertexMeta> =
            BinaryHeap::with_capacity(self.iterable.vertex_count());

        for vert in self.iterable.vertices() {
            self.distances.insert(*vert, f32::MAX);
        }

        vertex_pq.push(VertexMeta {
            id: *self.source,
            distance: 0.0,
        });

        self.distances.insert(*self.source, 0.0);
        self.previous.insert(*self.source, None);

        while let Some(vert_meta) = vertex_pq.pop() {
            if !visited.insert(vert_meta.id) {
                continue;
            }

            for neighbor in self.iterable.out_neighbors(&vert_meta.id) {
                if !visited.contains(&neighbor) {
                    let mut alt_dist = *self.distances.get(&vert_meta.id).unwrap();

                    if let Some(w) = self.iterable.weight(&vert_meta.id, &neighbor) {
                        alt_dist += w;
                    }

                    if alt_dist < *self.distances.get(&neighbor).unwrap() {
                        self.distances.insert(*neighbor, alt_dist);
                        self.previous.insert(*neighbor, Some(vert_meta.id));

                        vertex_pq.push(VertexMeta {
                            id: *neighbor,
                            distance: alt_dist,
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_empty_graph() {
        let random_vertex = VertexId::random();

        let graph: Graph<usize> = Graph::new();
        let result = Dijkstra::new(&graph, &random_vertex);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_invalid_source() {
        let random_vertex = VertexId::random();

        let mut graph: Graph<usize> = Graph::new();
        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        graph.add_edge_with_weight(&v1, &v2, 0.0);

        let result = Dijkstra::new(&graph, &random_vertex);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_negative_weight_edge() {
        let mut graph: Graph<usize> = Graph::new();
        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);

        graph.add_edge_with_weight(&v1, &v2, -0.1);
        graph.add_edge_with_weight(&v2, &v1, 0.1);

        let result = Dijkstra::new(&graph, &v1);

        assert!(result.is_err());
    }

    #[test]
    fn test_set_source_with_invalid_vertex() {
        let random_vertex = VertexId::random();

        let mut graph: Graph<usize> = Graph::new();
        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        graph.add_edge_with_weight(&v1, &v2, 0.0);

        let mut iterator = Dijkstra::new(&graph, &v1).unwrap();
        let result = iterator.set_source(&random_vertex);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_path_to_with_invalid_vertex() {
        let random_vertex = VertexId::random();

        let mut graph: Graph<usize> = Graph::new();
        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        graph.add_edge_with_weight(&v1, &v2, 0.0);

        let mut iterator = Dijkstra::new(&graph, &v1).unwrap();
        let result = iterator.get_path_to(&random_vertex);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_distance_with_invalid_vertex() {
        let random_vertex = VertexId::random();

        let mut graph: Graph<usize> = Graph::new();
        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        graph.add_edge_with_weight(&v1, &v2, 0.0);

        let mut iterator = Dijkstra::new(&graph, &v1).unwrap();
        let result = iterator.get_distance(&random_vertex);

        assert!(result.is_err());
    }

    #[test]
    fn test_on_connected_graphs() {
        let infinity = f32::MAX;

        let mut graph: Graph<usize> = Graph::new();

        let v_a = graph.add_vertex(1);
        let v_b = graph.add_vertex(2);
        let v_c = graph.add_vertex(3);
        let v_d = graph.add_vertex(4);
        let v_e = graph.add_vertex(5);
        let v_f = graph.add_vertex(6);

        graph.add_edge_with_weight(&v_a, &v_b, 0.1).unwrap();
        graph.add_edge_with_weight(&v_b, &v_d, 0.2).unwrap();
        graph.add_edge_with_weight(&v_c, &v_b, 0.5).unwrap();
        graph.add_edge_with_weight(&v_c, &v_d, 0.1).unwrap();
        graph.add_edge_with_weight(&v_c, &v_e, 0.5).unwrap();
        graph.add_edge_with_weight(&v_d, &v_f, 0.8).unwrap();

        {
            let mut iterator = Dijkstra::new(&graph, &v_a).unwrap();

            assert_eq!(iterator.get_distance(&v_a).unwrap(), 0.0);
            assert_eq!(iterator.get_distance(&v_b).unwrap(), 0.1);
            assert_eq!(iterator.get_distance(&v_c).unwrap(), infinity);
            assert_eq!(iterator.get_distance(&v_d).unwrap(), 0.3);
            assert_eq!(iterator.get_distance(&v_e).unwrap(), infinity);
            assert_eq!(iterator.get_distance(&v_f).unwrap(), 1.1);
        }

        graph.add_edge_with_weight(&v_b, &v_a, 0.1).unwrap();
        graph.add_edge_with_weight(&v_d, &v_b, 0.2).unwrap();
        graph.add_edge_with_weight(&v_b, &v_c, 0.5).unwrap();
        graph.add_edge_with_weight(&v_d, &v_c, 0.1).unwrap();
        graph.add_edge_with_weight(&v_e, &v_c, 0.5).unwrap();
        graph.add_edge_with_weight(&v_f, &v_d, 0.8).unwrap();

        let mut iterator = Dijkstra::new(&graph, &v_a).unwrap();

        assert_eq!(iterator.get_distance(&v_a).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_b).unwrap(), 0.1);
        assert_eq!(iterator.get_distance(&v_c).unwrap(), 0.4);
        assert_eq!(iterator.get_distance(&v_d).unwrap(), 0.3);
        assert_eq!(iterator.get_distance(&v_e).unwrap(), 0.9);
        assert_eq!(iterator.get_distance(&v_f).unwrap(), 1.1);

        iterator.set_source(&v_c);

        assert_eq!(iterator.get_distance(&v_a).unwrap(), 0.4);
        assert_eq!(iterator.get_distance(&v_b).unwrap(), 0.3);
        assert_eq!(iterator.get_distance(&v_c).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_d).unwrap(), 0.1);
        assert_eq!(iterator.get_distance(&v_e).unwrap(), 0.5);
        assert_eq!(iterator.get_distance(&v_f).unwrap(), 0.900_000_04);
        // Ugh! I wish there was something like `assert_approx_eq!()`. Too lazy to write on my own.

        assert_eq!(iterator.clone().get_path_to(&v_a).unwrap().count(), 4);
        assert_eq!(iterator.clone().get_path_to(&v_b).unwrap().count(), 3);
        assert_eq!(iterator.clone().get_path_to(&v_c).unwrap().count(), 1);
        assert_eq!(iterator.clone().get_path_to(&v_d).unwrap().count(), 2);
        assert_eq!(iterator.clone().get_path_to(&v_e).unwrap().count(), 2);
        assert_eq!(iterator.clone().get_path_to(&v_f).unwrap().count(), 3);

        /*
        // To run these tests, uncomment and use `-- --nocapture` flag in `cargo test`

        for vert in graph.vertices() {
            println!("Current vertex: {:?}", graph.fetch(vert));

            for arg in iterator.get_path_to(vert).unwrap() {
                print!(" {:?}", graph.fetch(arg));
            }
            println!();
        }
        */
    }

    #[test]
    fn test_on_unweighted_graph() {
        let infinity = f32::MAX;

        let mut graph: Graph<usize> = Graph::new();

        let v_a = graph.add_vertex(1);
        let v_b = graph.add_vertex(2);
        let v_c = graph.add_vertex(3);
        let v_d = graph.add_vertex(4);
        let v_e = graph.add_vertex(5);
        let v_f = graph.add_vertex(6);

        graph.add_edge(&v_a, &v_b).unwrap();
        graph.add_edge(&v_b, &v_d).unwrap();
        graph.add_edge(&v_c, &v_b).unwrap();
        graph.add_edge(&v_c, &v_d).unwrap();
        graph.add_edge(&v_c, &v_e).unwrap();
        graph.add_edge(&v_d, &v_f).unwrap();

        let mut iterator = Dijkstra::new(&graph, &v_a).unwrap();

        assert_eq!(iterator.get_distance(&v_a).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_b).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_c).unwrap(), infinity);
        assert_eq!(iterator.get_distance(&v_d).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_e).unwrap(), infinity);
        assert_eq!(iterator.get_distance(&v_f).unwrap(), 0.0);

        assert_eq!(iterator.clone().get_path_to(&v_a).unwrap().count(), 1);
        assert_eq!(iterator.clone().get_path_to(&v_b).unwrap().count(), 2);
        assert_eq!(iterator.clone().get_path_to(&v_c).unwrap().count(), 0);
        assert_eq!(iterator.clone().get_path_to(&v_d).unwrap().count(), 3);
        assert_eq!(iterator.clone().get_path_to(&v_e).unwrap().count(), 0);
        assert_eq!(iterator.clone().get_path_to(&v_f).unwrap().count(), 4);

        iterator.set_source(&v_c).unwrap();

        assert_eq!(iterator.get_distance(&v_a).unwrap(), infinity);
        assert_eq!(iterator.get_distance(&v_b).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_c).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_d).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_e).unwrap(), 0.0);
        assert_eq!(iterator.get_distance(&v_f).unwrap(), 0.0);

        assert_eq!(iterator.clone().get_path_to(&v_a).unwrap().count(), 0);
        assert_eq!(iterator.clone().get_path_to(&v_b).unwrap().count(), 2);
        assert_eq!(iterator.clone().get_path_to(&v_c).unwrap().count(), 1);
        assert_eq!(iterator.clone().get_path_to(&v_d).unwrap().count(), 2);
        assert_eq!(iterator.clone().get_path_to(&v_e).unwrap().count(), 2);
        assert_eq!(iterator.clone().get_path_to(&v_f).unwrap().count(), 3);

        /*
        // To run these tests, uncomment and use `-- --nocapture` flag in `cargo test`

        for vert in graph.vertices() {
            println!("Current vertex: {:?}", graph.fetch(vert));

            for arg in iterator.get_path_to(vert).unwrap() {
                print!(" {:?}", graph.fetch(arg));
            }
            println!();
        }
        */
    }
}
