use egui::Ui;

use crate::auction;

#[derive(Default)]
pub struct BidScreen {
    curr_auction: Option<crate::auction::Auction>,
    bid_amount: String,
    status: String,
    bids: Vec<crate::auction::bid::Bid>,
}

pub enum BidScreenEvent {
    GetBids,
    SubmitBid { amount: u64 },
    Back,
}

impl BidScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<BidScreenEvent> {
        let mut result = None;
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Bid Menu");
            ui.add_space(10.0);
            ui.add_space(20.0);
            // Auction Info
            if let Some(auction) = &self.curr_auction {
                ui.label(format!("Auction ID: {}", auction.id));
                ui.label(format!("Item: {}", auction.item_name));
                ui.label(format!("Starting Price: {}", auction.starting_price));
                ui.label(format!("End Time: {}", auction.get_ending_time_as_string()));
                // Check if finished
                if auction.finished() {
                    ui.colored_label(egui::Color32::RED, "Finished");
                } else {
                    ui.colored_label(egui::Color32::GREEN, "Ongoing");
                }
            } else {
                ui.label("No auction selected.");
            }
            if let Some(auction) = &self.curr_auction {
                // Bid Section
                ui.group(|ui| {
                    ui.label("Bid:");
                    ui.horizontal(|ui| {
                        ui.label("Enter your bid amount:");
                        ui.text_edit_singleline(&mut self.bid_amount);
                        if auction.finished() {
                            ui.label("Auction has finished.");
                        } else {
                            if ui.button("Submit Bid").clicked() {
                                if let Ok(amount) = self.bid_amount.parse::<u64>() {
                                    if amount
                                        > self.curr_auction.as_ref().unwrap().starting_price as u64
                                    {
                                        self.status = "".to_string();
                                        result = Some(BidScreenEvent::SubmitBid { amount });
                                    } else {
                                        self.status =
                                            "Bid amount must be greater than starting price."
                                                .to_string();
                                    }
                                } else {
                                    self.status = "Invalid bid amount.".to_string();
                                }
                            }
                            ui.label(&self.status);
                        }
                    });
                });

                ui.add_space(10.0);
                // Get bids
                ui.horizontal(|ui| {
                    if ui.button("Get Bids").clicked() {
                        result = Some(BidScreenEvent::GetBids);
                    }
                });
                ui.add_space(10.0);
                // Display Bids
                ui.group(|ui| {
                    ui.label("Bids:");
                    if self.bids.is_empty() {
                        ui.label("No bids yet.");
                    } else {
                        for bid in &self.bids {
                            ui.horizontal(|ui| {
                                ui.label(format!("Bidder: {}", hex::encode(bid.bidder_id.clone())));
                                ui.label(format!("Amount: {}", bid.amount));
                            });
                        }
                    }
                });
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
}
