use egui::Ui;

#[derive(Default)]
pub struct JoinScreen {
    port: String,
}

pub enum JoinScreenEvent {
    Submitted(u16),
    Back,
}

impl JoinScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<JoinScreenEvent> {
        let mut result = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Join or Create Distribution Auction Network");

            ui.add_space(10.0);
            ui.label("Please enter the port number to join the server:");

            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.label("Port:");
                ui.add(egui::TextEdit::singleline(&mut self.port).hint_text("e.g. 8080"));
            });

            ui.add_space(15.0);
            if ui.button("Join Server").clicked() {
                match self.port.parse::<u16>() {
                    Ok(port) => result = Some(JoinScreenEvent::Submitted(port)),
                    Err(_) => {
                        println!("Invalid port: {}", self.port);
                    }
                }
            }

            ui.add_space(10.0);
            if ui.button("Back").clicked() {
                result = Some(JoinScreenEvent::Back);
            }
        });

        result
    }
}
