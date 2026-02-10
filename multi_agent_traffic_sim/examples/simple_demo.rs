use multi_agent_traffic_sim::map::{Graph, Node, Edge};
use multi_agent_traffic_sim::simulation::SimulationEngine;

fn main() {
    env_logger::init();
    println!("=== Simple Traffic Simulator Demo ===");
    
    let mut graph = Graph::new();
    graph.add_node(Node::new(0, 0.0, 0.0));
    graph.add_node(Node::new(1, 10.0, 0.0));
    graph.add_edge(Edge::new(0, 1, 10.0, 1));
    
    let mut engine = SimulationEngine::new(graph);
    
    println!("Running engine...");
    engine.run(100);
    
    println!("Completed engine simulation.");
}
