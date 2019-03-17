use std::io::Write;
use std::borrow::Cow;

use crate::vertex_id::VertexId;
use crate::graph::Graph;
use std::sync::Arc;
use std::fmt::Debug;

type Nd = isize;
type Ed= (isize, isize);
struct Edges(Vec<Ed>);

pub fn render_to(graph: &Graph<isize>, output: &mut impl Write) {
    let vertices = graph.hashmap_vertices();
    let edges : Vec<(_, _)> = graph.edges().unwrap().iter().map(|w| {
        let inbound = w.inbound();
        let outbound = w.outbound();

        (vertices.get(inbound).unwrap().0, vertices.get(outbound).unwrap().0.clone())
    }).collect();

    dot::render(&Edges(edges), output).unwrap()
}

//TODO add `render_to` function with trait
/*
   pub fn render_to(graph: &Graph<impl std::fmt::Debug>, output: &mut impl Write) {
//let edges = Edges(vec!((0,1), (0,2), (1,3), (2,3), (3,4), (4,4)));
//dot::render(&edges, output).unwrap()

unimplemented!()
}
*/

impl<'a> dot::Labeller<'a, Nd, Ed> for Edges {
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example1").unwrap() }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{:?}", *n)).unwrap()
    }
}

impl <'a> dot::GraphWalk<'a, Nd, Ed> for Edges {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());

        for &(s, t) in v {
            nodes.push(s); nodes.push(t);
        }

        nodes.sort();
        nodes.dedup();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a,Ed> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Ed) -> Nd { e.0 }
    fn target(&self, e: &Ed) -> Nd { e.1 }
}
