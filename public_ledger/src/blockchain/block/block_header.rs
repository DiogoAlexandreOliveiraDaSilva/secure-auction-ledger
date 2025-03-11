//!Block Header



pub struct BlockHeader {
    parent_hash: String,
    nonce: u64,
    difficulty: u64,
    timestamp: u64,
}

impl BlockHeader {
    pub fn get_parent_hash(&self) -> String {
        self.parent_hash.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn get_difficulty(&self) -> u64 {
        self.difficulty
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }

    pub fn new(parent_hash: String, prev_nonce: u64, difficulty:u64) -> BlockHeader {
        BlockHeader {
            parent_hash,
            nonce: prev_nonce+1,
            difficulty,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        }
    }
}