//! Block Body

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBody {
    transactions: String,
}

impl BlockBody {
    pub fn get_transactions(&self) -> String {
        self.transactions.clone()
    }

    pub fn new(transactions: String) -> BlockBody {
        BlockBody { transactions }
    }
}
