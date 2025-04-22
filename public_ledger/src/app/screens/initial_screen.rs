use egui::Ui;

#[derive(Default)]
pub struct InitialScreen {
    port: String,
}

pub enum InitialScreenEvent {
    Submitted(u16),
}

impl InitialScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<InitialScreenEvent> {
        let mut result = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Welcome to the Distributed Auction System");

            ui.add_space(10.0);
            ui.label("This is a distributed auction system using Kademlia and a custom blockchain.");
            ui.label("Please enter the port number to start the server:");

            ui.add_space(20.0);
            ui.horizontal(|ui| {
                ui.label("Port:");
                ui.add(egui::TextEdit::singleline(&mut self.port).hint_text("e.g. 8080"));
            });

            ui.add_space(15.0);
            if ui.button("Start Server").clicked() {
                match self.port.parse::<u16>() {
                    Ok(port) => result = Some(InitialScreenEvent::Submitted(port)),
                    Err(_) => {
                        println!("Invalid port: {}", self.port);
                    }
                }
            }
        });

        result
    }
}
