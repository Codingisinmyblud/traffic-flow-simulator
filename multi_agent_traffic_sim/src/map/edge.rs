use super::node::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: f64, // could be travel time or distance
    pub capacity: usize, 
}

impl Edge {
    pub fn new(from: NodeId, to: NodeId, weight: f64, capacity: usize) -> Self {
        Self { from, to, weight, capacity }
    }
}
