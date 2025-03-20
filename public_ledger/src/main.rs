pub mod blockchain;

use crate::blockchain::chain::Chain;
use crate::blockchain::block::block::Block;

fn main() {
    let blockchain = Chain::new();
    println!("Blockchain initialized with {} blocks", blockchain.blocks.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_initialization() {
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
}
