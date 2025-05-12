use crate::blockchain::chain::Chain;
use serde::{Deserialize, Serialize};

use super::{Auction, bid::Bid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuctionSignature {
    pub auction_id: String,
    pub auction_hash: Vec<u8>,
}

impl AuctionSignature {
    pub fn new(auction_id: String, auction_hash: Vec<u8>) -> Self {
        AuctionSignature {
            auction_id,
            auction_hash,
        }
    }

    pub fn deserialized_from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    pub fn serialized_to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn get_signatures(chain: &Chain) -> Vec<AuctionSignature> {
        chain
            .get_blocks()
            .iter()
            .filter_map(|block| {
                AuctionSignature::deserialized_from_bytes(block.get_transactions()).ok()
            })
            .collect()
    }

    pub fn verify_auctions(
        signatures: Vec<AuctionSignature>,
        auctions: Vec<Auction>,
    ) -> Vec<Auction> {
        let mut verified_auctions = Vec::new();
        for mut auction in auctions {
            for signature in &signatures {
                if auction.id.to_string() == signature.auction_id
                    && auction.get_hash() == signature.auction_hash
                {
                    verified_auctions.push(auction.clone());
                    break;
                }
            }
        }
        verified_auctions
    }
}

// Bid Signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BidSignature {
    pub bid_id: String,
    pub bid_hash: Vec<u8>,
}

impl BidSignature {
    pub fn new(bid_id: String, bid_hash: Vec<u8>) -> Self {
        BidSignature { bid_id, bid_hash }
    }

    pub fn deserialized_from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    pub fn serialized_to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn get_signatures(chain: &Chain) -> Vec<BidSignature> {
        chain
            .get_blocks()
            .iter()
            .filter_map(|block| {
                BidSignature::deserialized_from_bytes(block.get_transactions()).ok()
            })
            .collect()
    }

    pub fn verify_bids(
        signatures: Vec<BidSignature>,
        bids: Vec<Bid>,
        auction: Auction,
    ) -> Vec<Bid> {
        let mut verified_bids: Vec<Bid> = Vec::new();

        // Sort bids by timestamp
        let mut sorted_bids = bids;
        sorted_bids.sort_by_key(|b| b.timestamp);

        let mut highest_bid = 0.0; // Track the highest valid bid so far

        for bid in sorted_bids {
            if bid.timestamp > auction.ending_time {
                continue; // Skip bids that are after the auction end time
            }

            // If bid is smaller or equal to the highest valid bid seen so far, invalidate
            if bid.amount <= highest_bid {
                continue; // Skip this bid, it's not valid
            }

            // Check if the bid's signature is valid
            let mut is_valid = false;
            for signature in &signatures {
                if bid.id.to_string() == signature.bid_id && bid.get_hash() == signature.bid_hash {
                    is_valid = true;
                    break;
                }
            }

            if is_valid {
                // Add the bid to the valid list and update the highest bid
                verified_bids.push(bid.clone());
                highest_bid = bid.amount;
            }
        }

        verified_bids
    }

    pub fn winning_bid(verified_bids: Vec<Bid>) -> Option<Bid> {
        verified_bids
            .into_iter()
            .max_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap())
    }
}
