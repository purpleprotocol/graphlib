// Copyright 2019 Octavian Oncescu

use crate::edge::Edge;
use crate::iterators::*;
use crate::vertex_id::VertexId;
use hashbrown::{HashMap, HashSet};

#[cfg(feature = "no_std")]
use core::iter;
#[cfg(not(feature = "no_std"))]
use std::iter;

#[cfg(feature = "no_std")]
use core::fmt::Debug;
#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

#[cfg(feature = "no_std")]
use core::mem;
#[cfg(not(feature = "no_std"))]
use std::mem;

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
use alloc::boxed::Box;
#[cfg(feature = "no_std")]
use alloc::vec;
#[cfg(feature = "no_std")]
use alloc::vec::Vec;

#[cfg(feature = "dot")]
use super::SEED;

#[derive(Clone, Debug, PartialEq)]
/// Graph operation error
pub enum GraphErr {
    /// There is no vertex with the given id in the graph
    NoSuchVertex,

    /// There is no such edge in the graph
    NoSuchEdge,

    /// Could not add an edge to the graph
    CannotAddEdge,

    /// The given weight is invalid
    InvalidWeight,

    /// The operation cannot be performed as it will
    /// create a cycle in the graph.
    CycleError,

    #[cfg(feature = "dot")]
    /// Could not render .dot file
    CouldNotRender,

    #[cfg(feature = "dot")]
    /// The name of the graph is invalid. Check [this](https://docs.rs/dot/0.1.1/dot/struct.Id.html#method.new)
    /// out for more information.
    InvalidGraphName,

    #[cfg(feature = "dot")]
    /// The name of the given label is invalid. Check [this](https://docs.rs/dot/0.1.1/dot/struct.Id.html#method.new)
    /// out for more information.
    InvalidLabel,
}

#[derive(Clone, Debug, Default)]
/// Graph data-structure
pub struct Graph<T> {
    /// Mapping of vertex ids and vertex values
    vertices: HashMap<VertexId, (T, VertexId)>,

    /// Mapping between edges and weights
    edges: HashMap<Edge, f32>,

    /// Set containing the roots of the graph
    roots: HashSet<VertexId>,

    /// Set containing the tips of the graph
    tips: HashSet<VertexId>,

    /// Mapping between vertex ids and inbound edges
    inbound_table: HashMap<VertexId, Vec<VertexId>>,

    /// Mapping between vertex ids and outbound edges
    outbound_table: HashMap<VertexId, Vec<VertexId>>,

    #[cfg(feature = "dot")]
    /// Mapping between vertices and labels
    labels: HashMap<VertexId, String>,
}

impl<T> Graph<T> {
    /// Creates a new graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// graph.add_vertex(0);
    /// assert_eq!(graph.vertex_count(), 1);
    /// ```
    pub fn new() -> Graph<T> {
        Graph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            roots: HashSet::new(),
            tips: HashSet::new(),
            inbound_table: HashMap::new(),
            outbound_table: HashMap::new(),

            #[cfg(feature = "dot")]
            labels: HashMap::new(),
        }
    }

    /// Creates a new graph with the given capacity.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::with_capacity(5);
    /// ```
    pub fn with_capacity(capacity: usize) -> Graph<T> {
        let edges_capacity = if capacity < 100 {
            usize::pow(capacity, 2)
        } else {
            capacity
        };

        Graph {
            vertices: HashMap::with_capacity(capacity),
            edges: HashMap::with_capacity(edges_capacity),
            roots: HashSet::with_capacity(capacity),
            tips: HashSet::with_capacity(capacity),
            inbound_table: HashMap::with_capacity(capacity),
            outbound_table: HashMap::with_capacity(capacity),

            #[cfg(feature = "dot")]
            labels: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the current capacity of the graph.
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::with_capacity(5);
    ///
    /// assert!(graph.capacity() >= 5);
    /// ```
    pub fn capacity(&self) -> usize {
        min!(
            self.vertices.capacity(),
            self.edges.capacity(),
            self.roots.capacity(),
            self.tips.capacity(),
            self.inbound_table.capacity(),
            self.outbound_table.capacity()
        )
    }

    /// Reserves capacity for at least additional more elements to be inserted in the given
    /// graph. After calling reserve, capacity will be greater than or equal to `self.vertex_count() + additional`.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::with_capacity(3);
    ///
    /// assert_eq!(graph.capacity(), 3);
    ///
    /// graph.add_vertex(0);
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    ///
    /// graph.reserve(10);
    /// assert!(graph.capacity() >= 13);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        // Calculate additional value for edges vector
        // such that it is always n^2 where n is the
        // number of vertices that are currently placed
        // in the graph.
        let new_capacity = self.vertices.len() + additional;
        let edges_capacity = if new_capacity < 100 {
            usize::pow(new_capacity, 2)
        } else {
            new_capacity
        };
        let edges_count = self.edges.len();
        let edges_additional = edges_capacity - edges_count;

        self.edges.reserve(edges_additional);
        self.roots.reserve(additional);
        self.tips.reserve(additional);
        self.vertices.reserve(additional);
        self.outbound_table.reserve(additional);
        self.inbound_table.reserve(additional);

        #[cfg(feature = "dot")]
        self.labels.reserve(additional);
    }

    /// Shrinks the capacity of the graph as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator may still inform the
    /// vector that there is space for a few more elements.
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::with_capacity(5);
    ///
    /// assert!(graph.capacity() >= 5);
    ///
    /// graph.shrink_to_fit();
    /// assert!(graph.capacity() < 5);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.edges.shrink_to_fit();
        self.roots.shrink_to_fit();
        self.tips.shrink_to_fit();
        self.vertices.shrink_to_fit();
        self.outbound_table.shrink_to_fit();
        self.inbound_table.shrink_to_fit();

        #[cfg(feature = "dot")]
        self.labels.shrink_to_fit();

        // Calculate additional value for edges vector
        // such that it is always n^2 where n is the
        // number of vertices that are currently placed
        // in the graph.
        let edges_capacity = usize::pow(self.vertices.len(), 2);
        let edges_count = self.edges.len();
        let edges_additional = edges_capacity - edges_count;

        self.edges.reserve(edges_additional);
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

        self.vertices.insert(id, (item, id));
        self.roots.insert(id);
        self.tips.insert(id);

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

        self.do_add_edge(a, b, 0.0, false)
    }

    /// Attempts to place a new edge in the graph, checking if the specified
    /// edge will create a cycle in the graph. If it does, this operation will fail.
    ///
    /// Note that this operation has a bigger performance hit than `Graph::add_edge()`.
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
    /// graph.add_edge_check_cycle(&v1, &v2);
    /// graph.add_edge_check_cycle(&v1, &v2);
    /// graph.add_edge_check_cycle(&v1, &v2);
    ///
    /// // Fails on adding an edge which creates
    /// // a cycle in the graph.
    /// assert_eq!(graph.add_edge_check_cycle(&v2, &v1), Err(GraphErr::CycleError));
    /// ```
    pub fn add_edge_check_cycle(&mut self, a: &VertexId, b: &VertexId) -> Result<(), GraphErr> {
        if self.has_edge(a, b) {
            return Ok(());
        }

        self.do_add_edge(a, b, 0.0, true)
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
    /// graph.add_edge_with_weight(&v1, &v2, 0.3);
    ///
    /// // Fails on adding an edge between an
    /// // existing vertex and a non-existing one.
    /// assert_eq!(graph.weight(&v1, &v2), Some(0.3));
    /// ```
    pub fn add_edge_with_weight(
        &mut self,
        a: &VertexId,
        b: &VertexId,
        weight: f32,
    ) -> Result<(), GraphErr> {
        if self.has_edge(a, b) {
            return Ok(());
        }

        if weight > 1.0 || weight < -1.0 {
            return Err(GraphErr::InvalidWeight);
        }

        self.do_add_edge(a, b, weight, false)
    }

    /// Returns the weight of the specified edge
    /// if it is listed.
    ///
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
    /// let v3 = graph.add_vertex(3);
    ///
    /// // Adding an edge is idempotent
    /// graph.add_edge_with_weight(&v1, &v2, 0.54543);
    ///
    /// assert_eq!(graph.weight(&v1, &v2), Some(0.54543));
    /// assert_eq!(graph.weight(&v1, &v3), None);
    /// ```
    pub fn weight(&self, a: &VertexId, b: &VertexId) -> Option<f32> {
        if !self.has_edge(a, b) {
            return None;
        }

        if let Some(result) = self.edges.get(&Edge::new(*a, *b)) {
            Some(*result)
        } else {
            None
        }
    }

    /// Sets the weight of the edge to the new value
    /// if the edge exists in the graph. Note that
    /// the given weight must be a number between
    /// (and including) `-1.0` and `1.0`.
    ///
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
    /// let v3 = graph.add_vertex(3);
    ///
    /// graph.add_edge_with_weight(&v1, &v2, 0.54543);
    /// assert_eq!(graph.weight(&v1, &v2), Some(0.54543));
    ///
    /// // Set new weight
    /// graph.set_weight(&v1, &v2, 0.123).unwrap();
    /// assert_eq!(graph.weight(&v1, &v2), Some(0.123));
    /// ```
    pub fn set_weight(
        &mut self,
        a: &VertexId,
        b: &VertexId,
        new_weight: f32,
    ) -> Result<(), GraphErr> {
        if !self.has_edge(a, b) {
            return Err(GraphErr::NoSuchEdge);
        }

        if new_weight > 1.0 || new_weight < -1.0 {
            return Err(GraphErr::InvalidWeight);
        }

        self.edges.insert(Edge::new(*a, *b), new_weight);

        // Sort outbound vertices after setting a new weight
        let mut outbounds = self.outbound_table.get(a).unwrap().clone();

        self.sort_outbounds(a.clone(), &mut outbounds);

        // Update outbounds
        self.outbound_table.insert(a.clone(), outbounds);

        Ok(())
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
            Some(outbounds) => outbounds.contains(b),
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

        // Remove each inbound edge
        if let Some(inbounds) = self.inbound_table.remove(id) {
            for vertex in inbounds {
                self.remove_edge(&vertex, id);

                // Add to tips if inbound vertex doesn't
                // have other outbound vertices.
                if self.out_neighbors_count(&vertex) == 0 {
                    self.tips.insert(vertex);
                }
            }
        }

        // Remove each outbound edge
        if let Some(outbounds) = self.outbound_table.remove(id) {
            for vertex in outbounds {
                self.remove_edge(id, &vertex);

                // Add to roots if outbound vertex doesn't
                // have other inbound vertices.
                if self.in_neighbors_count(&vertex) == 0 {
                    self.roots.insert(vertex);
                }
            }
        }

        self.roots.remove(&id);
        self.tips.remove(&id);
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
        if let Some(outbounds) = self.outbound_table.get_mut(a) {
            outbounds.retain(|v| v != b);
            if outbounds.is_empty() {
                self.outbound_table.remove(a);
            }
        }

        if let Some(inbounds) = self.inbound_table.get_mut(b) {
            inbounds.retain(|v| v != a);
            if inbounds.is_empty() {
                self.inbound_table.remove(b);
            }
        }

        // If outbound vertex doesn't have any more inbounds,
        // mark it as root.
        if self.in_neighbors_count(&b) == 0 {
            self.roots.insert(b.clone());
        }

        // Mark vertex as tip if it doesn't have any more outbounds.
        if self.out_neighbors_count(&a) == 0 {
            self.tips.insert(a.clone());
        }

        self.edges.remove(&Edge::new(*a, *b));
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
        let vertices: Vec<VertexId> = self
            .vertices()
            .filter(|v| !fun(self.fetch(v).unwrap()))
            .cloned()
            .collect();

        vertices.iter().for_each(|v| self.remove(&v));
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

    /// Performs a map over all of the vertices of the graph,
    /// applying the given transformation function to each one.
    ///
    /// Returns a new graph with the same edges but with transformed
    /// vertices.
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let id1 = graph.add_vertex(1);
    /// let id2 = graph.add_vertex(2);
    ///
    /// graph.add_edge(&id1, &id2);
    ///
    /// // Map each vertex
    /// let mapped: Graph<usize> = graph.map(|v| v + 2);
    ///
    /// assert!(graph.has_edge(&id1, &id2));
    /// assert!(mapped.has_edge(&id1, &id2));
    /// assert_eq!(graph.fetch(&id1).unwrap(), &1);
    /// assert_eq!(graph.fetch(&id2).unwrap(), &2);
    /// assert_eq!(mapped.fetch(&id1).unwrap(), &3);
    /// assert_eq!(mapped.fetch(&id2).unwrap(), &4);
    /// ```
    pub fn map<R>(&self, fun: impl Fn(&T) -> R) -> Graph<R> {
        let mut graph: Graph<R> = Graph::new();

        // Copy edge and vertex information
        graph.edges = self.edges.clone();
        graph.roots = self.roots.clone();
        graph.tips = self.tips.clone();
        graph.inbound_table = self.inbound_table.clone();
        graph.outbound_table = self.outbound_table.clone();
        graph.vertices = self
            .vertices
            .iter()
            .map(|(id, (v, i))| (*id, (fun(v), *i)))
            .collect();

        #[cfg(feature = "dot")]
        {
            graph.labels = self.labels.clone();
        }

        graph
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
        match self.inbound_table.get(id) {
            Some(neighbors) => VertexIter(Box::new(neighbors.iter().map(AsRef::as_ref))),
            None => VertexIter(Box::new(iter::empty())),
        }
    }

    /// Returns an iterator over the outbound neighbors
    /// of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// #[macro_use] extern crate graphlib;
    /// use std::collections::HashSet;
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
    /// assert!(set![&v2, &v4] == graph.out_neighbors(&v1).collect());
    /// ```
    pub fn out_neighbors(&self, id: &VertexId) -> VertexIter<'_> {
        match self.outbound_table.get(id) {
            Some(iter) => VertexIter(Box::new(iter.iter().rev().map(AsRef::as_ref))),
            None => VertexIter(Box::new(iter::empty())),
        }
    }

    /// Returns an iterator over the inbound and outbound neighbors
    /// of the vertex with the given id.
    ///
    /// ## Example
    /// ```rust
    /// #[macro_use] extern crate graphlib;
    /// use std::collections::HashSet;
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
    /// assert!(set![&v2, &v4, &v3] == graph.neighbors(&v1).collect());
    /// ```
    pub fn neighbors(&self, id: &VertexId) -> VertexIter<'_> {
        let mut visited = HashSet::new();
        let neighbors = self
            .out_neighbors(id)
            .chain(self.in_neighbors(id))
            //Remove duplicates.
            .filter(move |&&v| visited.insert(v));

        VertexIter(Box::new(neighbors))
    }

    /// Returns an iterator over all edges that are situated
    /// in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut edges = vec![];
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
    /// // Iterate over edges
    /// for v in graph.edges() {
    ///     edges.push(v);
    /// }
    ///
    /// assert_eq!(edges.len(), 3);
    /// ```
    pub fn edges(&self) -> impl Iterator<Item = (&VertexId, &VertexId)> {
        self.edges.iter().map(|(e, _)| (e.inbound(), e.outbound()))
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
        VertexIter(Box::new(self.roots.iter().map(AsRef::as_ref)))
    }

    /// Returns an iterator over the tips of the graph. These
    /// are all the vertices that have an inbound edge but no
    /// outbound edge.
    ///
    /// ## Example
    /// ```rust
    /// #[macro_use] extern crate graphlib;
    /// use std::collections::HashSet;
    /// use graphlib::Graph;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let mut tips = set![];
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
    /// // Iterate over tips
    /// for v in graph.tips() {
    ///     tips.insert(v);
    /// }
    ///
    /// assert_eq!(tips.len(), 2);
    /// assert_eq!(tips, set![&v2, &v4]);
    /// ```
    pub fn tips(&self) -> VertexIter<'_> {
        VertexIter(Box::new(self.tips.iter().map(AsRef::as_ref)))
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
        VertexIter(Box::new(self.vertices.keys().map(AsRef::as_ref)))
    }

    /// Returns an iterator over the vertices
    /// of the graph in Depth-First Order. The iterator
    /// will follow vertices with lower weights first.
    ///
    /// ## Example
    /// ```rust
    /// #[macro_use] extern crate graphlib;
    /// use graphlib::Graph;
    /// use std::collections::HashSet;
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
    /// let mut dfs = graph.dfs();
    ///
    /// assert_eq!(dfs.next(), Some(&v3));
    /// assert_eq!(dfs.next(), Some(&v1));
    /// assert!(set![&v2, &v4] == dfs.collect());
    /// ```
    pub fn dfs(&self) -> Dfs<'_, T> {
        Dfs::new(self)
    }

    /// Returns an iterator over the vertices
    /// of the graph in Breadth-First Order. The iterator
    /// will follow vertices with lower weights first.
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
    /// ```
    pub fn bfs(&self) -> Bfs<'_, T> {
        Bfs::new(self)
    }

    /// Returns an iterator over the vertices
    /// of the graph which follows a DFS based
    /// topological order (Kahn's algorithm).
    ///
    /// Topological sorting is not possible for
    /// graphs which contain a cycle. You may
    /// use topo.is_cylic() == false to verify
    /// that your graph is a DAG.
    ///
    /// If you attempt to use a topological
    /// order without confirming that your graph
    /// is a DAG, you may encounter a panic!().
    ///
    /// The panic!() will be encountered when
    /// the iterator detects that there are no
    /// more vertices to visit, but all vertices
    /// have not been visited.
    ///
    /// ## Example
    /// ```rust
    /// #[macro_use] extern crate graphlib;
    /// use graphlib::Graph;
    /// use std::collections::HashSet;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(2);
    /// let v3 = graph.add_vertex(3);
    /// let v4 = graph.add_vertex(4);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v2, &v3).unwrap();
    /// graph.add_edge(&v3, &v4).unwrap();
    ///
    /// let mut topo = graph.topo();
    ///
    /// assert_eq!(topo.next(), Some(&v1));
    /// assert_eq!(topo.next(), Some(&v2));
    /// assert!(set![&v3, &v4] == topo.collect());
    /// ```
    pub fn topo(&self) -> Topo<'_, T> {
        Topo::new(self)
    }

    /// Returns an iterator over the shortest path from the source
    /// vertex to the destination vertex. The iterator will yield
    /// `None` if there is no such path or the provided vertex ids
    /// do not belong to any vertices in the graph.
    /// ## Example
    /// ```rust
    /// #[macro_use] extern crate graphlib;
    /// use graphlib::Graph;
    /// use std::collections::HashSet;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(2);
    /// let v3 = graph.add_vertex(3);
    /// let v4 = graph.add_vertex(4);
    /// let v5 = graph.add_vertex(5);
    /// let v6 = graph.add_vertex(6);
    ///
    /// graph.add_edge(&v1, &v2).unwrap();
    /// graph.add_edge(&v2, &v3).unwrap();
    /// graph.add_edge(&v3, &v4).unwrap();
    /// graph.add_edge(&v3, &v5).unwrap();
    /// graph.add_edge(&v5, &v6).unwrap();
    /// graph.add_edge(&v6, &v4).unwrap();
    ///
    /// let mut dijkstra = graph.dijkstra(&v1, &v4);
    ///
    /// assert_eq!(dijkstra.next(), Some(&v1));
    /// assert_eq!(dijkstra.next(), Some(&v2));
    /// assert_eq!(dijkstra.next(), Some(&v3));
    /// assert_eq!(dijkstra.next(), Some(&v4));
    /// assert_eq!(dijkstra.next(), None);
    /// ```
    pub fn dijkstra<'a>(&'a self, src: &'a VertexId, dest: &'a VertexId) -> VertexIter<'a> {
        if let Some(dijkstra) = Dijkstra::new(&self, src).ok() {
            if let Some(iter) = dijkstra.get_path_to(dest).ok() {
                iter
            } else {
                VertexIter(Box::new(iter::empty()))
            }
        } else {
            VertexIter(Box::new(iter::empty()))
        }
    }

    /// Returns an iterator over the values of the vertices
    /// placed in the graph.
    ///
    /// ## Example
    /// ```rust
    /// #[macro_use] extern crate graphlib;
    /// use graphlib::Graph;
    /// use std::collections::HashSet;
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(2);
    /// let v3 = graph.add_vertex(3);
    ///
    /// let mut values = graph.values();
    ///
    /// assert!(set![&1, &2, &3] == values.collect());
    /// ```
    pub fn values(&self) -> ValuesIter<'_, T> {
        let iter = self.vertices.values().map(|(v, _)| v);

        ValuesIter(Box::new(iter))
    }

    #[cfg(feature = "dot")]
    /// Creates a file with the dot representation of the graph.
    /// This method requires the `dot` crate feature.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::Graph;
    ///
    /// use std::fs::File;
    /// let mut f = File::create("example1.dot").unwrap();
    ///
    /// let mut graph: Graph<String> = Graph::new();
    ///
    ///  let v1 = graph.add_vertex("test1".to_string());
    ///  let v2 = graph.add_vertex("test2".to_string());
    ///  let v3 = graph.add_vertex("test3".to_string());
    ///  let v4 = graph.add_vertex("test4".to_string());
    ///
    ///  let v5 = graph.add_vertex("test5".to_string());
    ///  let v6 = graph.add_vertex("test6".to_string());
    ///
    ///  graph.add_edge(&v1, &v2).unwrap();
    ///  graph.add_edge(&v3, &v1).unwrap();
    ///  graph.add_edge(&v1, &v4).unwrap();
    ///  graph.add_edge(&v5, &v6).unwrap();
    ///
    ///  assert!(graph.to_dot("example1", &mut f).is_ok());
    /// ```
    pub fn to_dot(
        &self,
        graph_name: &str,
        output: &mut impl ::std::io::Write,
    ) -> Result<(), GraphErr> {
        let edges: Vec<(_, _)> = self
            .edges
            .iter()
            .map(|(w, _)| {
                let inbound = w.inbound();
                let outbound = w.outbound();

                (self.label(inbound).unwrap(), self.label(outbound).unwrap())
            })
            .collect();

        let edges = crate::dot::Edges::new(edges, graph_name)?;
        dot::render(&edges, output).map_err(|_| GraphErr::CouldNotRender)
    }

    #[cfg(feature = "dot")]
    /// Labels the vertex with the given id. Returns the old label if successful.
    ///
    /// This method requires the `dot` crate feature.
    ///
    /// ## Example
    /// ```rust
    /// use graphlib::{Graph, VertexId};
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let random_id = VertexId::random();
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    ///
    /// assert!(graph.label_vertex(&v1, "V1").is_ok());
    /// assert!(graph.label_vertex(&v2, "V2").is_ok());
    /// assert!(graph.label_vertex(&v3, "V3").is_ok());
    /// assert!(graph.label_vertex(&random_id, "will fail").is_err());
    /// ```
    pub fn label_vertex(&mut self, vertex_id: &VertexId, label: &str) -> Result<String, GraphErr> {
        // Check label validity
        let _ = dot::Id::new(label.to_owned()).map_err(|_| GraphErr::InvalidLabel)?;

        if self.vertices.get(vertex_id).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        let old_label = self.label(vertex_id).unwrap();
        self.labels.insert(vertex_id.clone(), label.to_owned());

        Ok(old_label)
    }

    #[cfg(feature = "dot")]
    /// Retrieves the label of the vertex with the given id.
    ///
    /// This method requires the `dot` crate feature.
    ///
    /// This function will return a default label if no label is set. Returns
    /// `None` if there is no vertex associated with the given id in the graph.
    pub fn label(&self, vertex_id: &VertexId) -> Option<String> {
        if self.vertices.get(vertex_id).is_none() {
            return None;
        }

        if let Some(label) = self.labels.get(vertex_id) {
            return Some(label.clone());
        }

        let bytes = super::gen_bytes();

        // Take only 8 bytes out of 16
        let to_encode: Vec<u8> = bytes.iter().take(8).cloned().collect();

        let encoded = hex::encode(&to_encode);
        let label = format!("N_{}", encoded);
        debug_assert!(dot::Id::new(label.to_owned()).is_ok());

        unsafe {
            let labels_ptr = mem::transmute::<
                &HashMap<VertexId, String>,
                &mut HashMap<VertexId, String>,
            >(&self.labels);
            labels_ptr.insert(vertex_id.clone(), label);
        }

        self.labels.get(vertex_id).cloned()
    }

    #[cfg(feature = "dot")]
    /// Maps each label that is placed on a vertex to a new label.
    ///
    /// This method requires the `dot` crate feature.
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use graphlib::{Graph, VertexId};
    ///
    /// let mut graph: Graph<usize> = Graph::new();
    /// let random_id = VertexId::random();
    /// let mut vertex_id: usize = 1;
    ///
    /// let v1 = graph.add_vertex(0);
    /// let v2 = graph.add_vertex(1);
    /// let v3 = graph.add_vertex(2);
    /// let v4 = graph.add_vertex(3);
    ///
    /// assert!(graph.label_vertex(&v1, &format!("V{}", vertex_id)).is_ok());
    /// vertex_id += 1;
    ///
    /// assert!(graph.label_vertex(&v2, &format!("V{}", vertex_id)).is_ok());
    /// vertex_id += 1;
    ///
    /// assert!(graph.label_vertex(&v3, &format!("V{}", vertex_id)).is_ok());
    ///
    /// assert_eq!(graph.label(&v1).unwrap(), "V1");
    /// assert_eq!(graph.label(&v2).unwrap(), "V2");
    /// assert_eq!(graph.label(&v3).unwrap(), "V3");
    ///
    /// let new_labels: HashMap<VertexId, String> = vec![v1.clone(), v2.clone(), v3.clone(), v4.clone()]
    ///     .iter()
    ///     .map(|id| {
    ///         vertex_id += 1;
    ///         let label = format!("V{}", vertex_id);
    ///
    ///         (id.clone(), label)
    ///     })
    ///     .collect();
    ///
    /// graph.map_labels(|id, _old_label| new_labels.get(id).unwrap().clone());
    ///
    /// assert_eq!(graph.label(&v1).unwrap(), "V4");
    /// assert_eq!(graph.label(&v2).unwrap(), "V5");
    /// assert_eq!(graph.label(&v3).unwrap(), "V6");
    /// assert_eq!(graph.label(&v4).unwrap(), "V7");
    /// ```
    pub fn map_labels(&mut self, mut fun: impl FnMut(&VertexId, &str) -> String) {
        // Initialize all labels
        for (v, _) in self.vertices.iter() {
            let _ = self.label(v);
        }

        for (id, l) in self.labels.iter_mut() {
            let new_label = fun(id, l);
            *l = new_label;
        }
    }

    fn do_add_edge(
        &mut self,
        a: &VertexId,
        b: &VertexId,
        weight: f32,
        check_cycle: bool,
    ) -> Result<(), GraphErr> {
        let id_ptr1 = if self.vertices.get(a).is_some() {
            *a
        } else {
            return Err(GraphErr::NoSuchVertex);
        };

        let id_ptr2 = if self.vertices.get(b).is_some() {
            *b
        } else {
            return Err(GraphErr::NoSuchVertex);
        };

        let edge = Edge::new(id_ptr1, id_ptr2);

        // Push edge
        self.edges.insert(edge, weight);

        // Update outbound table
        match self.outbound_table.get(&id_ptr1) {
            Some(outbounds) => {
                let mut outbounds = outbounds.clone();
                outbounds.push(id_ptr2.clone());

                self.sort_outbounds(id_ptr1.clone(), &mut outbounds);
                self.outbound_table.insert(id_ptr1.clone(), outbounds);
            }
            None => {
                self.outbound_table.insert(id_ptr1.clone(), vec![id_ptr2]);
            }
        }

        // Update inbound table
        match self.inbound_table.get_mut(&id_ptr2) {
            Some(inbounds) => {
                inbounds.push(id_ptr1);
            }
            None => {
                self.inbound_table.insert(id_ptr2, vec![id_ptr1]);
            }
        }

        // Remove outbound vertex from roots
        let was_root = self.roots.remove(&b);

        // Remove inbound vertex from tips
        let was_tip = self.tips.remove(&a);

        let mut is_cyclic = false;

        if check_cycle {
            let mut dfs = Dfs::new(&self);
            is_cyclic = dfs.is_cyclic();
        }

        // Roll-back changes if cycle check succeeds
        if is_cyclic {
            // Remove from edge table
            self.remove_edge(a, b);

            if was_root {
                self.roots.insert(b.clone());
            }

            if was_tip {
                self.tips.insert(a.clone());
            }

            return Err(GraphErr::CycleError);
        }

        Ok(())
    }

    fn sort_outbounds(&self, inbound: VertexId, outbounds: &mut Vec<VertexId>) {
        let outbound_weights: HashMap<VertexId, f32> = outbounds
            .iter()
            .map(|id| (*id, *self.edges.get(&Edge::new(inbound, *id)).unwrap()))
            .collect();

        // Sort outbounds
        outbounds.sort_by(|a, b| {
            let a_weight = outbound_weights.get(a).cloned();
            let b_weight = outbound_weights.get(b).cloned();

            match (a_weight, b_weight) {
                // Sort normally if both weights are set
                (Some(a_weight), Some(b_weight)) => {
                    a_weight.partial_cmp(&b_weight).unwrap_or_else(|| a.cmp(b))
                }
                (Some(weight), None) => {
                    if weight != 0.00 {
                        weight.partial_cmp(&0.00).unwrap_or_else(|| a.cmp(b))
                    } else {
                        // Fallback to lexicographic sort
                        a.cmp(b)
                    }
                }
                (None, Some(weight)) => {
                    if weight != 0.00 {
                        weight.partial_cmp(&0.00).unwrap_or_else(|| a.cmp(b))
                    } else {
                        // Fallback to lexicographic sort
                        a.cmp(b)
                    }
                }
                // Sort lexicographically by ids if no weight is set
                (None, None) => a.cmp(b),
            }
        });
    }

    /// Attempts to fetch a reference to a stored vertex id
    /// which is equal to the given `VertexId`.
    pub(crate) fn fetch_id_ref<'b>(&'b self, id: &VertexId) -> Option<&'b VertexId> {
        match self.vertices.get(id) {
            Some((_, id_ptr)) => Some(id_ptr),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_thread_safe() {
        let mut graph: Graph<usize> = Graph::new();
        graph.add_vertex(0);

        std::panic::set_hook(Box::new(move |_| {
            let mut graph = graph.clone();

            graph.add_vertex(1);
            graph.add_vertex(2);
        }));
    }

    #[test]
    fn dfs() {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(0);
        let v2 = graph.add_vertex(1);
        let v3 = graph.add_vertex(2);
        let v4 = graph.add_vertex(3);
        let v5 = graph.add_vertex(4);
        let v6 = graph.add_vertex(5);
        let v7 = graph.add_vertex(6);

        graph.add_edge_with_weight(&v1, &v2, -0.23).unwrap();
        graph.add_edge(&v3, &v1).unwrap();
        graph.add_edge_with_weight(&v1, &v4, -0.56).unwrap();
        graph.add_edge_with_weight(&v1, &v5, 0.44).unwrap();
        graph.add_edge(&v5, &v6).unwrap();
        graph.add_edge(&v5, &v7).unwrap();

        graph.set_weight(&v5, &v6, 0.23).unwrap();
        graph.set_weight(&v5, &v7, 0.33).unwrap();

        let mut dfs = graph.dfs();

        assert_eq!(dfs.next(), Some(&v3));
        assert_eq!(dfs.next(), Some(&v1));
        assert_eq!(dfs.next(), Some(&v4));
        assert_eq!(dfs.next(), Some(&v2));
        assert_eq!(dfs.next(), Some(&v5));
        assert_eq!(dfs.next(), Some(&v6));
        assert_eq!(dfs.next(), Some(&v7));
    }

    #[test]
    fn dfs_mul_roots() {
        let mut graph: Graph<usize> = Graph::new();

        let v1 = graph.add_vertex(0);
        let v2 = graph.add_vertex(1);
        let v3 = graph.add_vertex(2);
        let v4 = graph.add_vertex(3);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v3, &v1).unwrap();
        graph.add_edge(&v1, &v4).unwrap();

        let v5 = graph.add_vertex(4);
        let v6 = graph.add_vertex(5);

        graph.add_edge(&v5, &v6).unwrap();

        // Iterate over vertices
        let mut dfs = graph.dfs();

        for _ in 0..2 {
            let v = dfs.next();

            if v == Some(&v3) {
                assert_eq!(dfs.next(), Some(&v1));
                assert!(set![&v2, &v4] == (&mut dfs).take(2).collect());
            } else if v == Some(&v5) {
                assert_eq!(dfs.next(), Some(&v6));
            } else {
                panic!("Not a root node")
            }
        }

        assert_eq!(dfs.count(), 0, "There were remaining nodes");
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = Graph::new();

        let v1 = graph.add_vertex(0);
        let v2 = graph.add_vertex(1);
        let v3 = graph.add_vertex(2);

        graph.add_edge(&v1, &v2).unwrap();
        graph.add_edge(&v2, &v3).unwrap();

        let old_inbound = graph.inbound_table.clone();
        let old_outbound = graph.outbound_table.clone();

        graph.add_edge(&v3, &v1).unwrap();
        graph.remove_edge(&v3, &v1);

        assert_eq!(old_inbound, graph.inbound_table.clone());
        assert_eq!(old_outbound, graph.outbound_table);
    }

    #[test]
    fn test_non_clonable_type() {
        // this simply tests that a Graph that has a non-clonable type can be created
        // this is done easiest by adding dyn Trait object, which can never be cloned
        //
        // It also tests that the dyn object can still be used as expected
        let mut graph = Graph::<Box<dyn std::fmt::Display>>::new();

        graph.add_vertex(Box::new(String::from("Hello World")));
        let mut result = String::new();
        for vertex_identifier in graph.vertices() {
            if let Some(v) = graph.fetch(vertex_identifier) {
                result = format!("{}", v);
            }
        }

        assert_eq!(result, "Hello World");
    }
    #[test]
    fn test_clonable() {
        let mut graph = Graph::new();
        graph.add_vertex(String::from("Test"));

        let cloned = graph.clone();
        assert_eq!(graph.vertex_count(), cloned.vertex_count());
        let mut cloned_iter = cloned.vertices();
        for vertex_identifier in graph.vertices() {
            if let Some(cloned_identifier) = cloned_iter.next() {
                assert_eq!(
                    graph.fetch(vertex_identifier),
                    cloned.fetch(cloned_identifier)
                );
            } else {
                panic!("graph and clone of graph are not equal!");
            }
        }
    }

    #[test]
    fn test_add_edge_cycle_check() {
        let mut graph: Graph<usize> = Graph::new();

        // Id of vertex that is not place in the graph
        let id = VertexId::random();

        let v1 = graph.add_vertex(1);
        let v2 = graph.add_vertex(2);

        // Adding an edge is idempotent
        graph.add_edge_check_cycle(&v1, &v2).unwrap();
        graph.add_edge_check_cycle(&v1, &v2).unwrap();
        graph.add_edge_check_cycle(&v1, &v2).unwrap();

        let mut graph2 = graph.clone();

        // Fails on adding an edge which creates
        // a cycle in the graph.
        assert_eq!(
            graph2.add_edge_check_cycle(&v2, &v1),
            Err(GraphErr::CycleError)
        );

        // Check that the graph state has rolled back
        assert_eq!(graph.edges, graph2.edges);
        assert_eq!(graph.roots, graph2.roots);
        assert_eq!(graph.tips, graph2.tips);
        assert_eq!(graph.inbound_table, graph2.inbound_table);
        assert_eq!(graph.outbound_table, graph2.outbound_table);
    }
}
