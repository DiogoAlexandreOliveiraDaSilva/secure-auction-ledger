use egui::Ui;

#[derive(Default)]
pub struct BidScreen {}

pub enum BidScreenEvent {
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
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    result = Some(BidScreenEvent::Back);
                }
            });
        });
        result
    }
}
