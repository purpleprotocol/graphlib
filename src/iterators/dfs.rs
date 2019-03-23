// Copyright 2019 Octavian Oncescu

use crate::graph::Graph;
use crate::vertex_id::VertexId;
use crate::iterators::VertexIter;

use hashbrown::HashSet;
use std::iter::{Cloned, Chain, Peekable,};

#[derive(Debug)]
/// Depth-First Iterator
pub struct Dfs<'a, T> {
    /// All the vertices to be checked with the roots coming first.
    unchecked: Peekable<Cloned<Chain<VertexIter<'a>, VertexIter<'a>>>>,
    /// All previously visited vertices.
    visited: HashSet<VertexId>,
    /// All vertices pending processing.
    pending_stack: Vec<VertexId>,
    /// The Graph being iterated.
    iterable: &'a Graph<T>,
    /// A cached answer to the question: does this Graph contain cycles.
    cached_cyclic: bool,
}

impl<'a, T> Dfs<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Dfs<'_, T> {
        let unchecked = graph.roots()
            .chain(graph.vertices())
            .cloned().peekable();

        Dfs {
            unchecked,
            iterable: graph,
            cached_cyclic: false,
            visited: HashSet::new(),
            pending_stack: Vec::new(),
        }
    }

    /// Returns true if the iterated graph has a cycle.
    /// 
    /// # Warning
    /// 
    /// It is a logic error to use this iterator after calling this function.
    pub fn is_cyclic(&mut self,) -> bool {
        //Check for a cached answer.
        if self.cached_cyclic { return self.cached_cyclic }

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
    fn process_vertex(&mut self,) -> Option<&'a VertexId> {
        //We have traversed this partition of the graph, move on.
        if self.pending_stack.is_empty() {
            //Spliting the borrows for the borrow checker.
            let unchecked = &mut self.unchecked;
            let visited = &self.visited;

            //Search for an unprocessed vertex.
            let next = unchecked.find(move |v,| !visited.contains(v));
            
            //We found a new vertex.
            if let Some(v) = next {
                self.pending_stack.push(v);
            }
        }

        //Get the next pending vertex.
        self.pending_stack.pop().iter()
        //Filter cycles.
        .filter_map(|v,| {
            //If this vertex forms a cycle do not return it.
            if !self.visited.insert(*v) {
                self.cached_cyclic = true;

                return None
            }

            //Add all of its neighbours to be processed.
            for v in self.iterable.out_neighbors(v) {
                //This neighbour forms a cycle don't process it.
                if self.visited.contains(v) { self.cached_cyclic = true }
                else { self.pending_stack.push(*v) }
            }

            self.iterable.fetch_id_ref(v)
        }).next()
    }
}

impl<'a, T> Iterator for Dfs<'a, T> {
    type Item = &'a VertexId;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.iterable.vertex_count() - self.visited.len();

        (remaining, Some(remaining))
    }
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (0..self.size_hint().0).filter_map(move |_,| self.process_vertex()).next()
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

            for _ in 0..100 { graph.add_vertex(0); }

            let mut dfs = graph.dfs();

            for _ in 0..99 { dfs.next(); }

            assert!(dfs.is_cyclic());
        }
    }
}
