use super::event::Event;
use crate::agents::{Robot, RobotId};
use crate::map::Graph;
use std::collections::{BinaryHeap, HashMap};

pub struct SimulationEngine {
    pub time: u64,
    pub event_queue: BinaryHeap<Event>,
    pub robots: HashMap<RobotId, Robot>,
    pub graph: Graph,
}

impl SimulationEngine {
    pub fn new(graph: Graph) -> Self {
        Self {
            time: 0,
            event_queue: BinaryHeap::new(),
            robots: HashMap::new(),
            graph,
        }
    }

    pub fn schedule_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    pub fn run_step(&mut self) -> Option<Event> {
        if let Some(event) = self.event_queue.pop() {
            self.time = event.time;
            Some(event)
        } else {
            None
        }
    }
    
    pub fn run(&mut self, max_time: u64) {
        while self.time < max_time {
            if let Some(event) = self.run_step() {
                self.process_event(event);
            } else {
                break;
            }
        }
    }

    fn process_event(&mut self, event: Event) {
        // Here we ideally call a scheduler to process logic.
        // We'll wrap it in simulation/scheduler.rs or implement minimally here.
        super::scheduler::process_event(self, event);
    }
}
