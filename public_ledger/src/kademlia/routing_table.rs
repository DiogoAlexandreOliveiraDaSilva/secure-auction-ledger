mod node;

// The routing table is a list of node that joined the network by contacting the bootstrap node in this case curr_node the local node
pub(crate) struct RoutingTable{
    // The current node
    curr_node: node::Node,
    // The list of nodes that joined the network by being contacting the bootstrap node
    k_bucket: Vec<node::Node>,
}

impl RoutingTable {
    // Constructor - Creates a new Routing Table wich means a new node and an empty vector table
    pub fn new(id: String, ip: String, port: u16) -> RoutingTable {
        let curr_node = node::Node::new(id, ip, port);
        RoutingTable {
            curr_node,
            k_bucket: Vec::new(),
        }
    }
}