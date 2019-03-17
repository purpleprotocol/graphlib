// Copyright 2019 Octavian Oncescu

use crate::graph::Graph;
use crate::vertex_id::VertexId;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct Bfs<'a, T> {
    queue: VecDeque<Arc<VertexId>>,
    current_ptr: Option<Arc<VertexId>>,
    visited_stack: HashSet<Arc<VertexId>>, // instead of a Vec, let's use a HashSet. (Haven't changed the name of variable yet. Maybe try "visited_set" ?!)
    roots_stack: Vec<Arc<VertexId>>,
    iterable: &'a Graph<T>,
}

impl<'a, T> Bfs<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Bfs<'_, T> {
        let mut roots_stack = Vec::with_capacity(graph.roots_count());

        for v in graph.roots() {
            roots_stack.push(Arc::from(*v));
        }

        let current_ptr = roots_stack.pop();

        Bfs {
            queue: VecDeque::with_capacity(graph.vertex_count()),
            current_ptr,
            visited_stack: HashSet::new(),  // Initially the set is empty. As soon as any node is visited, insert it in here.
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
                if !self.visited_stack.contains(&current_ptr) { // if set doesn't have a node
                    self.visited_stack.insert(current_ptr.clone()); // then insert it
                    return self.iterable.fetch_id_ref(current_ptr.as_ref());
                }

                // Iterate through current neighbors
                // and check their visited status.
                for n in self.iterable.out_neighbors(current_ptr.as_ref()) {
                    if !self.visited_stack.contains(n) {  // assuming &(*n) is same as 'n'. (I'm still a noob.)
                        self.visited_stack.insert(Arc::from(*n));
                        self.queue.push_back(Arc::from(*n));

                        return self.iterable.fetch_id_ref(n);
                    }
                }

                // Move to next root if possible and yield it.
                if self.queue.is_empty() {
                    if let Some(next_root) = self.roots_stack.pop() {
                        next_ptr = Some(next_root.clone());
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
