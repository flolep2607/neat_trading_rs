mod graph;
use graph::main::Graph;

fn main() -> eframe::Result<()> {
    let mut graph = Graph::new();

    graph.add_vertex("A".to_string());
    graph.add_vertex("B".to_string());
    graph.add_vertex("C".to_string());
    graph.add_vertex("D".to_string());
    graph.add_vertex("E".to_string());
    graph.add_vertex("F".to_string());

    // graph.add_edge("A".to_string(), "B".to_string());
    // graph.add_edge("A".to_string(), "C".to_string());
    // graph.add_edge("B".to_string(), "C".to_string());

    let options = eframe::NativeOptions::default();
    let graph_app = graph::vizualize::GraphApp { graph };
    eframe::run_native(
        "Graph Visualization",
        options,
        Box::new(|_cc| Ok(Box::new(graph_app))),
    )
}
