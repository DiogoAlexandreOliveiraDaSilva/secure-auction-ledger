use public_ledger::blockchain::chain::Chain;
use public_ledger::blockchain::block::block::Block;

#[test]
fn test_create_blockchain() {
    let blockchain = Chain::new();
    assert_eq!(blockchain.blocks.len(), 1, "Blockchain should start with a Genesis block");
}

#[test]
fn test_add_block() {
    let mut blockchain = Chain::new();
    let new_block = Block::genesis();
    blockchain.add_block(new_block);

    assert_eq!(blockchain.blocks.len(), 2, "Blockchain should have 2 blocks after adding one");
}
