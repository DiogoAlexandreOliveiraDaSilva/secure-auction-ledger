//!Block Header



pub struct BlockHeader {
    parent_hash: String,
    timestamp: u64
}

impl BlockHeader {
    pub fn get_parent_hash(&self) -> String {
        self.parent_hash.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn new(parent_hash: String) -> BlockHeader {
        BlockHeader {
            parent_hash,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        }
    }
}