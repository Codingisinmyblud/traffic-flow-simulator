use crate::map::NodeId;
use crate::planning::Time;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    NodeConflict { node: NodeId, time: Time },
    EdgeConflict { from: NodeId, to: NodeId, time: Time },
}

// Logic to check if there's a conflict between two planned paths
pub fn detect_conflict(path_a: &[(NodeId, Time)], path_b: &[(NodeId, Time)]) -> Option<ConflictType> {
    for &(node_a, time_a) in path_a {
        for &(node_b, time_b) in path_b {
            if node_a == node_b && time_a == time_b {
                return Some(ConflictType::NodeConflict { node: node_a, time: time_a });
            }
        }
    }
    
    // Check edge conflicts (head-on collision or same directional collision overlapping)
    for i in 0..path_a.len().saturating_sub(1) {
        let (u_a, t_a_start) = path_a[i];
        let (v_a, t_a_end) = path_a[i+1];
        
        for j in 0..path_b.len().saturating_sub(1) {
            let (u_b, t_b_start) = path_b[j];
            let (v_b, t_b_end) = path_b[j+1];
            
            // Check Head-on
            if u_a == v_b && v_a == u_b {
                if std::cmp::max(t_a_start, t_b_start) < std::cmp::min(t_a_end, t_b_end) {
                    return Some(ConflictType::EdgeConflict { from: u_a, to: v_a, time: std::cmp::max(t_a_start, t_b_start) });
                }
            }

            // Check Same direction overlap
            if u_a == u_b && v_a == v_b {
                if std::cmp::max(t_a_start, t_b_start) < std::cmp::min(t_a_end, t_b_end) {
                    return Some(ConflictType::EdgeConflict { from: u_a, to: v_a, time: std::cmp::max(t_a_start, t_b_start) });
                }
            }
        }
    }
    
    None
}
