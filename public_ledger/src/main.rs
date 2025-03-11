mod blockchain;
#[cfg(test)]
mod tests;

fn main() {
    let difficulty = 2;

    let mut block_header = blockchain::block::block_header::BlockHeader::new(
        "0000000000000000000000000000000000000000000000000000000000000000".to_string(), 
        0,
        difficulty
    );

    let block_body = blockchain::block::block_body::BlockBody::new("Genesis block".to_string());
    let mut genesis_block = blockchain::block::Block::new(block_header, block_body);

    println!("Mining genesis block...");
    let genesis_nonce = genesis_block.mine();

    block_header = blockchain::block::block_header::BlockHeader::new(
        genesis_block.get_hash(),
        genesis_nonce,
        difficulty
    );

    let block_body = blockchain::block::block_body::BlockBody::new("Hello, world!".to_string());
    let mut block = blockchain::block::Block::new(block_header, block_body);

    println!("Mining block...");
    let nonce = block.mine();

    if block.is_valid() {
        println!("Block is valid!");
    } else {
        println!("Block is invalid!");
    }
}
