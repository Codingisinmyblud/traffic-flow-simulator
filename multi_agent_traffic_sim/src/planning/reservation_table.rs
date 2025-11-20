use crate::map::NodeId;
use std::collections::HashMap;

pub type Time = u64;

#[derive(Default)]
pub struct ReservationTable {
    pub node_reservations: HashMap<NodeId, Vec<(Time, Time)>>,
    pub edge_reservations: HashMap<(NodeId, NodeId), Vec<(Time, Time)>>,
}

impl ReservationTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reserve_node(&mut self, node: NodeId, start: Time, end: Time) {
        self.node_reservations.entry(node).or_insert_with(Vec::new).push((start, end));
    }

    pub fn reserve_edge(&mut self, from: NodeId, to: NodeId, start: Time, end: Time) {
        self.edge_reservations.entry((from, to)).or_insert_with(Vec::new).push((start, end));
    }

    pub fn is_node_free(&self, node: NodeId, start: Time, end: Time) -> bool {
        if let Some(reservations) = self.node_reservations.get(&node) {
            for &(rs, re) in reservations {
                if start < re && end > rs {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_edge_free(&self, from: NodeId, to: NodeId, start: Time, end: Time) -> bool {
        if let Some(reservations) = self.edge_reservations.get(&(to, from)) {
             for &(rs, re) in reservations {
                if start < re && end > rs {
                    return false;
                }
            }
        }
        
        if let Some(reservations) = self.edge_reservations.get(&(from, to)) {
            for &(rs, re) in reservations {
                if start < re && end > rs {
                    return false;
                }
            }
        }
        
        true
    }
}
