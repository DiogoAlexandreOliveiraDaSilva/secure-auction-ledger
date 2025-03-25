#[test]
fn test_k_bucket_node_order(){
    use crate::kademlia::routing_table::k_bucket::K_Bucket;
    use crate::kademlia::routing_table::node::Node;

    let mut bucket = K_Bucket::new(5);
    let node1: Node = Node::new("127.0.0.1".to_string(), 1);
    let node1_id: [u8; 20] = *node1.get_id();
    let node2 = Node::new("127.0.0.1".to_string(), 2);
    let node2_id: [u8; 20] = *node2.get_id();
    let node3 = Node::new("127.0.0.1".to_string(), 3);
    let node3_id: [u8; 20] = *node3.get_id();
    bucket.add_node(node1); 
    bucket.add_node(node2);
    bucket.add_node(node3);
    let node1_copy = Node::with_id(node1_id, "127.0.0.1".to_string(), 1);
    bucket.add_node(node1_copy);
    assert_eq!(bucket.get_nodes().len(), 3);
    assert_eq!(bucket.get_nodes()[0].get_id(), &node1_id);
    assert_eq!(bucket.get_nodes()[1].get_id(), &node3_id);
    assert_eq!(bucket.get_nodes()[2].get_id(), &node2_id);
}

