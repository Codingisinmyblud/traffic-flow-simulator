use crate::agents::RobotId;

pub fn calculate_priority(robot_id: RobotId, wait_time: u64) -> u64 {
    // Arbitrary priority function
    (wait_time * 10) + (robot_id as u64 % 5)
}
