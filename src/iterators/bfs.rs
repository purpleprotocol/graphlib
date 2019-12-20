// Copyright 2019 Octavian Oncescu

use crate::graph::Graph;
use crate::vertex_id::VertexId;

use hashbrown::HashSet;
#[cfg(not(feature = "no_std"))]
use std::collections::VecDeque;

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::collections::vec_deque::VecDeque;

#[cfg(feature = "no_std")]
use alloc::vec::Vec;

#[cfg(feature = "no_std")]
use core::fmt::Debug;

#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

#[derive(Debug)]
/// Breadth-First Iterator
pub struct Bfs<'a, T> {
    queue: VecDeque<VertexId>,
    current_ptr: Option<VertexId>,
    visited_set: HashSet<VertexId>,
    roots_stack: Vec<VertexId>,
    iterable: &'a Graph<T>,
}

impl<'a, T> Bfs<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Bfs<'_, T> {
        let mut roots_stack = Vec::with_capacity(graph.roots_count());

        for v in graph.roots() {
            roots_stack.push(v.clone());
        }

        let current_ptr = roots_stack.pop();

        Bfs {
            queue: VecDeque::with_capacity(graph.vertex_count()),
            current_ptr,
            visited_set: HashSet::with_capacity(graph.vertex_count()),
            roots_stack,
            iterable: graph,
        }
    }
}

impl<'a, T> Iterator for Bfs<'a, T> {
    type Item = &'a VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut next_ptr = None;

            if let Some(current_ptr) = &self.current_ptr {
                // Yield current pointed value if
                // it isn't in the visited stack.
                if !self.visited_set.contains(current_ptr) {
                    self.visited_set.insert(current_ptr.clone());
                    return self.iterable.fetch_id_ref(current_ptr.as_ref());
                }

                // Iterate through current neighbors
                // and check their visited status.
                for n in self.iterable.out_neighbors(current_ptr.as_ref()) {
                    if !self.visited_set.contains(n) {
                        self.visited_set.insert(n.clone());
                        self.queue.push_back(n.clone());

                        return self.iterable.fetch_id_ref(n);
                    }
                }

                // Move to next root if possible and yield it.
                if self.queue.is_empty() {
                    if let Some(next_root) = self.roots_stack.pop() {
                        next_ptr = Some(next_root);
                    } else {
                        // Break execution if there are no more roots
                        return None;
                    }
                } else {
                    // Pop item from queue and set it
                    // as the current pointer.
                    next_ptr = self.queue.pop_front();
                }
            } else {
                return None;
            }

            if next_ptr.is_some() {
                self.current_ptr = next_ptr;
            }
        }
    }
}
