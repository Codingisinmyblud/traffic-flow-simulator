use crate::map::NodeId;
use crate::planning::ReservationTable;

pub struct Scheduler {
    pub reservation_table: ReservationTable,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            reservation_table: ReservationTable::new(),
        }
    }

    // Attempt to schedule. Returns true if successful.
    pub fn schedule_path(&mut self, path: &[NodeId], start_time: u64) -> bool {
        let mut t = start_time;
        
        // First pass: check validity
        for i in 0..path.len() {
            let node = path[i];
            let next_t = t + 10; 
            
            if !self.reservation_table.is_node_free(node, t, next_t) {
                return false;
            }
            if i < path.len() - 1 {
                let next_node = path[i+1];
                if !self.reservation_table.is_edge_free(node, next_node, t, next_t) {
                    return false;
                }
            }
            t = next_t;
        }
        
        // Second pass: actually reserve
        t = start_time;
        for i in 0..path.len() {
            let node = path[i];
            let next_t = t + 10;
            self.reservation_table.reserve_node(node, t, next_t);
            if i < path.len() - 1 {
                 let next_node = path[i+1];
                 self.reservation_table.reserve_edge(node, next_node, t, next_t);
            }
            t = next_t;
        }
        
        true
    }
}
