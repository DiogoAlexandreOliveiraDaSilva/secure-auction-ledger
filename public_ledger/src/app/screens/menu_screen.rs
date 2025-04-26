use std::result;

use egui::Ui;

#[derive(Default)]
pub struct MenuScreen {
    ip: String,
    port: String,
    node_id: Vec<u8>,
    key: String,
    value: String,
}

pub enum MenuScreenEvent {
    SubmittedStore {
        key: String,
        value: String,
    },
}
impl MenuScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<MenuScreenEvent> {
        let mut result = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Distributed Auction System Menu");
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label("My Info:");
            ui.horizontal(|ui| {
                ui.label("IP:");
                ui.text_edit_singleline(&mut self.ip);
            });
            ui.horizontal(|ui| {
                ui.label("Port:");
                ui.text_edit_singleline(&mut self.port);
            });
            ui.horizontal(|ui| {
                ui.label("Node ID:");
                ui.text_edit_singleline(&mut hex::encode(self.node_id.clone()));
            });
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label("Store Key-Value Pair:");
            ui.horizontal(|ui| {
                ui.label("Key:");
                ui.text_edit_singleline(&mut self.key);
            });
            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.text_edit_singleline(&mut self.value);
            });

            ui.add_space(10.0);
            if ui.button("Submit Store").clicked() {
                result = Some(MenuScreenEvent::SubmittedStore {
                    key: self.key.clone(),
                    value: self.value.clone(),
                });
            }
        });

        result
    }

    pub fn set_info(&mut self, ip: String, port: String, node_id: Vec<u8>) {
        self.ip = ip;
        self.port = port;
        self.node_id = node_id;
    }
}