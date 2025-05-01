use egui::Ui;

#[derive(Default)]
pub struct CreateScreen {
    item_name: String,
    starting_price: String,
    duration: String,
}

pub enum CreateScreenEvent {
    Submitted(String, f64, u64),
    Back,
}

impl CreateScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<CreateScreenEvent> {
        let mut result = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Create Auction");
            ui.add_space(10.0);

            ui.label("Item Name:");
            ui.text_edit_singleline(&mut self.item_name);
            ui.add_space(10.0);

            ui.label("Starting Price:");
            ui.text_edit_singleline(&mut self.starting_price);
            ui.add_space(10.0);

            ui.label("Duration (Hours):");
            ui.text_edit_singleline(&mut self.duration);
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    result = Some(CreateScreenEvent::Back);
                }
                if ui.button("Submit").clicked() {
                    if let (Ok(price), Ok(time)) = (
                        self.starting_price.parse::<f64>(),
                        self.duration.parse::<u64>(),
                    ) {
                        result = Some(CreateScreenEvent::Submitted(
                            self.item_name.clone(),
                            price,
                            time,
                        ));
                    }
                }
            });
        });

        result
    }
}
