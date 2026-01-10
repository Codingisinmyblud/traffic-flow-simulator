use super::engine::SimulationEngine;
use super::event::{Event, EventType};

pub fn process_event(engine: &mut SimulationEngine, event: Event) {
    match event.event_type {
        EventType::TaskAssigned { robot_id, target: _ } => {
            engine.schedule_event(Event {
                time: engine.time,
                event_type: EventType::ReplanRequested { robot_id },
            });
        }
        EventType::ReplanRequested { robot_id: _ } => {
            // Omitted complex replanning logic for now, in a real system:
            // 1. A* from current to target
            // 2. Schedule MoveStart
        }
        EventType::MoveStart { robot_id, from: _, to } => {
            let next_time = engine.time + 10; 
            engine.schedule_event(Event {
                time: next_time,
                event_type: EventType::MoveComplete {
                    robot_id,
                    at: to,
                },
            });
        }
        EventType::MoveComplete { robot_id, at } => {
            if let Some(robot) = engine.robots.get_mut(&robot_id) {
                robot.current_node = at;
                robot.state = crate::agents::RobotState::Idle;
            }
        }
        EventType::ConflictDetected { robot_ids: _ } => {
            // Implement resolution logic
        }
    }
}
