use multi_agent_traffic_sim::map::{Graph, Node, Edge};
use multi_agent_traffic_sim::planning::astar::find_path_astar;

#[test]
fn test_astar_simple_path() {
    let mut graph = Graph::new();
    graph.add_node(Node::new(1, 0.0, 0.0));
    graph.add_node(Node::new(2, 10.0, 0.0));
    graph.add_node(Node::new(3, 20.0, 0.0));

    graph.add_edge(Edge::new(1, 2, 10.0, 1));
    graph.add_edge(Edge::new(2, 3, 10.0, 1));

    let path = find_path_astar(&graph, 1, 3);
    assert_eq!(path, Some(vec![1, 2, 3]));
}

#[test]
fn test_astar_no_path() {
    let mut graph = Graph::new();
    graph.add_node(Node::new(1, 0.0, 0.0));
    graph.add_node(Node::new(2, 10.0, 0.0));

    let path = find_path_astar(&graph, 1, 2);
    assert_eq!(path, None); // Disconnected
}
