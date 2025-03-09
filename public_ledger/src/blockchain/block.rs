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
        hash.finish().as_ref().to_vec().iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn new(header: block_header::BlockHeader, body: block_body::BlockBody) -> Block {
        Block {
            header,
            body
        }
    }

}
