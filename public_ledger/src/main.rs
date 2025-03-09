mod blockchain;
#[cfg(test)]
mod tests;

fn main() {
    // Create a new block
    let block_header = blockchain::block::block_header::BlockHeader::new("teste".to_string());
    let block_body = blockchain::block::block_body::BlockBody::new("teste".to_string());
    let block = blockchain::block::Block::new(block_header, block_body);
    println!("Block hash: {}", block.get_hash());
}
