use egui::Ui;

#[derive(Default)]
pub struct BidScreen {
    curr_auction: Option<crate::auction::Auction>,
    bid_amount: String,
}

pub enum BidScreenEvent {
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

            // Bid Section
            ui.group(|ui| {
                ui.label("Bid:");
                ui.horizontal(|ui| {
                    ui.label("Enter your bid amount:");
                    ui.text_edit_singleline(&mut self.bid_amount);
                    if ui.button("Submit Bid").clicked() {
                        if let Ok(amount) = self.bid_amount.parse::<u64>() {
                            result = Some(BidScreenEvent::SubmitBid { amount });
                        } else {
                            ui.label("Invalid amount. Please enter a valid number.");
                        }
                    }
                });
            });

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
}
