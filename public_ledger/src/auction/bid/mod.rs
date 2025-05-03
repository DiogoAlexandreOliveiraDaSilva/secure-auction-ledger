use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: u32,
    pub auction_id: u32,
    pub bidder_id: u32,
    pub amount: f64,
    pub timestamp: u64,
}
