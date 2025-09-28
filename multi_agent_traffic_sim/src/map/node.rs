use serde::{Deserialize, Serialize};

pub type NodeId = usize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub x: f64,
    pub y: f64,
}

impl Node {
    pub fn new(id: NodeId, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }
}
