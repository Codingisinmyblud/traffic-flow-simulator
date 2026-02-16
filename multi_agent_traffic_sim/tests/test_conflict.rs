use multi_agent_traffic_sim::coordination::conflict::{detect_conflict, ConflictType};
use multi_agent_traffic_sim::map::NodeId;
use multi_agent_traffic_sim::planning::Time;

#[test]
fn test_node_conflict_detected() {
    let path_a: Vec<(NodeId, Time)> = vec![(1, 0), (2, 10), (3, 20)];
    let path_b: Vec<(NodeId, Time)> = vec![(4, 0), (2, 10), (5, 20)];

    let conflict = detect_conflict(&path_a, &path_b);
    assert_eq!(conflict, Some(ConflictType::NodeConflict { node: 2, time: 10 }));
}

#[test]
fn test_no_conflict() {
    let path_a: Vec<(NodeId, Time)> = vec![(1, 0), (2, 10), (3, 20)];
    let path_b: Vec<(NodeId, Time)> = vec![(4, 0), (5, 10), (6, 20)];

    let conflict = detect_conflict(&path_a, &path_b);
    assert_eq!(conflict, None);
}
