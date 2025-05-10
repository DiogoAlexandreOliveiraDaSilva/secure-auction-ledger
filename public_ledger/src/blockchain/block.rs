//! Block structure
pub(crate) mod block_body;
pub(crate) mod block_header;
use ring::digest;
use serde::{Deserialize, Serialize};

// A block is a structure that contains a header and a body
// The header contains metadata about the block
// The body contains the transactions

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Block {
    pub header: block_header::BlockHeader,
    pub body: block_body::BlockBody,
}

impl Block {
    // Creates block hash based on it's information
    pub fn get_hash(&self) -> Vec<u8> {
        let mut hash = digest::Context::new(&digest::SHA512);
        hash.update(&self.header.get_parent_hash());
        hash.update(self.header.get_timestamp().to_string().as_bytes());
        hash.update(self.body.get_transactions());
        hash.update(self.header.get_nonce().to_string().as_bytes());
        hash.update(self.header.get_difficulty().to_string().as_bytes());
        let digest_result = hash.finish();

        let mut hash_vec = Vec::new();
        for byte in digest_result.as_ref() {
            hash_vec.push(*byte);
        }
        hash_vec
    }

    // Returns the block's nonce
    pub fn get_nonce(&self) -> u64 {
        self.header.get_nonce()
    }

    // Increments the block's nonce until the block is valid (hash starts with zeros corresponding to the difficulty)
    pub fn mine(&mut self) -> u64 {
        loop {
            if self.is_valid() {
                println!(
                    "Block mined with nonce: {}, hash: {}",
                    self.header.get_nonce(),
                    hex::encode(self.get_hash())
                );
                return self.header.get_nonce();
            }

            self.header.set_nonce(self.header.get_nonce() + 1);
        }
    }

    // Checks if the block is valid (hash starts with zeros corresponding to the difficulty)
    pub fn is_valid(&self) -> bool {
        let hash = self.get_hash();
        let target = "0".repeat(self.header.get_difficulty() as usize);
        let hash_str = hex::encode(hash);
        hash_str.starts_with(&target)
    }

    pub fn new(header: block_header::BlockHeader, body: block_body::BlockBody) -> Block {
        Block { header, body }
    }

    pub fn genesis() -> Block {
        Block {
            header: block_header::BlockHeader::genesis(),
            body: block_body::BlockBody::new(vec![]),
        }
    }

    pub fn deserialized(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }

    pub fn serialized(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn get_transactions(&self) -> &Vec<u8> {
        self.body.get_transactions()
    }
}
