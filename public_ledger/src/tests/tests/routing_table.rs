#[test]
fn test_routing_table_order() {
    use crate::kademlia::routing_table::RoutingTable;
    use crate::kademlia::routing_table::node::Node;

    let mut routing_table = RoutingTable::new("127.0.0.1".to_string(), 1);
    let node1: Node = Node::new("127.0.0.1".to_string(), 2);
    let node2: Node = Node::new("127.0.0.1".to_string(), 3);
    routing_table.add_node(node2);
    routing_table.add_node(node1);
    for (k, k_bucket) in routing_table.get_k_bucket_map() {
        for node in k_bucket.get_nodes() {
            println!("k-bucket index: {}, node: {:?}", k, node.get_id());
        }
    }
}