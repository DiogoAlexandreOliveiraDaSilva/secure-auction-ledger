use super::block::Block;

pub(crate) struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    // Creates a new chain with the genesis block
    pub fn new() -> Chain {
        Chain { blocks: vec![] }
    }

    // Adds a block to the chain
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    // Returns the last block in the chain
    pub fn get_last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    // Returns the entire chain
    pub fn get_chain(&self) -> &Vec<Block> {
        &self.blocks
    }
}
