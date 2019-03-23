// Copyright 2019 Octavian Oncescu

use crate::graph::Graph;
use crate::vertex_id::VertexId;

use hashbrown::{HashMap, HashSet,};
use std::sync::Arc;

#[derive(Debug)]
/// Depth-First Iterator
pub struct Dfs<'a, T> {
    recursion_stack: Vec<Arc<VertexId>>,
    color_map: HashMap<Arc<VertexId>, Color>,
    roots_stack: Vec<Arc<VertexId>>,
    iterable: &'a Graph<T>,
    /// A cached answer to the question: does this Graph contain cycles.
    cached_cyclic: Option<bool>,
}

#[derive(Debug)]
enum Color {
    White,
    Grey,
    Black,
}

impl<'a, T> Dfs<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Dfs<'_, T> {
        let mut roots_stack = Vec::with_capacity(graph.roots_count());
        let color_map: HashMap<Arc<VertexId>, Color> = graph
            .vertices()
            .map(|v| (Arc::from(*v), Color::White))
            .collect();

        if graph.roots_count() == 0 && graph.vertex_count() != 0 {
            // Pick random vertex as first root
            for (random_vertex, _) in color_map.iter() {
                roots_stack.push(random_vertex.clone());
                break;
            }
        } else {
            for v in graph.roots() {
                roots_stack.push(Arc::from(*v));
            }
        }

        Dfs {
            color_map,
            recursion_stack: Vec::with_capacity(graph.vertex_count()),
            roots_stack,
            iterable: graph,
            cached_cyclic: None,
        }
    }

    /// Returns true if the iterated graph has a cycle.
    pub fn is_cyclic(&mut self,) -> bool {
        //Check for a cached answer.
        if let Some(cyclic) = self.cached_cyclic { return cyclic }

        //Calculate the answer.
        let cyclic = (|| {
            //If there are no roots then there must be cycles.
            if self.iterable.roots_count() == 0 { return true }

            //The vertices pending processing.
            let mut pending_stack = Vec::new();
            //The ids of all the visited vertices.
            let mut visited = HashSet::new();
            //This is all the vertices which have been visited by the current root.
            let mut root_visited = HashSet::new();
            
            //Iterate all roots to check all paths.
            for root in self.iterable.roots() {
                pending_stack.push(*root);

                //Process all pending vertices.
                while let Some(v) = pending_stack.pop() {
                    //If true there is a cycle.
                    if !root_visited.insert(v) { return true }

                    //Add all of this vertexes outbound neibours to be processed.
                    for &v in self.iterable.out_neighbors(&v) {
                        //If this vertex exists in visited then we have already checked it for cycles.
                        if !visited.contains(&v) {
                            pending_stack.push(v)
                        }
                    }
                }

                //Forget all the vertexes visited from this root specifically.
                visited.extend(root_visited.drain());
            }

            false
        })();

        self.cached_cyclic = Some(cyclic);
        return cyclic;
    }
}

impl<'a, T> Iterator for Dfs<'a, T> {
    type Item = &'a VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.roots_stack.is_empty() {
            let root = self.roots_stack[self.roots_stack.len() - 1].clone();

            // No vertices have been visited yet,
            // so we begin from the current root.
            if self.recursion_stack.is_empty() {
                self.recursion_stack.push(root.clone());
                self.color_map.insert(root.clone(), Color::Grey);

                return self.iterable.fetch_id_ref(root.as_ref());
            }

            // Check if the topmost item on the recursion stack
            // has outbound neighbors. If it does, we traverse
            // them until we find one that is unvisited.
            //
            // If either the topmost item on the recursion stack
            // doesn't have neighbors or all of its neighbors
            // are visited, we pop it from the stack.
            let mut current = self.recursion_stack.pop().unwrap();

            loop {
                if self.iterable.out_neighbors_count(current.as_ref()) == 0
                    && !self.recursion_stack.is_empty()
                {
                    // Mark as processed
                    self.color_map.insert(current.clone(), Color::Black);

                    // Pop from recursion stack
                    current = self.recursion_stack.pop().unwrap();

                    continue;
                }

                break;
            }

            let mut mark = true;

            // Traverse current neighbors
            for n in self.iterable.out_neighbors(current.as_ref()) {
                let reference = Arc::from(*n);

                if let Some(Color::White) = self.color_map.get(&reference) {
                    self.recursion_stack.push(current);
                    self.recursion_stack.push(reference.clone());
                    self.color_map.insert(reference, Color::Grey);
                    mark = false;

                    return Some(n);
                }
            }

            if mark {
                self.color_map.insert(current.clone(), Color::Black);
            }

            // Begin traversing from next root if the
            // recursion stack is empty.
            if self.recursion_stack.is_empty() {
                self.roots_stack.pop();
            }
        }

        None
    }
}
