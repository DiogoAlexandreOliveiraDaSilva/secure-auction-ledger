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
        let mut event = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Distributed Auction System");

            ui.add_space(10.0);
            ui.label("You can now either join an existing auction house or create a new one.");

            ui.add_space(20.0);
            
            ui.horizontal(|ui| {
                if ui.button("Join").clicked() {
                    event = Some(SelectionScreenEvent::Join);
                }
                if ui.button("Create").clicked() {
                    event = Some(SelectionScreenEvent::Create);
                }
            });

            ui.add_space(10.0);
            ui.label("Please select an option to proceed.");
        });

        event
    }
}
