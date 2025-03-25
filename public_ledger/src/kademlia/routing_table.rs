use std::collections::HashMap;

pub(crate) mod node;
pub(crate) mod k_bucket;
mod params;
pub(crate) mod node_id;

// The routing table is a list of node that joined the network by contacting the bootstrap node in this case curr_node the local node
pub(crate) struct RoutingTable{
    // The current node
    curr_node: node::Node,
    // This must be a vector of K-Buckets
    k_bucket_map: HashMap<u8, k_bucket::K_Bucket>,
}

impl RoutingTable {
    // Constructor - Creates a new Routing Table wich means a new node and an empty vector table
    pub fn new(ip: String, port: u16) -> RoutingTable {
        let curr_node = node::Node::new(ip, port);
        RoutingTable {
            curr_node,
            k_bucket_map: HashMap::new(),
        }
    }

    // Get the current node
    pub fn get_curr_node(&self) -> &node::Node {
        &self.curr_node
    }

    // Add a node to the routing table
    pub fn add_node(&mut self, node: node::Node) {
        // Get the distance between the current node and the node to add
        let distance = node_id::distance(self.get_curr_node().get_id(), node.get_id());
        // Get the index of the bucket in the table
        let index = node_id::find_k_bucket_index(&distance);
        // Check if the bucket exists
        if !self.k_bucket_map.contains_key(&(index as u8)) {
            // Create a new bucket
            self.k_bucket_map.insert(index as u8, k_bucket::K_Bucket::new(params::MAX_BUCKET_SIZE));
        }
        // Add the node to the bucket
        self.k_bucket_map.get_mut(&(index as u8)).unwrap().add_node(node);
    }

    // Get the number of nodes in the routing table
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        for bucket in self.k_bucket_map.values() {
            size += bucket.get_size();
        }
        size
    }
}