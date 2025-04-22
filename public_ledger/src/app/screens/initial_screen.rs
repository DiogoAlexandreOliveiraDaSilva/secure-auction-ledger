use egui::Ui;


// Initial Screen where you set the port number

#[derive(Default)]
pub struct InitialScreen {
    port: String,
}

pub enum InitialScreenEvent {
    Submitted(u16),
}


impl InitialScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<InitialScreenEvent> {
        ui.heading("Welcome to the Distributed Auction System");
        ui.label("This is a distributed auction system using Kademlia and a custom blockchain.");
        ui.label("Please enter the port number you want to start the server on.");

        // Input section
        ui.horizontal(|ui| {
            ui.label("Enter port number:");
            ui.text_edit_singleline(&mut self.port);
        });

        // Submit button (now outside the closure)
        if ui.button("Set Port").clicked() {
                    if let Ok(port) = self.port.parse::<u16>() {
                        return Some(InitialScreenEvent::Submitted(port));
                    } else {
                        println!("Invalid port: {}", self.port);
                        return None;
                    }
                }
        None

    }
}