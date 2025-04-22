use egui::Ui;


#[derive(Default)]
pub struct SelectionScreen {
    // No fields needed for now
}

pub enum SelectionScreenEvent { 
    Join,
    Create
}

impl SelectionScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<SelectionScreenEvent> {
        ui.heading("Distributed Auction System");
        ui.label("Now you can either join an existing auction house or create a new one.");
        let mut event = None;
        ui.horizontal(|ui| {
            if ui.button("Join").clicked() {
                event = Some(SelectionScreenEvent::Join);
            }
            if ui.button("Create").clicked() {
                event = Some(SelectionScreenEvent::Create);
            }
        });
        ui.label("Please select an option.");
        event
    }
}
