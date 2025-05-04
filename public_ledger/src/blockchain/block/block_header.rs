//!Block Header

use serde::{Deserialize, Serialize};

// A block header contains metadata about the block

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    // prev_hash is the hash of the previous block in the chain
    prev_hash: String,
    // nonce is a number that miners increment in order to find a valid hash
    nonce: u64,
    // difficulty is the number of zeros that the hash of the block should start with for it to be valid
    difficulty: u64,
    // timestamp is the time at which the block was created
    timestamp: u64,
}

impl BlockHeader {
    pub fn get_parent_hash(&self) -> String {
        self.prev_hash.clone()
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

    pub fn new(prev_hash: String, difficulty: u64) -> BlockHeader {
        BlockHeader {
            prev_hash,
            nonce: 0,
            difficulty,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn genesis() -> BlockHeader {
        BlockHeader {
            prev_hash: "0".to_string(),
            nonce: 0,
            difficulty: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
