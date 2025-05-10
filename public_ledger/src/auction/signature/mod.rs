use crate::blockchain::chain::Chain;
use serde::{Deserialize, Serialize};

use super::Auction;

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
