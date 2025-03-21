mod node;
mod k_bucket;
mod params;
pub(crate) mod node_id;

// The routing table is a list of node that joined the network by contacting the bootstrap node in this case curr_node the local node
pub(crate) struct RoutingTable{
    // The current node
    curr_node: node::Node,
    // The list of nodes that joined the network by being contacting the bootstrap node
    k_bucket: k_bucket::K_Bucket,
}

impl RoutingTable {
    // Constructor - Creates a new Routing Table wich means a new node and an empty vector table
    pub fn new(ip: String, port: u16) -> RoutingTable {
        let curr_node = node::Node::new(ip, port);
        RoutingTable {
            curr_node,
            k_bucket: k_bucket::K_Bucket::new(params::MAX_BUCKET_SIZE),
        }
    }

    // Get the current node
    pub fn get_curr_node(&self) -> &node::Node {
        &self.curr_node
    }
}