mod bid;

use bid::Bid;

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

    pub fn add_bid(&mut self, bid: Bid) {
        self.bids.push(bid);
    }
}
