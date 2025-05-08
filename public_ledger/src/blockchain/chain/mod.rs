use super::block::Block;

#[derive(Default, Clone)]
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

    // Returns the first block in the chain (genesis block)
    pub fn get_first_block(&self) -> &Block {
        self.blocks
            .get(0)
            .unwrap_or_else(|| panic!("Chain is empty, no genesis block found."))
    }
    // Returns the entire chain
    pub fn get_chain(&self) -> &Vec<Block> {
        &self.blocks
    }
}
