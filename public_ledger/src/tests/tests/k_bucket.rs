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


#[test]
fn test_get_closest_k_nodes_returns_closest() {
    use crate::kademlia::routing_table::{RoutingTable, node::Node};
    use crate::kademlia::routing_table::node_id;

    let mut rt = RoutingTable::new("127.0.0.1".to_string(), 8080); // Assuming your RoutingTable implements k_bucket_map internally

    // Manually create nodes with known IDs
    let target_id: [u8; 20] = [0b00000000; 20];

    // Closer in XOR space
    let node1 = Node::with_id([0b00000001; 20], "127.0.0.1".to_string(), 1); // XOR distance: 1
    let node2 = Node::with_id([0b00000100; 20], "127.0.0.1".to_string(), 2); // XOR distance: 4
    let node3 = Node::with_id([0b00000010; 20], "127.0.0.1".to_string(), 3); // XOR distance: 2
    let node4 = Node::with_id([0b00010000; 20], "127.0.0.1".to_string(), 4); // XOR distance: 16

    // Add to buckets (assumes auto-bucketing)
    rt.add_node(node1.clone());
    rt.add_node(node2.clone());
    rt.add_node(node3.clone());
    rt.add_node(node4.clone());

    // Ask for the 3 closest nodes
    let closest = rt.get_closest_k_nodes(&target_id, 3);

    // Expected sorted order by XOR distance: node1 (1), node3 (2), node5 (3)
    let expected_ids = vec![
        node1.get_id(),
        node3.get_id(),
        node2.get_id(),];

    let result_ids: Vec<&[u8; 20]> = closest.iter().map(|n| n.get_id()).collect();

    for i in rt.get_all_nodes().iter() {
        println!("Node ID: {:?}", i.get_id());
    }

    assert_eq!(result_ids, expected_ids);    
}
