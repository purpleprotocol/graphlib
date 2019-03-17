use std::io::Write;
use std::borrow::Cow;

use crate::graph::Graph;
use std::fmt::Display;

/*
   Bounds on types throw warnings
   type Nd<D: Display + Clone + Ord> = D;
   type Ed<D: Display + Clone + Ord> = (D, D);
   */
type Nd<D> = D;
type Ed<D> = (D, D);

struct Edges<D: Display + Clone>(Vec<Ed<D>>);

/// Creates a file with the dot representation of the graph.
/// This method requires the `graphviz` feature.
///
/// ## Example
/// ```rust
/// use graphlib::Graph;
/// use graphlib::dot::render_to;
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
///  render_to(&graph, &mut f);
/// ```
pub fn render_to(graph: &Graph<impl Display + Clone + Ord>, output: &mut impl Write) {
    let vertices = graph.hashmap_vertices();
    let edges : Vec<(_, _)> = graph.edges().unwrap().iter().map(|w| {
        let inbound = w.inbound();
        let outbound = w.outbound();

        (vertices.get(inbound).unwrap().0.clone(), vertices.get(outbound).unwrap().0.clone())
    }).collect();

    dot::render(&Edges(edges), output).unwrap()
}

impl<'a, D: Display + Clone + Ord> dot::Labeller<'a, Nd<D>, Ed<D>> for Edges<D> {
    //TODO make it possible to rename Id
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example1").unwrap() }

    fn node_id(&'a self, n: &Nd<D>) -> dot::Id<'a> {
        dot::Id::new(format!("{}", *n)).unwrap()
    }
}

impl <'a, D: Display + Clone + Ord> dot::GraphWalk<'a, Nd<D>, Ed<D>> for Edges<D> {
    fn nodes(&self) -> dot::Nodes<'a, Nd<D>> {
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());

        for (s, t) in v.iter() {
            nodes.push(s.clone()); nodes.push(t.clone());
        }

        nodes.sort();
        nodes.dedup();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a,Ed<D>> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Ed<D>) -> Nd<D> { e.0.clone() }
    fn target(&self, e: &Ed<D>) -> Nd<D> { e.1.clone() }
}
