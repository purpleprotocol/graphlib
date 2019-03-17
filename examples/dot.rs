use graphlib::Graph;
use graphlib::dot::render_to;

pub fn main() {
    // This example requires the `graphviz` feature.
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();

    let mut graph: Graph<String> = Graph::new();

    let v1 = graph.add_vertex("test1".to_string());
    let v2 = graph.add_vertex("test2".to_string());
    let v3 = graph.add_vertex("test3".to_string());
    let v4 = graph.add_vertex("test4".to_string());

    let v5 = graph.add_vertex("test5".to_string());
    let v6 = graph.add_vertex("test6".to_string());

    graph.add_edge(&v1, &v2).unwrap();
    graph.add_edge(&v3, &v1).unwrap();
    graph.add_edge(&v1, &v4).unwrap();
    graph.add_edge(&v5, &v6).unwrap();

    render_to(&graph, &mut f);
}
