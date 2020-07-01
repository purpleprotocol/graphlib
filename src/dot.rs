use crate::{Graph, GraphErr, VertexId};

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

type Nd = VertexId;
type Ed<'a> = (&'a VertexId, &'a VertexId);


pub(crate) struct DotGraph<'a, T> {
    name: dot::Id<'a>,
    graph: &'a Graph<T>,
}


impl<'a, T> DotGraph<'a, T> {
    pub fn new(graph: &'a Graph<T>, name: &'a str) -> Result<DotGraph<'a, T>, GraphErr> {
        let name = dot::Id::new(name)
            .map_err(|_| GraphErr::InvalidGraphName)?;
        Ok(DotGraph { name, graph })
    }
}


impl<'a, T> dot::Labeller<'a, Nd, Ed<'a>> for DotGraph<'a, T> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new(self.name.as_slice()).unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        let hex = format!("N{}", hex::encode(n.bytes()));
        dot::Id::new(hex).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        let label = self.graph.vertex_label(n).unwrap();
        dot::LabelText::label(Cow::Borrowed(label))
    }

    fn edge_label<'b>(&'b self, e: &Ed) -> dot::LabelText<'b> {
        let label = self.graph.edge_label(e.0, e.1).unwrap();
        dot::LabelText::LabelStr(Cow::Borrowed(label))
    }
}


impl<'a, T> dot::GraphWalk<'a, Nd, Ed<'a>> for DotGraph<'a, T> {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let nodes = self.graph.vertices().cloned().collect();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        self.graph.edges()
            .map(|e| (e.1, e.0))
            .collect()
    }

    fn source(&self, e: &Ed) -> Nd {
        *e.0
    }

    fn target(&self, e: &Ed) -> Nd {
        *e.1
    }
}

