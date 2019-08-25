use crate::graph::Graph;
use crate::graph::GraphErr;

#[cfg(feature = "no_std")]
use core::io::Write;

#[cfg(not(feature = "no_std"))]
use std::io::Write;

#[cfg(feature = "no_std")]
use core::borrow::Cow;

#[cfg(not(feature = "no_std"))]
use std::borrow::Cow;

#[cfg(feature = "no_std")]
use core::fmt::Debug;

#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

/*
   Bounds on types throw warnings
   type Nd<D: Clone + Debug> = D;
   type Ed<D: Clone + Debug> = (D, D);
   */
type Nd = String;
type Ed = (String, String);

pub(crate) struct Edges<'a> {
    pub(crate) edges: Vec<Ed>,
    pub(crate) graph_name: dot::Id<'a>,
}

impl<'a> Edges<'a> {
    pub fn new(edges: Vec<Ed>, graph_name: &'a str) -> Result<Edges<'a>, GraphErr> {
        let graph_name = dot::Id::new(graph_name).map_err(|_| GraphErr::InvalidGraphName)?;

        Ok(Edges {
            edges,
            graph_name,
        })
    }
}

impl<'a> dot::Labeller<'a, Nd, Ed> for Edges<'a> {
    fn graph_id(&'a self) -> dot::Id { dot::Id::new(self.graph_name.as_slice()).unwrap() }

    fn node_id(&'a self, n: &Nd) -> dot::Id {
        dot::Id::new(n.clone()).unwrap()
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for Edges<'a> {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let &Edges { edges: ref v, .. } = self;
        let mut nodes = Vec::with_capacity(v.len());

        for (s, t) in v.iter() {
            nodes.push(s.clone()); 
            nodes.push(t.clone());
        }

        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        let &Edges { edges: ref edges, .. } = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Ed) -> Nd { e.0.clone() }
    fn target(&self, e: &Ed) -> Nd { e.1.clone() }
}
