use crate::map::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub target: NodeId,
}

impl Task {
    pub fn new(id: usize, target: NodeId) -> Self {
        Self { id, target }
    }
}
