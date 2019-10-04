// Copyright 2019 Octavian Oncescu

use crate::graph::Graph;
use crate::iterators::VertexIter;
use crate::vertex_id::VertexId;

#[cfg(feature = "no_std")]
use core::iter::{Chain, Cloned, Peekable};
use hashbrown::HashSet;
#[cfg(not(feature = "no_std"))]
use std::iter::{Chain, Cloned, Peekable};

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::vec::Vec;

#[cfg(feature = "no_std")]
use core::fmt::Debug;

#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

#[derive(Debug)]
/// Depth-First Iterator
pub struct Dfs<'a, T> 
    where T: Clone + Debug
{
    /// All the vertices to be checked with the roots coming first.
    unchecked: Peekable<Cloned<Chain<VertexIter<'a>, VertexIter<'a>>>>,
    /// All black vertices.
    black: HashSet<VertexId>,
    /// All grey vertices.
    grey: HashSet<VertexId>,
    /// All vertices pending processing.
    pending_stack: Vec<VertexId>,
    /// The Graph being iterated.
    iterable: &'a Graph<T>,
    /// A cached answer to the question: does this Graph contain cycles.
    cached_cyclic: bool,
}

impl<'a, T> Dfs<'a, T> 
    where T: Clone + Debug
{
    pub fn new(graph: &'a Graph<T>) -> Dfs<'_, T> {
        let unchecked = graph.roots().chain(graph.vertices()).cloned().peekable();

        Dfs {
            unchecked,
            iterable: graph,
            cached_cyclic: false,
            grey: HashSet::new(),
            black: HashSet::new(),
            pending_stack: Vec::new(),
        }
    }

    /// Returns true if the iterated graph has a cycle.
    ///
    /// # Warning
    ///
    /// It is a logic error to use this iterator after calling this function.
    pub fn is_cyclic(&mut self) -> bool {
        //Check for a cached answer.
        if self.cached_cyclic {
            return self.cached_cyclic;
        }

        //Search until an answer is found.
        while self.process_vertex().is_some() {}

        self.cached_cyclic
    }

    /// Processes the next vertex.
    ///
    /// Will return None if:
    ///
    /// * No vertices are left.
    /// * The next vertex forms a cycle.
    fn process_vertex(&mut self) -> Option<&'a VertexId> {
        //We have traversed this partition of the graph, move on.
        if self.pending_stack.is_empty() {
            //Mark all the grey vertices black.
            self.black.extend(self.grey.drain());

            //Spliting the borrows for the borrow checker.
            let unchecked = &mut self.unchecked;
            let black = &self.black;

            //Search for an unprocessed vertex.
            let next = unchecked.find(move |v| !black.contains(v));

            //We found a new vertex.
            if let Some(v) = next {
                self.pending_stack.push(v);
            }
        }

        //Get the next pending vertex.
        self.pending_stack
            .pop()
            .iter()
            //Filter cycles.
            .filter_map(|v| {
                //If this vertex forms a cycle do not return it.
                if !self.grey.insert(*v) {
                    self.cached_cyclic = true;

                    return None;
                }

                //Add all of its neighbours to be processed.
                for v in self.iterable.out_neighbors(v) {
                    //This neighbour forms a cycle don't process it.
                    if self.grey.contains(v) {
                        self.cached_cyclic = true
                    } else {
                        self.pending_stack.push(*v)
                    }
                }

                self.iterable.fetch_id_ref(v)
            })
            .next()
    }
}

impl<'a, T> Iterator for Dfs<'a, T>
    where T: Clone + Debug
{
    type Item = &'a VertexId;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.iterable.vertex_count() - self.black.len();

        (0, Some(remaining))
    }
    fn next(&mut self) -> Option<Self::Item> {
        (0..self.size_hint().1.unwrap())
            .filter_map(move |_| self.process_vertex())
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_cyclic() {
        /*
        A previous version of the function would fail if the iterator had passed through the last cycle.

        The current version written 2019-03-23 caches if any cycles have been found as it
        iterates to resolve this issue.
        */

        for _ in 0..100 {
            let mut graph = Graph::new();

            let v = graph.add_vertex(0);

            assert!(graph.add_edge(&v, &v).is_ok(), "Failed to create cycle");

            for _ in 0..100 {
                graph.add_vertex(0);
            }

            let mut dfs = graph.dfs();

            for _ in 0..99 {
                dfs.next();
            }

            assert!(dfs.is_cyclic());
        }
    }
    #[test]
    fn not_cyclic() {
        let mut graph = Graph::new();

        let v1 = graph.add_vertex(());
        let v2 = graph.add_vertex(());
        let v3 = graph.add_vertex(());

        graph.add_edge(&v1, &v2);
        graph.add_edge(&v3, &v2);

        graph.add_vertex(());

        assert_eq!(graph.is_cyclic(), false);
    }

    #[test]
    fn not_cyclic_edge_to_successor() {
        let mut graph = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v2, &v3).unwrap();
        graph.add_edge(&v1, &v3).unwrap();

        assert_eq!(graph.is_cyclic(), false);
    }

    #[test]
    fn not_cyclic_edge_split_merge() {
        let mut graph = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);
        let v4 = graph.add_vertex(4);
        let v5 = graph.add_vertex(5);
        let v6 = graph.add_vertex(6);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v2, &v3).unwrap();
        graph.add_edge(&v3, &v4).unwrap();
        graph.add_edge(&v3, &v5).unwrap();
        graph.add_edge(&v4, &v6).unwrap();
        graph.add_edge(&v5, &v6).unwrap();

        assert_eq!(graph.is_cyclic(), false);
    }

    #[test]
    fn not_cyclic_split_merge_continue() {
        // TODO: rename that test

        let mut graph = Graph::new();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);
        let v3 = graph.add_vertex(3);
        let v4 = graph.add_vertex(4);
        let v5 = graph.add_vertex(5);
        let v6 = graph.add_vertex(6);
        let v7 = graph.add_vertex(7);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v2, &v3).unwrap();
        graph.add_edge(&v3, &v4).unwrap();
        graph.add_edge(&v3, &v5).unwrap();
        graph.add_edge(&v4, &v6).unwrap();
        graph.add_edge(&v5, &v6).unwrap();
        graph.add_edge(&v1, &v6).unwrap();
        graph.add_edge(&v6, &v7).unwrap();

        assert_eq!(graph.is_cyclic(), false);
    }

    #[test]
    fn cycle_self_edge() {
        let mut graph = Graph::new();

        let v1 = graph.add_vertex(1);

        graph.add_edge(&v1, &v1).unwrap();

        assert!(graph.is_cyclic());
    }
}
