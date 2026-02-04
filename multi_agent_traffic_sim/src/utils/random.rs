use rand::Rng;
use crate::map::NodeId;

pub fn get_random_node_id(max_nodes: usize) -> NodeId {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max_nodes)
}
