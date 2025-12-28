use crate::agents::RobotId;
use crate::map::NodeId;
use crate::planning::Time;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    MoveStart { robot_id: RobotId, from: NodeId, to: NodeId },
    MoveComplete { robot_id: RobotId, at: NodeId },
    ConflictDetected { robot_ids: Vec<RobotId> },
    ReplanRequested { robot_id: RobotId },
    TaskAssigned { robot_id: RobotId, target: NodeId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub time: Time,
    pub event_type: EventType,
}


impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
