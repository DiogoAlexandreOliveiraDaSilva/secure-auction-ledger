use crate::blockchain::block::block::Block;

pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain{
    pub fn new() -> Self {
        let genesis_block = Block::genesis();
        Chain{
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}