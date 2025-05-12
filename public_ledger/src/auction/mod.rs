pub(crate) mod bid;
pub(crate) mod signature;

use chrono::{DateTime, TimeZone, Utc};

use bid::Bid;
use ring::digest::{Context, SHA256};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Auction {
    pub id: u32,
    pub item_name: String,
    pub starting_price: f64,
    pub starting_time: u64,
    pub ending_time: u64,
    pub bids: Vec<Bid>,
}

impl Auction {
    pub fn new(
        id: u32,
        item_name: String,
        starting_price: f64,
        starting_time: u64,
        ending_time: u64,
    ) -> Self {
        Auction {
            id,
            item_name,
            starting_price,
            starting_time,
            ending_time,
            bids: Vec::new(),
        }
    }

    pub fn new_with_duration(
        id: u32,
        item_name: String,
        starting_price: f64,
        duration_hours: u64,
    ) -> Self {
        let starting_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let ending_time = starting_time + duration_hours * 3600;

        Auction {
            id,
            item_name,
            starting_price,
            starting_time,
            ending_time,
            bids: Vec::new(),
        }
    }

    pub fn add_bid(&mut self, bid: Bid) {
        self.bids.push(bid);
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

    pub fn finished(&self) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        current_time > self.ending_time
    }

    pub fn get_ending_time_as_string(&self) -> String {
        // Assuming `self.ending_time` is a timestamp (u64) in seconds
        let datetime = Utc.timestamp(self.ending_time as i64, 0);
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
