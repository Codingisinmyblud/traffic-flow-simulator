use crate::agents::RobotId;
use std::collections::HashMap;

// A simple Wait-For Graph (WFG) implementation using adjacency lis
#[derive(Default)]
pub struct WaitForGraph {
    pub waits_for: HashMap<RobotId, Vec<RobotId>>,
}

impl WaitForGraph {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_wait(&mut self, waiter: RobotId, waitee: RobotId) {
        self.waits_for.entry(waiter).or_insert_with(Vec::new).push(waitee);
    }
    
    pub fn remove_wait(&mut self, waiter: RobotId, waitee: RobotId) {
        if let Some(list) = self.waits_for.get_mut(&waiter) {
            list.retain(|&id| id != waitee);
        }
    }
    
    pub fn detect_deadlock(&self) -> Option<Vec<RobotId>> {
        for &start_node in self.waits_for.keys() {
            let mut visited = Vec::new();
            if self.dfs_cycle(start_node, &mut visited) {
                return Some(visited);
            }
        }
        None
    }
    
    fn dfs_cycle(&self, current: RobotId, visited: &mut Vec<RobotId>) -> bool {
        if visited.contains(&current) {
            return true;
        }
        visited.push(current);
        
        if let Some(neighbors) = self.waits_for.get(&current) {
            for &neighbor in neighbors {
                if self.dfs_cycle(neighbor, &mut visited.clone()) {
                    return true;
                }
            }
        }
        
        visited.pop();
        false
    }
}
