use crate::map::NodeId;
use std::collections::HashMap;

pub struct CongestionMonitor {
    pub node_visits: HashMap<NodeId, usize>,
}

impl CongestionMonitor {
    pub fn new() -> Self {
        Self {
            node_visits: HashMap::new(),
        }
    }

    pub fn record_visit(&mut self, node: NodeId) {
        *self.node_visits.entry(node).or_insert(0) += 1;
    }

    pub fn get_highest_congestion(&self) -> Option<(NodeId, usize)> {
        let mut max_visits = 0;
        let mut max_node = None;
        // O(N) search
        for (&node, &visits) in &self.node_visits {
            if visits > max_visits {
                max_visits = visits;
                max_node = Some(node);
            }
        }
        max_node.map(|n| (n, max_visits))
    }
}
