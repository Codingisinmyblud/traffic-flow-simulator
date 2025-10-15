use std::fs::File;
use std::io::BufReader;
use super::graph::Graph;

pub fn load_graph_from_json(path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let graph: Graph = serde_json::from_reader(reader)?;
    Ok(graph)
}
