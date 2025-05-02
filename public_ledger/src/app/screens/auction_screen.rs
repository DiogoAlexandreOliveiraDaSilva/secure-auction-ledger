use egui::Ui;

use crate::auction::Auction;

#[derive(Default)]
pub struct AuctionScreen {
    auction_list: Vec<Auction>,
}

pub enum AuctionScreenEvent {
    Create,
    Back,
    GetAuctions,
}

impl AuctionScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<AuctionScreenEvent> {
        let mut result = None;

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
                        ui.label(format!("End Time: {}", auction.ending_time));
                    });
                }
            });
        });

        result
    }

    pub fn refresh_auctions(&mut self, auctions: Vec<Auction>) {
        self.auction_list = auctions;
    }
}
