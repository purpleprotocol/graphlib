use graphlib::Graph;

pub fn main() {
    // This example requires the `dot` feature.
    use std::fs::File;
    let mut f = File::create("example1.dot").unwrap();

    let mut graph: Graph<String> = Graph::new();

    let v1 = graph.add_vertex("test1".to_string());
    let v2 = graph.add_vertex("test2".to_string());
    let v3 = graph.add_vertex("test3".to_string());
    let v4 = graph.add_vertex("test4".to_string());
    let v5 = graph.add_vertex("test5".to_string());
    let v6 = graph.add_vertex("test6".to_string());

    #[cfg(feature = "dot")]
    {
        graph.add_vertex_label(&v1, "label: test1").unwrap();
        graph.add_vertex_label(&v2, "label: test2").unwrap();
        graph.add_vertex_label(&v3, "label: test3").unwrap();
        graph.add_vertex_label(&v4, "label: test4").unwrap();
        graph.add_vertex_label(&v5, "label: test5").unwrap();
        graph.add_vertex_label(&v6, "label: test6").unwrap();
    }

    graph.add_edge(&v1, &v2).unwrap();
    graph.add_edge(&v3, &v1).unwrap();
    graph.add_edge(&v1, &v4).unwrap();
    graph.add_edge(&v5, &v6).unwrap();

    #[cfg(feature = "dot")]
    {
        graph.add_edge_label(&v1, &v2, "V1&rarr;V2").unwrap();
        graph.add_edge_label(&v3, &v1, "V3&rarr;V1").unwrap();
        graph.add_edge_label(&v1, &v4, "V1&rarr;V4").unwrap();
        graph.add_edge_label(&v5, &v6, "V5&rarr;V6").unwrap();
    }

    #[cfg(feature = "dot")]
    graph.to_dot("example1", &mut f).unwrap();
}
