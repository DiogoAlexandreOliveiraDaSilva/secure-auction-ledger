use crate::kademlia::routing_table::RoutingTable;
use crate::kademlia::routing_table::node::Node;

#[test]
fn test_print_routing_table_output() {

    //print routing table of a node
    let mut rt = RoutingTable::new("127.0.0.1".to_string(), 8080);
    
    let node1 = Node::new("192.168.1.1".to_string(), 8081);
    let node2 = Node::new("127.0.0.2".to_string(), 8082);
    let node3 = Node::new("127.0.0.3".to_string(), 8083);
    
    rt.add_node(node1.clone());
    rt.add_node(node2.clone());
    rt.add_node(node3.clone());

    let output = rt.print();
    println!("Routing table output:\n{}", output); // visible with `-- --nocapture`

    // Basic structural checks
    assert!(output.contains("Routing Table:"), "Missing routing table header");
    assert!(output.contains("Bucket"), "Missing bucket labels");

    // Check for each node's presence
    for node in &[node1, node2, node3] {
        let expected_substr = format!("ip: {}, port: {}", node.get_ip(), node.get_port());
        assert!(
            output.contains(&expected_substr),
            "Output missing node: {}",
            expected_substr
        );
    }
}
