use egui::Ui;

#[derive(Default)]
pub struct AuctionScreen {}

pub enum AuctionScreenEvent {
    Create,
    Back,
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
        });

        result
    }
}
