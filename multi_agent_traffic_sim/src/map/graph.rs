use std::collections::HashMap;
use super::node::{Node, NodeId};
use super::edge::Edge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<NodeId, Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
    
    pub fn get_neighbors(&self, node_id: NodeId) -> Vec<NodeId> {
        let mut neighbors = Vec::new();
        for edge in &self.edges {
            if edge.from == node_id {
                neighbors.push(edge.to);
            }
        }
        neighbors
    }

    pub fn get_edge(&self, from: NodeId, to: NodeId) -> Option<&Edge> {
        for edge in &self.edges {
            if edge.from == from && edge.to == to {
                return Some(edge);
            }
        }
        None
    }
}
