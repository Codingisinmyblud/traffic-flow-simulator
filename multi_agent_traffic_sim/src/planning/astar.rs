use crate::map::{Graph, NodeId};
use std::collections::HashMap;

pub fn find_path_astar(graph: &Graph, start: NodeId, goal: NodeId) -> Option<Vec<NodeId>> {
    let mut open_set = Vec::new();
    open_set.push(start);

    let mut came_from: HashMap<NodeId, NodeId> = HashMap::new();
    let mut g_score: HashMap<NodeId, f64> = HashMap::new();
    g_score.insert(start, 0.0);
    
    let mut f_score: HashMap<NodeId, f64> = HashMap::new();
    f_score.insert(start, heuristic(graph, start, goal));

    while !open_set.is_empty() {
        let mut current_idx = 0;
        let mut min_f = std::f64::INFINITY;
        
        for (i, &node) in open_set.iter().enumerate() {
            let f = *f_score.get(&node).unwrap_or(&std::f64::INFINITY);
            if f < min_f {
                min_f = f;
                current_idx = i;
            }
        }

        let current = open_set.remove(current_idx);

        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        for neighbor in graph.get_neighbors(current) {
            let edge = graph.get_edge(current, neighbor).unwrap();
            let tentative_g = g_score.get(&current).unwrap() + edge.weight;

            let prev_g = *g_score.get(&neighbor).unwrap_or(&std::f64::INFINITY);

            if tentative_g < prev_g {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                f_score.insert(neighbor, tentative_g + heuristic(graph, neighbor, goal));

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    None
}

fn heuristic(graph: &Graph, a: NodeId, b: NodeId) -> f64 {
    if let (Some(node_a), Some(node_b)) = (graph.nodes.get(&a), graph.nodes.get(&b)) {
        ((node_a.x - node_b.x).powi(2) + (node_a.y - node_b.y).powi(2)).sqrt()
    } else {
        0.0
    }
}

fn reconstruct_path(came_from: &HashMap<NodeId, NodeId>, mut current: NodeId) -> Vec<NodeId> {
    let mut total_path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        current = prev;
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}
