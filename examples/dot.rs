use graphlib::Graph;
use graphlib::dot::render_to;

pub fn main() {
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();

    let mut graph: Graph<isize> = Graph::new();

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

    render_to(&graph, &mut f);
}
