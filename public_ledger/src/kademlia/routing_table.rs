use std::collections::HashMap;

pub(crate) mod node;
pub(crate) mod k_bucket;
pub(crate) mod params;
pub(crate) mod node_id;

// The routing table is a list of node that joined the network by contacting the bootstrap node in this case curr_node the local node
pub(crate) struct RoutingTable{
    // The current node
    curr_node: node::Node,
    // This must be a vector of K-Buckets
    k_bucket_map: HashMap<u8, k_bucket::K_Bucket>,
    // Local Storage
    local_storage: HashMap<[u8; 20], Vec<u8>>,
}

impl RoutingTable {
    // Constructor - Creates a new Routing Table wich means a new node and an empty vector table
    pub fn new(ip: String, port: u16) -> RoutingTable {
        let curr_node = node::Node::new(ip, port);
        RoutingTable {
            curr_node,
            k_bucket_map: HashMap::new(),
            local_storage: HashMap::new(),
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

        // Check if it's adding itself
        if index == 160 {
            return;
        }

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

    pub fn get_k_bucket_map(&self) -> &HashMap<u8, k_bucket::K_Bucket> {
        &self.k_bucket_map
    }

    pub fn get_all_nodes(&self) -> Vec<node::Node> {
        let mut all_nodes = Vec::new();
        for bucket in self.k_bucket_map.values() {
            all_nodes.extend(bucket.get_nodes().iter().cloned());
        }
        all_nodes
    }

    pub fn get_closest_k_nodes(&self, id: &[u8; 20], k: usize) -> Vec<node::Node> {
        // Get all nodes from the routing table
        let all_nodes_from_buckets = self.get_all_nodes();

        //Sort the nodes by distance to the given id
        let mut all_nodes = all_nodes_from_buckets;
        all_nodes.sort_by(|a, b| {
            let dist_a = node_id::distance(id, a.get_id());
            let dist_b = node_id::distance(id, b.get_id());
            dist_a.cmp(&dist_b)
        });

        // Return the closest k nodes
        if all_nodes.len() > k {
            all_nodes.truncate(k);
        }

        // Return the closest k nodes
        all_nodes
    }

    // Store a value in the local storage
    pub fn store(&mut self, key: [u8; 20], value: Vec<u8>) {
        self.local_storage.insert(key, value);
    }

    // Get a value from the local storage
    pub fn get(&self, key: [u8; 20]) -> Option<&Vec<u8>> {
        self.local_storage.get(&key)
    }

}