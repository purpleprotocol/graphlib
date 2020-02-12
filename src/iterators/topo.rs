// Copyright 2019 Gary Pennington

use crate::graph::Graph;
use crate::vertex_id::VertexId;

use hashbrown::HashMap;

#[cfg(feature = "no_std")]
use alloc::vec::Vec;

#[cfg(feature = "no_std")]
use core::fmt::Debug;

#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

const PANIC_MSG: &str = "graph contains cycle(s)";

#[derive(Debug)]
/// Topological Iterator
pub struct Topo<'a, T> {
    /// The Graph being iterated.
    iterable: &'a Graph<T>,
    /// Processed vertices
    vertices: Vec<&'a VertexId>,
    /// Working set of vertices
    roots: Vec<&'a VertexId>,
    /// Working set of vertex edges
    vertex_edges: HashMap<&'a VertexId, usize>,
}

impl<'a, T> Topo<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Topo<'_, T> {
        let mut roots = vec![];
        for node in graph.roots() {
            roots.push(node);
        }

        Topo {
            iterable: graph,
            vertices: vec![],
            roots,
            vertex_edges: HashMap::new(),
        }
    }

    /// Processes the next vertex.
    ///
    /// Will return None if:
    ///
    /// * No vertices are left.
    fn process_vertex(&mut self, check_cyclic: bool) -> Option<&'a VertexId> {
        match self.roots.pop() {
            Some(node) => {
                self.vertices.push(node);
                for out in self.iterable.out_neighbors(node) {
                    let count = match self.vertex_edges.get_mut(out) {
                        Some(count) => count,
                        None => {
                            self.vertex_edges
                                .insert(out, self.iterable.in_neighbors_count(out));
                            self.vertex_edges.get_mut(out).unwrap()
                        }
                    };
                    if *count == 1 {
                        self.roots.push(out);
                    } else {
                        *count -= 1;
                    }
                }
                Some(node)
            }
            None => {
                if check_cyclic && self.vertices.len() != self.iterable.vertex_count() {
                    panic!(PANIC_MSG);
                }
                None
            }
        }
    }

    ///
    /// Returns true if the iterated graph has a cycle.
    ///
    /// # Warning
    ///
    /// It is a logic error to use this iterator after calling this function.
    pub fn is_cyclic(&mut self) -> bool {
        //Search until an answer is found.
        while self.process_vertex(false).is_some() {}

        self.vertices.len() != self.iterable.vertex_count()
    }
}

impl<'a, T> Iterator for Topo<'a, T> {
    type Item = &'a VertexId;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.iterable.vertex_count() - self.vertices.len();

        (0, Some(remaining))
    }
    fn next(&mut self) -> Option<Self::Item> {
        (0..self.size_hint().1.unwrap())
            .filter_map(move |_| self.process_vertex(true))
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_not_cyclic() {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);
        let v4 = graph.add_vertex(4);
        let v5 = graph.add_vertex(5);
        let v6 = graph.add_vertex(6);
        let v7 = graph.add_vertex(7);

        graph.add_edge(&v1, &v4).unwrap();
        graph.add_edge(&v2, &v4).unwrap();
        graph.add_edge(&v2, &v5).unwrap();
        graph.add_edge(&v3, &v5).unwrap();
        graph.add_edge(&v4, &v6).unwrap();
        graph.add_edge(&v4, &v7).unwrap();
        graph.add_edge(&v5, &v6).unwrap();
        graph.add_edge(&v6, &v7).unwrap();

        let mut topo = graph.topo();

        assert!(!topo.is_cyclic());
    }

    #[test]
    fn is_cyclic() {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);
        let v4 = graph.add_vertex(4);
        let v5 = graph.add_vertex(5);
        let v6 = graph.add_vertex(6);
        let v7 = graph.add_vertex(7);

        graph.add_edge(&v1, &v4).unwrap();
        graph.add_edge(&v2, &v4).unwrap();
        graph.add_edge(&v2, &v5).unwrap();
        graph.add_edge(&v3, &v5).unwrap();
        graph.add_edge(&v4, &v6).unwrap();
        graph.add_edge(&v4, &v7).unwrap();
        graph.add_edge(&v5, &v6).unwrap();
        graph.add_edge(&v6, &v7).unwrap();
        graph.add_edge(&v7, &v2).unwrap();

        let mut topo = graph.topo();

        assert!(topo.is_cyclic());
    }

    #[test]
    #[should_panic(expected = "graph contains cycle(s)")]
    fn is_cyclic_and_panic() {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);
        let v4 = graph.add_vertex(4);
        let v5 = graph.add_vertex(5);
        let v6 = graph.add_vertex(6);
        let v7 = graph.add_vertex(7);

        graph.add_edge(&v1, &v4).unwrap();
        graph.add_edge(&v2, &v4).unwrap();
        graph.add_edge(&v2, &v5).unwrap();
        graph.add_edge(&v3, &v5).unwrap();
        graph.add_edge(&v4, &v6).unwrap();
        graph.add_edge(&v4, &v7).unwrap();
        graph.add_edge(&v5, &v6).unwrap();
        graph.add_edge(&v6, &v7).unwrap();
        graph.add_edge(&v7, &v2).unwrap();

        let mut topo = graph.topo();

        topo.next();
        topo.next();
        topo.next();
        topo.next();
        topo.next();
        topo.next();
        topo.next();
    }

    #[test]
    fn was_cyclic() {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v2, &v3).unwrap();
        graph.add_edge(&v3, &v1).unwrap();

        graph.remove_edge(&v3, &v1);

        let mut topo = graph.topo();

        assert!(!topo.is_cyclic());
    }
}
