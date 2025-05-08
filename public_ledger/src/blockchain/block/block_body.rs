//! Block Body

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBody {
    transactions: Vec<u8>,
}

impl BlockBody {
    pub fn get_transactions(&self) -> &Vec<u8> {
        &self.transactions
    }

    pub fn new(transactions: Vec<u8>) -> BlockBody {
        BlockBody { transactions }
    }
}
