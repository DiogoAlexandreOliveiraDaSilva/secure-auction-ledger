use std::time::{SystemTime, UNIX_EPOCH};

use ring::digest::{Context, SHA256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: u32,
    pub auction_id: u32,
    pub bidder_id: Vec<u8>,
    pub amount: f64,
    pub timestamp: u64,
}

impl Bid {
    pub fn default() -> Self {
        Bid {
            id: 0,
            auction_id: 0,
            bidder_id: Vec::new(),
            amount: 0.0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }

    pub fn new(id: u32, auction_id: u32, bidder_id: Vec<u8>, amount: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Bid {
            id,
            auction_id,
            bidder_id,
            amount,
            timestamp,
        }
    }

    pub fn serialized(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn deserialized(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }

    pub fn get_hash(&self) -> Vec<u8> {
        let mut context = Context::new(&SHA256);
        context.update(self.serialized().as_bytes());
        context.finish().as_ref().to_vec()
    }
}
