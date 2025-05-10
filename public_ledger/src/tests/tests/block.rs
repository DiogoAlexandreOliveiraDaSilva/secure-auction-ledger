// Test block mining
// This test checks if the block is mined correctly.
#[test]
fn test_block_mining() {
    use crate::blockchain::block::Block;
    use crate::blockchain::block::block_body::BlockBody;
    use crate::blockchain::block::block_header::BlockHeader;

    let header = BlockHeader::new("parent_hash".as_bytes().to_vec());
    let body = BlockBody::new("transactions".as_bytes().to_vec());
    let mut block = Block::new(header, body);
    block.mine();
    assert_eq!(block.is_valid(), true);
}
