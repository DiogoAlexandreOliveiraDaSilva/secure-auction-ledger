pub mod blockchain;

use crate::blockchain::block::block::Block;
use crate::blockchain::block::block_header::BlockHeader;
use crate::blockchain::block::block_body::BlockBody;

fn main() {
    let difficulty = 2;

    let mut block_header = BlockHeader::new(
        "0000000000000000000000000000000000000000000000000000000000000000".to_string(), 
        difficulty
    );

    let block_body = BlockBody::new(vec![]);
    let mut genesis_block = Block::new(block_header, block_body);

    println!("Mining genesis block...");
    let _genesis_nonce = genesis_block.mine();

    block_header = BlockHeader::new(genesis_block.get_hash(), difficulty);
    
    let block_body = BlockBody::new(vec![]);

    let mut block = Block::new(block_header, block_body);

    println!("Mining block...");
    let _nonce = block.mine();

    if block.is_valid() {
        println!("Block is valid!");
    } else {
        println!("Block is invalid!");
    }
}