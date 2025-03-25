#[test]
fn test_distance() {
    use crate::kademlia::routing_table::node_id::distance;

    let node_id1 : [u8; 20] = [0b10101010; 20];  // 20 bytes of 0b10101010
    let node_id2 : [u8; 20] = [0b01010101; 20];  // 20 bytes of 0b01010101

    // Compute XOR distance
    let dist = distance(&node_id1, &node_id2);

    // Check that the result is 20 bytes
    assert_eq!(dist.len(), 20);

    // Check XOR property: A ⊕ B should result in all 1s (0xFF)
    for &byte in &dist {
        assert_eq!(byte, 0b11111111);  // 0b10101010 ⊕ 0b01010101 = 0b11111111
    }

    // Additional check: A ⊕ A = 0
    let zero_dist = distance(&node_id1, &node_id1);
    assert_eq!(zero_dist, [0; 20]); // Should be all zeros
}

#[test]
fn test_find_k_index(){
    use crate::kademlia::routing_table::node_id::find_k_bucket_index;

    let node_id1 : [u8; 20] = [0b10101010; 20];  // 20 bytes of 0b10101010
    let node_id2 : [u8; 20] = [0b01010101; 20];  // 20 bytes of 0b01010101

    // Compute XOR distance
    let dist = crate::kademlia::routing_table::node_id::distance(&node_id1, &node_id2);

    // Check index of first '1' bit
    let k_bucket_index = find_k_bucket_index(&dist);
    assert_eq!(k_bucket_index, 0);  // First bit is 1 because 0b10101010 ⊕ 0b01010101 = 0b11111111
}
