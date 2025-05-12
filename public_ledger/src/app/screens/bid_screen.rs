use std::collections::HashSet;

use egui::Ui;

use crate::auction;
use crate::auction::signature::BidSignature;

#[derive(Default)]
pub struct BidScreen {
    curr_auction: Option<crate::auction::Auction>,
    bid_amount: String,
    status: String,
    bids: Vec<crate::auction::bid::Bid>,
    chain: Option<crate::blockchain::chain::Chain>,
    toggle_valid: bool,
}

pub enum BidScreenEvent {
    GetBids,
    SubmitBid { amount: u64 },
    Back,
}

impl BidScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<BidScreenEvent> {
        let mut result = None;
        let mut verified_bids = BidSignature::verify_bids(
            BidSignature::get_signatures(self.chain.as_ref().unwrap()),
            self.bids.clone(),
            self.curr_auction.as_ref().unwrap().clone(),
        );

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Bid Menu");
            ui.add_space(10.0);
            // Auction Info
            if let Some(auction) = &self.curr_auction {
                ui.label(format!("Auction ID: {}", auction.id));
                ui.label(format!("Item: {}", auction.item_name));
                ui.label(format!("Starting Price: {}", auction.starting_price));
                ui.label(format!("End Time: {}", auction.get_ending_time_as_string()));
                // Check if finished
                if auction.finished() {
                    ui.colored_label(egui::Color32::RED, "Finished");
                    let winning_bid = BidSignature::winning_bid(verified_bids.clone());
                    if let Some(winning_bid) = winning_bid {
                        ui.label(format!(
                            "Winning Bidder: {}",
                            hex::encode(winning_bid.bidder_id)
                        ));
                        ui.label(format!("Winning Amount: {}", winning_bid.amount));
                    } else {
                        ui.label("No winner");
                    }
                } else {
                    ui.colored_label(egui::Color32::GREEN, "Ongoing");
                    let winning_bid = BidSignature::winning_bid(verified_bids.clone());
                    if let Some(winning_bid) = winning_bid {
                        ui.label(format!(
                            "Current Winning Bidder: {}",
                            hex::encode(winning_bid.bidder_id)
                        ));
                        ui.label(format!("Current Winning Amount: {}", winning_bid.amount));
                    }
                    // Bid Section
                    ui.group(|ui| {
                        ui.label("Bid:");
                        ui.horizontal(|ui| {
                            ui.label("Enter your bid amount:");
                            ui.text_edit_singleline(&mut self.bid_amount);
                            if ui.button("Submit Bid").clicked() {
                                if let Ok(amount) = self.bid_amount.parse::<u64>() {
                                    if amount
                                        > BidSignature::winning_bid(verified_bids.clone())
                                            .map_or(0, |bid| bid.amount as u64)
                                    {
                                        self.status = "".to_string();
                                        result = Some(BidScreenEvent::SubmitBid { amount });
                                    } else {
                                        self.status =
                                            "Bid amount must be greater than current winning bid."
                                                .to_string();
                                    }
                                } else {
                                    self.status = "Invalid bid amount.".to_string();
                                }
                            }
                            ui.label(&self.status);
                        });
                    });
                }
                ui.add_space(10.0);
                // Get bids
                ui.horizontal(|ui| {
                    if ui.button("Get Bids").clicked() {
                        result = Some(BidScreenEvent::GetBids);
                    }
                    // Toggle
                    ui.checkbox(&mut self.toggle_valid, "Show Valid Bids Only");
                });
                ui.add_space(10.0);
                // Display Bids
                if self.toggle_valid {
                    ui.label("Valid Bids:");
                    for bid in verified_bids {
                        ui.horizontal(|ui| {
                            ui.label(format!("Bidder: {}", hex::encode(bid.bidder_id)));
                            ui.label(format!("Amount: {}", bid.amount));
                        });
                    }
                } else {
                    ui.label("All Bids:");

                    // Create a HashSet of verified bid IDs for quick lookup
                    let verified_bid_ids: HashSet<_> = verified_bids.iter().map(|b| b.id).collect();

                    for bid in self.bids.iter() {
                        // Check if this bid's ID is in the verified bid IDs set
                        if verified_bid_ids.contains(&bid.id) {
                            ui.horizontal(|ui| {
                                ui.label(format!("Bidder: {}", hex::encode(bid.bidder_id.clone())));
                                ui.label(format!("Amount: {}", bid.amount));
                                ui.colored_label(egui::Color32::GREEN, "Valid");
                            });
                        } else {
                            ui.horizontal(|ui| {
                                ui.label(format!("Bidder: {}", hex::encode(bid.bidder_id.clone())));
                                ui.label(format!("Amount: {}", bid.amount));
                                ui.colored_label(egui::Color32::RED, "Not Valid");
                            });
                        }
                    }
                }
            } else {
                ui.label("No auction selected.");
            }

            // Back Button
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    result = Some(BidScreenEvent::Back);
                }
            });
        });
        result
    }

    pub fn set_auction(&mut self, auction: crate::auction::Auction) {
        self.curr_auction = Some(auction);
    }

    pub fn get_auction(&self) -> Option<&crate::auction::Auction> {
        self.curr_auction.as_ref()
    }

    pub fn set_bids(&mut self, bids: Vec<crate::auction::bid::Bid>) {
        self.bids = bids;
    }

    pub fn set_chain(&mut self, chain: crate::blockchain::chain::Chain) {
        self.chain = Some(chain);
    }
}
