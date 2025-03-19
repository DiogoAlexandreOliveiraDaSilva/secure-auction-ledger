//! Block Body
use serde_json;
use crate::blockchain::transaction::transaction::Transaction;

pub struct BlockBody {
    pub transactions: Vec<Transaction>
}

impl BlockBody {
    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    
    pub fn serialize_transactions(&self) -> String {
        serde_json::to_string(&self.transactions).unwrap()
    }
    
    pub fn new(transactions: Vec<Transaction>) -> Self {
        BlockBody {
            transactions
        }
    }
}
