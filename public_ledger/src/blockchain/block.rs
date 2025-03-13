//! Block structure
pub(crate) mod block_header;
pub(crate) mod block_body;
use ring::digest;

pub(crate) struct Block {
    header: block_header::BlockHeader,
    body: block_body::BlockBody
}


impl Block {
    pub fn get_hash(&self) -> String {
        let mut hash = digest::Context::new(&digest::SHA512);
        hash.update(self.header.get_parent_hash().as_bytes());
        hash.update(self.header.get_timestamp().to_string().as_bytes());
        hash.update(self.body.get_transactions().as_bytes());
        hash.update(self.header.get_nonce().to_string().as_bytes());
        hash.update(self.header.get_difficulty().to_string().as_bytes());
        let digest_result = hash.finish();

        digest_result
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    pub fn get_nonce(&self) -> u64 {
        self.header.get_nonce()
    }

    pub fn mine(&mut self) -> u64 {
        loop {
            if self.is_valid() {
                println!("Block mined with nonce: {}, hash: {}", self.header.get_nonce(), self.get_hash());
                return self.header.get_nonce();
            }
            
            //TODO: Confirm if this is the correct way to increment the nonce
            self.header.set_nonce(self.header.get_nonce() + 1);
        }
    }

    pub fn is_valid(&self) -> bool {
        let hash = self.get_hash();
        let target = "0".repeat(self.header.get_difficulty() as usize);
        hash.starts_with(&target)
    }

    pub fn new(header: block_header::BlockHeader, body: block_body::BlockBody) -> Block {
        Block {
            header,
            body
        }
    }
}
