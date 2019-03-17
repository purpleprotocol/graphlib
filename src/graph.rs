// Copyright 2019 Octavian Oncescu

use crate::edge::Edge;
use crate::iterators::{Bfs, Dfs, VertexIter};
use crate::vertex_id::VertexId;
use hashbrown::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum GraphErr {
    NoSuchVertex,
    CannotAddEdge,
}

#[derive(Clone, Debug)]
pub struct Graph<T> {
    vertices: HashMap<Arc<VertexId>, (T, Arc<VertexId>)>,
    edges: Vec<Edge>,
    roots: Vec<Arc<VertexId>>,
    inbound_table: HashMap<Arc<VertexId>, Vec<Arc<VertexId>>>,
    outbound_table: HashMap<Arc<VertexId>, Vec<Arc<VertexId>>>,
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            vertices: HashMap::new(),
            edges: Vec::new(),
            roots: Vec::new(),
            inbound_table: HashMap::new(),
            outbound_table: HashMap::new(),
        }
    }

    /// Creates a Graph with the given capacity.
    /// *note: you can push more data into the graph but it will reallocate itself*
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::with_capacity(5);
    /// ```
    pub fn with_capacity(capacity: usize) -> Graph<T> {
        Graph {
            vertices: HashMap::with_capacity(capacity),
            edges: Vec::with_capacity(usize::pow(capacity, 2)),
            roots: Vec::with_capacity(capacity),
            inbound_table: HashMap::with_capacity(capacity),
            outbound_table: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the minimum number of elements the Graph can hold without reallocating.
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::with_capacity(5);
    ///
    /// assert_eq!(graph.capacity(), 5);
    /// ```
    pub fn capacity(&self) -> usize {
        min!(
            self.vertices.capacity(),
            self.edges.capacity(),
            self.roots.capacity(),
            self.inbound_table.capacity(),
            self.outbound_table.capacity()
        )
    }

    /// Reserves capacity for at least additional more elements to be inserted in the given
    /// `Graph`. After calling reserve, capacity will be greater than or equal to `self.len() + additional`.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::with_capacity(5);
    ///
    /// assert_eq!(graph.capacity(), 5);
    ///
    /// graph.reserve(10);
    /// assert_eq!(graph.capacity(), 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.edges.reserve(additional);
        self.roots.reserve(additional);
        self.vertices.reserve(additional);
        self.outbound_table.reserve(additional);
        self.inbound_table.reserve(additional);
    }

    /// Shrinks the capacity of the Graph as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator may still inform the
    /// vector that there is space for a few more elements.
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    /// let mut graph: Graph<usize> = Graph::with_capacity(5);
    ///
    /// assert_eq!(graph.capacity(), 5);
    ///
    /// graph.shrink_to_fit();
    /// assert_eq!(graph.capacity(), 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.edges.shrink_to_fit();
        self.roots.shrink_to_fit();
        self.vertices.shrink_to_fit();
        self.outbound_table.shrink_to_fit();
        self.inbound_table.shrink_to_fit();
    }

    /// Adds a new vertex to the graph and returns the id
    /// of the added vertex.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let id = graph.add_vertex(1);
    ///
    /// assert_eq!(graph.fetch(&id).unwrap(), &1);
    /// ```
    pub fn add_vertex(&mut self, item: T) -> VertexId {
        let id = VertexId::random();
        let id_ptr = Arc::new(id);

        self.vertices.insert(id_ptr.clone(), (item, id_ptr.clone()));
        self.roots.push(id_ptr);

        id
    }

    /// Attempts to place a new edge in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::{Graph, GraphErr, VertexId};
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// // Id of vertex that is not place in the graph
    /// let id = VertexId::random();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(2);
    ///
    /// // Adding an edge is idempotent
    /// graph.add_edge(&v1, &v2);
    /// graph.add_edge(&v1, &v2);
    /// graph.add_edge(&v1, &v2);
    ///
    /// // Fails on adding an edge between an
    /// // existing vertex and a non-existing one.
    /// assert_eq!(graph.add_edge(&v1, &id), Err(GraphErr::NoSuchVertex));
    /// ```
    pub fn add_edge(&mut self, a: &VertexId, b: &VertexId) -> Result<(), GraphErr> {
        if self.has_edge(a, b) {
            return Ok(());
        }

        let a_prime = self.vertices.get(a);
        let b_prime = self.vertices.get(b);

        // Check vertices existence
        match (a_prime, b_prime) {
            (Some((_, id_ptr1)), Some((_, id_ptr2))) => {
                let edge = Edge::new(id_ptr1.clone(), id_ptr2.clone());

                // Push edge
                self.edges.push(edge);

                // Update outbound table
                match self.outbound_table.get(id_ptr1) {
                    Some(outbounds) => {
                        let mut outbounds = outbounds.clone();
                        outbounds.push(id_ptr2.clone());

                        self.outbound_table.insert(id_ptr1.clone(), outbounds);
                    }
                    None => {
                        self.outbound_table
                            .insert(id_ptr1.clone(), vec![id_ptr2.clone()]);
                    }
                }

                // Update inbound table
                match self.inbound_table.get(id_ptr2) {
                    Some(inbounds) => {
                        let mut inbounds = inbounds.clone();
                        inbounds.push(id_ptr1.clone());

                        self.inbound_table.insert(id_ptr2.clone(), inbounds);
                    }
                    None => {
                        self.inbound_table
                            .insert(id_ptr2.clone(), vec![id_ptr1.clone()]);
                    }
                }

                // Remove outbound vertex from roots
                self.roots = self.roots.iter().filter(|v| ***v != *b).cloned().collect();

                Ok(())
            }
            _ => Err(GraphErr::NoSuchVertex),
        }
    }

    /// Checks whether or not exists an edge between
    /// the vertices with the given ids.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(2);
    /// let v3 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    ///
    /// assert!(graph.has_edge(&v1, &v2));
    /// assert!(!graph.has_edge(&v2, &v3));
    /// ```
    pub fn has_edge(&self, a: &VertexId, b: &VertexId) -> bool {
        match self.outbound_table.get(a) {
            Some(outbounds) => outbounds.iter().any(|v| *b == **v),
            None => false,
        }
    }

    /// Returns the total number of edges that are listed
    /// in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v2, &v3).unwrap();
    /// graph.add_edge(&v3, &v4).unwrap();
    ///
    /// assert_eq!(graph.edge_count(), 3);
    /// ```
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the number of vertices that are placed in
    /// the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// assert_eq!(graph.vertex_count(), 3);
    /// ```
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Attempts to fetch a reference to an item placed
    /// in the graph using the provided `VertexId`.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let id = graph.add_vertex(1);
    ///
    /// assert_eq!(*graph.fetch(&id).unwrap(), 1);
    /// ```
    pub fn fetch(&self, id: &VertexId) -> Option<&T> {
        let result = self.vertices.get(id);

        match result {
            Some((result, _)) => Some(result),
            None => None,
        }
    }

    /// Attempts to fetch a mutable reference to an item placed
    /// in the graph using the provided `VertexId`.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let id = graph.add_vertex(1);
    ///
    /// assert_eq!(*graph.fetch(&id).unwrap(), 1);
    ///
    /// // Fetch a mutable reference
    /// let v = graph.fetch_mut(&id).unwrap();
    ///
    /// // Mutate vertex value
    /// *v = 2;
    ///
    /// assert_eq!(*graph.fetch(&id).unwrap(), 2);
    /// ```
    pub fn fetch_mut(&mut self, id: &VertexId) -> Option<&mut T> {
        let result = self.vertices.get_mut(id);

        match result {
            Some((result, _)) => Some(result),
            None => None,
        }
    }

    /// Removes a vertex that matches the given `VertexId`.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(2);
    /// let v3 = graph.add_vertex(3);
    ///
    /// // The remove operation is idempotent
    /// graph.remove(&v2);
    /// graph.remove(&v2);
    /// graph.remove(&v2);
    ///
    /// assert_eq!(graph.vertex_count(), 2);
    /// ```
    pub fn remove(&mut self, id: &VertexId) {
        self.vertices.remove(id);
        self.inbound_table.remove(id);

        // Mark outbounds as roots if they have no inbounds.
        for (n, _) in self.outbound_table.iter() {
            if self.in_neighbors_count(n) == 0 {
                self.roots.push(n.clone());
            }
        }

        self.outbound_table.remove(id);
        self.edges.retain(|e| !e.matches_any(id));
        self.roots.retain(|r| r.as_ref() != id);
    }

    /// Removes the specified edge from the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v2, &v3).unwrap();
    /// graph.add_edge(&v3, &v4).unwrap();
    ///
    /// assert_eq!(graph.edge_count(), 3);
    ///
    /// // The remove edge operation is idempotent
    /// graph.remove_edge(&v2, &v3);
    /// graph.remove_edge(&v2, &v3);
    /// graph.remove_edge(&v2, &v3);
    ///
    /// assert_eq!(graph.edge_count(), 2);
    /// ```
    pub fn remove_edge(&mut self, a: &VertexId, b: &VertexId) {
        let mut remove = false;

        if let Some(outbounds) = self.outbound_table.get_mut(a) {
            outbounds.retain(|v| *v.as_ref() != *b);
            remove = true;
        }

        // If outbound vertex doesn't have any more inbounds,
        // mark it as root.
        if self.in_neighbors_count(&b) == 0 {
            self.roots.push(Arc::from(*b));
        }

        if remove {
            self.edges.retain(|e| !e.matches(a, b));
        }
    }

    /// Iterates through the graph and only keeps
    /// vertices that match the given condition.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(2);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// graph.retain(|v| *v != 2);
    ///
    /// assert_eq!(graph.vertex_count(), 2);
    /// ```
    pub fn retain(&mut self, fun: impl Fn(&T) -> bool) {
        let vertices: Vec<VertexId> = self.vertices().cloned().collect();
        let vertices: Vec<VertexId> = vertices
            .iter()
            .filter(|v| !fun(self.fetch(v).unwrap()))
            .cloned()
            .collect();

        vertices.iter().for_each(|v| self.remove(v));
    }

    /// Performs a fold over the vertices that are
    /// situated in the graph in Depth-First Order.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// let result = graph.fold(0, |v, acc| v + acc);
    ///
    /// assert_eq!(result, 6);
    /// ```
    pub fn fold<A>(&self, initial: A, fun: impl Fn(&T, A) -> A) -> A {
        let mut acc = initial;

        for v in self.dfs() {
            acc = fun(self.fetch(v).unwrap(), acc)
        }

        acc
    }

    /// Returns true if the graph has cycles.
    ///
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// println!("V1: {:?}", v1);
    /// println!("V2: {:?}", v2);
    /// println!("V3: {:?}", v3);
    /// println!("V4: {:?}", v4);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v2, &v3).unwrap();
    /// graph.add_edge(&v3, &v4).unwrap();
    ///
    /// assert!(!graph.is_cyclic());
    ///
    /// graph.add_edge(&v3, &v1);
    ///
    /// assert!(graph.is_cyclic());
    /// ```
    pub fn is_cyclic(&self) -> bool {
        let mut dfs = self.dfs();
        dfs.is_cyclic()
    }

    /// Returns the number of root vertices
    /// in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// assert_eq!(graph.roots_count(), 1);
    /// ```
    pub fn roots_count(&self) -> usize {
        self.roots.len()
    }

    /// Returns the total count of neighboring vertices
    /// of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// assert_eq!(graph.neighbors_count(&v1), 3);
    /// ```
    pub fn neighbors_count(&self, id: &VertexId) -> usize {
        self.in_neighbors_count(id) + self.out_neighbors_count(id)
    }

    /// Returns the total count of inbound neighboring
    /// vertices of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// assert_eq!(graph.in_neighbors_count(&v1), 1);
    /// ```
    pub fn in_neighbors_count(&self, id: &VertexId) -> usize {
        match self.inbound_table.get(id) {
            Some(ins) => ins.len(),
            None => 0,
        }
    }

    /// Returns the total count of outbound neighboring
    /// vertices of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    /// let v5 = graph.add_vertex(4);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    /// graph.add_edge(&v2, &v5).unwrap();
    /// graph.add_edge(&v2, &v3).unwrap();
    ///
    /// assert_eq!(graph.out_neighbors_count(&v1), 2);
    /// assert_eq!(graph.out_neighbors_count(&v2), 2);
    /// ```
    pub fn out_neighbors_count(&self, id: &VertexId) -> usize {
        match self.outbound_table.get(id) {
            Some(outs) => outs.len(),
            None => 0,
        }
    }

    /// Returns an iterator over the inbound neighbors
    /// of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut neighbors = vec![];
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// // Iterate over neighbors
    /// for v in graph.in_neighbors(&v1) {
    ///     neighbors.push(v);
    /// }
    ///
    /// assert_eq!(neighbors.len(), 1);
    /// assert_eq!(neighbors[0], &v3);
    /// ```
    pub fn in_neighbors(&self, id: &VertexId) -> VertexIter<'_> {
        let mut collection: Vec<&VertexId> = vec![];

        if let Some(inbounds) = self.inbound_table.get(id) {
            collection = inbounds.iter().map(|v| v.as_ref()).collect();
        };

        VertexIter::new(collection)
    }

    /// Returns an iterator over the outbound neighbors
    /// of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut neighbors = vec![];
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// // Iterate over neighbors
    /// for v in graph.out_neighbors(&v1) {
    ///     neighbors.push(v);
    /// }
    ///
    /// assert_eq!(neighbors.len(), 2);
    /// assert_eq!(neighbors[0], &v2);
    /// assert_eq!(neighbors[1], &v4);
    /// ```
    pub fn out_neighbors(&self, id: &VertexId) -> VertexIter<'_> {
        let mut collection: Vec<&VertexId> = vec![];

        if let Some(outbounds) = self.outbound_table.get(id) {
            collection = outbounds.iter().map(|v| v.as_ref()).collect();
        };

        VertexIter::new(collection)
    }

    /// Returns an iterator over the outbound neighbors
    /// of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut neighbors = vec![];
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// // Iterate over neighbors
    /// for v in graph.neighbors(&v1) {
    ///     neighbors.push(v);
    /// }
    ///
    /// assert_eq!(neighbors.len(), 3);
    /// assert_eq!(neighbors[0], &v2);
    /// assert_eq!(neighbors[1], &v4);
    /// assert_eq!(neighbors[2], &v3);
    /// ```
    pub fn neighbors(&self, id: &VertexId) -> VertexIter<'_> {
        let mut collection: Vec<&VertexId> = vec![];

        match (self.outbound_table.get(id), self.inbound_table.get(id)) {
            (Some(outbounds), None) => {
                collection = outbounds.iter().map(|v| v.as_ref()).collect();
            }
            (None, Some(inbounds)) => {
                collection = inbounds.iter().map(|v| v.as_ref()).collect();
            }
            (Some(outbounds), Some(inbounds)) => {
                collection = outbounds.iter().map(|v| v.as_ref()).collect();

                let inbounds: Vec<&VertexId> = inbounds.iter().map(|v| v.as_ref()).collect();

                collection.extend_from_slice(&inbounds);
            }
            (None, None) => {} // Do nothing
        };

        VertexIter::new(collection)
    }

    /// Returns an iterator over the root vertices
    /// of the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut roots = vec![];
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// // Iterate over roots
    /// for v in graph.roots() {
    ///     roots.push(v);
    /// }
    ///
    /// assert_eq!(roots.len(), 1);
    /// assert_eq!(roots[0], &v3);
    /// ```
    pub fn roots(&self) -> VertexIter<'_> {
        let collection: Vec<&VertexId> = self.roots.iter().map(|v| v.as_ref()).collect();

        VertexIter::new(collection)
    }

    /// Returns an iterator over all of the
    /// vertices that are placed in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut vertices = vec![];
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// // Iterate over vertices
    /// for v in graph.vertices() {
    ///     vertices.push(v);
    /// }
    ///
    /// assert_eq!(vertices.len(), 4);
    /// ```
    pub fn vertices(&self) -> VertexIter<'_> {
        let collection: Vec<&VertexId> = self.vertices.iter().map(|(v, _)| v.as_ref()).collect();

        VertexIter::new(collection)
    }

    /// Returns an iterator over the vertices
    /// of the graph in Depth-First Order.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut vertices = vec![];
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    ///
    /// // Iterate over vertices
    /// for v in graph.dfs() {
    ///     vertices.push(v);
    /// }
    ///
    /// assert_eq!(vertices.len(), 4);
    /// assert_eq!(vertices[0], &v3);
    /// assert_eq!(vertices[1], &v1);
    /// assert_eq!(vertices[2], &v2);
    /// assert_eq!(vertices[3], &v4);
    /// ```
    pub fn dfs(&self) -> Dfs<'_, T> {
        Dfs::new(self)
    }

    /// Returns an iterator over the vertices
    /// of the graph in Breadth-First Order.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut vertices = vec![];
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    /// let v5 = graph.add_vertex(4);
    /// let v6 = graph.add_vertex(5);
    /// let v7 = graph.add_vertex(6);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v3, &v1).unwrap();
    /// graph.add_edge(&v1, &v4).unwrap();
    /// graph.add_edge(&v1, &v7).unwrap();
    /// graph.add_edge(&v2, &v5).unwrap();
    /// graph.add_edge(&v5, &v6).unwrap();
    ///
    /// // Iterate over vertices
    /// for v in graph.bfs() {
    ///     vertices.push(v);
    /// }
    ///
    /// assert_eq!(vertices.len(), 7);
    /// assert_eq!(vertices[0], &v3);
    /// assert_eq!(vertices[1], &v1);
    /// assert_eq!(vertices[2], &v2);
    /// assert_eq!(vertices[3], &v4);
    /// assert_eq!(vertices[4], &v7);
    /// assert_eq!(vertices[5], &v5);
    /// assert_eq!(vertices[6], &v6);
    /// ```
    pub fn bfs(&self) -> Bfs<'_, T> {
        Bfs::new(self)
    }

    /// Attempts to fetch a reference to a stored vertex id
    /// which is equal to the given `VertexId`.
    pub fn fetch_id_ref<'b>(&'b self, id: &VertexId) -> Option<&'b VertexId> {
        match self.vertices.get(id) {
            Some((_, id_ptr)) => Some(id_ptr.as_ref()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs() {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let v1 = graph.add_vertex(0);
        let v2 = graph.add_vertex(1);
        let v3 = graph.add_vertex(2);
        let v4 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v3, &v1).unwrap();
        graph.add_edge(&v1, &v4).unwrap();

        // Iterate over vertices
        for v in graph.dfs() {
            vertices.push(v);
        }

        assert_eq!(vertices.len(), 4);
        assert_eq!(vertices[0], &v3);
        assert_eq!(vertices[1], &v1);
        assert_eq!(vertices[2], &v2);
        assert_eq!(vertices[3], &v4);
    }

    #[test]
    fn dfs_mul_roots() {
        let mut graph: Graph<usize> = Graph::new();
        let mut vertices = vec![];

        let v1 = graph.add_vertex(0);
        let v2 = graph.add_vertex(1);
        let v3 = graph.add_vertex(2);
        let v4 = graph.add_vertex(3);

        let v5 = graph.add_vertex(4);
        let v6 = graph.add_vertex(5);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v3, &v1).unwrap();
        graph.add_edge(&v1, &v4).unwrap();
        graph.add_edge(&v5, &v6).unwrap();

        // Iterate over vertices
        for v in graph.dfs() {
            vertices.push(v);
        }

        assert_eq!(vertices.len(), 6);
        assert_eq!(vertices[0], &v5);
        assert_eq!(vertices[1], &v6);
        assert_eq!(vertices[2], &v3);
        assert_eq!(vertices[3], &v1);
        assert_eq!(vertices[4], &v2);
        assert_eq!(vertices[5], &v4);
    }
}
