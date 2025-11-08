use crate::map::{Graph, NodeId};
use std::collections::HashMap;

pub fn find_path_dijkstra(graph: &Graph, start: NodeId, goal: NodeId) -> Option<Vec<NodeId>> {
    let mut unvisited: Vec<NodeId> = graph.nodes.keys().copied().collect();
    let mut dist: HashMap<NodeId, f64> = HashMap::new();
    let mut prev: HashMap<NodeId, NodeId> = HashMap::new();

    for &id in &unvisited {
        dist.insert(id, std::f64::INFINITY);
    }
    dist.insert(start, 0.0);

    while !unvisited.is_empty() {
        let mut min_idx = 0;
        let mut min_dist = std::f64::INFINITY;
        for (i, &node) in unvisited.iter().enumerate() {
            if dist[&node] < min_dist {
                min_dist = dist[&node];
                min_idx = i;
            }
        }

        if min_dist == std::f64::INFINITY {
            break;
        }

        let u = unvisited.remove(min_idx);

        if u == goal {
            let mut path = vec![u];
            let mut curr = u;
            while let Some(&p) = prev.get(&curr) {
                curr = p;
                path.push(curr);
            }
            path.reverse();
            return Some(path);
        }

        for v in graph.get_neighbors(u) {
            if unvisited.contains(&v) {
                let weight = graph.get_edge(u, v).unwrap().weight;
                let alt = dist[&u] + weight;
                if alt < dist[&v] {
                    dist.insert(v, alt);
                    prev.insert(v, u);
                }
            }
        }
    }
    
    None
}
