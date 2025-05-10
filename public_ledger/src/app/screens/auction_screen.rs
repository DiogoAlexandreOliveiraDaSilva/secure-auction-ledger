use egui::Ui;

use crate::auction::Auction;
use crate::auction::signature::AuctionSignature;

#[derive(Default)]
pub struct AuctionScreen {
    auction_list: Vec<Auction>,
    chain: Option<crate::blockchain::chain::Chain>,
}

pub enum AuctionScreenEvent {
    Create,
    Back,
    GetAuctions,
    BidMenu { auction_id: u32 },
}

impl AuctionScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<AuctionScreenEvent> {
        let mut result = None;
        let mut verified_auctions = AuctionSignature::verify_auctions(
            AuctionSignature::get_signatures(self.chain.as_ref().unwrap()),
            self.auction_list.clone(),
        );

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Auction Menu");
            ui.add_space(10.0);
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.button("Create Auction").clicked() {
                    result = Some(AuctionScreenEvent::Create);
                }
                if ui.button("Back").clicked() {
                    result = Some(AuctionScreenEvent::Back);
                }
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Get Auctions").clicked() {
                    result = Some(AuctionScreenEvent::GetAuctions);
                }
            });
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.label("Auctions:");
                for auction in &self.auction_list {
                    ui.horizontal(|ui| {
                        ui.label(format!("Auction ID: {}", auction.id));
                        ui.label(format!("Item: {}", auction.item_name));
                        ui.label(format!("Starting Price: {}", auction.starting_price));
                        ui.label(format!("End Time: {}", auction.get_ending_time_as_string()));
                        // If auction is in verified_auctions list, show "Verified" else "Not Verified"
                        if verified_auctions.iter().any(|a| a.id == auction.id) {
                            ui.colored_label(egui::Color32::GREEN, "Verified");
                            // Check i finished
                            if auction.finished() {
                                ui.colored_label(egui::Color32::RED, "Finished");
                            } else {
                                ui.colored_label(egui::Color32::GREEN, "Ongoing");
                                // Bid Button
                                if ui.button("Bid").clicked() {
                                    result = Some(AuctionScreenEvent::BidMenu {
                                        auction_id: auction.id,
                                    });
                                }
                            }
                        } else {
                            ui.colored_label(egui::Color32::RED, "Not Verified");
                        }
                    });
                }
            });
        });

        result
    }

    pub fn refresh_auctions(&mut self, auctions: Vec<Auction>) {
        self.auction_list = auctions;
    }

    pub fn set_chain(&mut self, chain: crate::blockchain::chain::Chain) {
        self.chain = Some(chain);
    }
}
