use super::state::RobotState;
use super::task::Task;
use crate::map::NodeId;
use serde::{Deserialize, Serialize};

pub type RobotId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub id: RobotId,
    pub current_node: NodeId,
    pub state: RobotState,
    pub current_task: Option<Task>,
    pub planned_path: Vec<NodeId>,
    pub velocity: f64,
}

impl Robot {
    pub fn new(id: RobotId, start_node: NodeId, velocity: f64) -> Self {
        Self {
            id,
            current_node: start_node,
            state: RobotState::Idle,
            current_task: None,
            planned_path: Vec::new(),
            velocity,
        }
    }

    pub fn assign_task(&mut self, task: Task) {
        self.current_task = Some(task);
    }
}
