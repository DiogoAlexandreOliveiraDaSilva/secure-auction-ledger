use egui::Ui;

#[derive(Default)]
pub struct BlockScreen {}

pub enum BlockScreenEvent {
    Back,
    // Add other events as needed
}

impl BlockScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<BlockScreenEvent> {
        let mut result = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Blockchain");
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("Back").clicked() {
                result = Some(BlockScreenEvent::Back);
            }
        });

        result
    }
}
