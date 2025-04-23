use egui::Ui;

#[derive(Default)]
pub struct MenuScreen {
    ip: String,
    port: String,
    node_id: Vec<u8>,
}

pub enum MenuScreenEvent {
}

impl MenuScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<MenuScreenEvent> {
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

        None
    }

    pub fn set_info(&mut self, ip: String, port: String, node_id: Vec<u8>) {
        self.ip = ip;
        self.port = port;
        self.node_id = node_id;
    }
}