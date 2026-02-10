use multi_agent_traffic_sim::map::{Graph, Node, Edge};
use multi_agent_traffic_sim::simulation::SimulationEngine;
use multi_agent_traffic_sim::agents::Robot;

fn main() {
    env_logger::init();
    println!("=== Stress Test ===");
    
    let mut graph = Graph::new();
    // Create a 10x10 grid network
    for x in 0..10 {
        for y in 0..10 {
            let id = y * 10 + x;
            graph.add_node(Node::new(id, x as f64 * 10.0, y as f64 * 10.0));
        }
    }
    
    // Add edges
    for x1 in 0..10 {
        for y1 in 0..10 {
            let id1 = y1 * 10 + x1;
            for x2 in 0..10 {
                for y2 in 0..10 {
                    let id2 = y2 * 10 + x2;
                    let dx = (x1 as i32 - x2 as i32).abs();
                    let dy = (y1 as i32 - y2 as i32).abs();
                    if dx + dy == 1 {
                        graph.add_edge(Edge::new(id1, id2, 10.0, 1));
                    }
                }
            }
        }
    }
    
    let mut engine = SimulationEngine::new(graph);
    
    // Add 100 robots
    for i in 0..100 {
        engine.robots.insert(i, Robot::new(i, i % 100, 1.0));
    }
    
    println!("Running simulation for 1000 ticks...");
    engine.run(1000);
    println!("Stress test concluded.");
}
