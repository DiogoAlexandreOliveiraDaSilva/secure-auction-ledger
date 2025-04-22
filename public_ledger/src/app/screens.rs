use egui::Ui;

#[derive(Default)]
pub struct InitialScreen;

#[derive(Default)]
pub struct CreateNetworkScreen {
    pub port: String,
}

pub enum AppState {
    Initial,
    CreateNetwork,
}

pub enum InitialScreenEvent {
    CreateNetwork,
}

pub enum CreateNetworkEvent {
    Submitted(u16),
    Back,
}

pub trait Screen {
    type Event;

    fn ui(&mut self, ui: &mut Ui) -> Option<Self::Event>;
}

impl InitialScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<InitialScreenEvent> {
        ui.heading("Welcome to the Distributed Auction System");

        if ui.button("Create Network").clicked() {
            return Some(InitialScreenEvent::CreateNetwork);
        }
        None
    }
}

impl CreateNetworkScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<CreateNetworkEvent> {
        ui.label("Creating a new network…");

        // Input section
        ui.horizontal(|ui| {
            ui.label("Enter port number:");
            ui.text_edit_singleline(&mut self.port);
        });

        // Submit button (now outside the closure)
        if ui.button("Submit").clicked() {
            if let Ok(port) = self.port.parse::<u16>() {
                return Some(CreateNetworkEvent::Submitted(port));
            } else {
                println!("Invalid port: {}", self.port);
            }
        }

        if ui.button("Back").clicked() {
            return Some(CreateNetworkEvent::Back);
        }

        None
    }
}
