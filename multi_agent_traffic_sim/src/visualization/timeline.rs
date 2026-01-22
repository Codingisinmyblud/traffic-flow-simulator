use crate::agents::RobotId;
use crate::map::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryEvent {
    pub time: u64,
    pub robot_id: RobotId,
    pub node: NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Timeline {
    pub events: Vec<TrajectoryEvent>,
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(&mut self, time: u64, robot_id: RobotId, node: NodeId) {
        self.events.push(TrajectoryEvent {
            time,
            robot_id,
            node,
        });
    }
}
