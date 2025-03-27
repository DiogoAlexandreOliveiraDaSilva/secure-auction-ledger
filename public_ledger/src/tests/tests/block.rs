// Test block mining
// This test checks if the block is mined correctly.
#[test]
fn test_block_mining() {
    use crate::blockchain::block::block_header::BlockHeader;
    use crate::blockchain::block::block_body::BlockBody;
    use crate::blockchain::block::Block;

    let header = BlockHeader::new("parent_hash".to_string(), 1);
    let body = BlockBody::new("transactions".to_string());
    let mut block = Block::new(header, body);
    block.mine();
    assert_eq!(block.is_valid(), true);
}