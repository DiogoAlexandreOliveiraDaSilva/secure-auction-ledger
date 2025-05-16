use std::result;

use egui::Ui;

use crate::kademlia;

#[derive(Default)]
pub struct MenuScreen {
    ip: String,
    port: String,
    node_id: Vec<u8>,
    key: String,
    value: String,
    search_key: String,
    search_value: String,
    routing_table: Option<kademlia::routing_table::RoutingTable>,
}

pub enum MenuScreenEvent {
    SubmittedStore { key: String, value: String },
    SubmittedSearch { key: String },
    Auction,
    Block,
    DisplayRoutingTable,
}
impl MenuScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<MenuScreenEvent> {
        let mut result = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Main Menu");
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

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label("Search Key:");
            ui.horizontal(|ui| {
                ui.label("Key:");
                ui.text_edit_singleline(&mut self.search_key);
            });
            ui.add_space(10.0);
            if ui.button("Submit Search").clicked() {
                result = Some(MenuScreenEvent::SubmittedSearch {
                    key: self.search_key.clone(),
                });
            }
            ui.add_space(10.0);
            if !self.search_value.is_empty() {
                let value = &self.search_value;
                ui.label(format!("Search Result: {}", value));
            }
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            if ui.button("Go to Auctions").clicked() {
                result = Some(MenuScreenEvent::Auction);
            }
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            if ui.button("Go to Blockchain").clicked() {
                result = Some(MenuScreenEvent::Block);
            }
        });

        // Button to display the routing table
        ui.add_space(10.0);
        ui.group(|ui| {
            if ui.button("Display Routing Table").clicked() {
                result = Some(MenuScreenEvent::DisplayRoutingTable);
            }
        });

        // Display the routing table if it exists
        if let Some(routing_table) = &self.routing_table {
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.label("Routing Table:");
                for (index, bucket) in routing_table.get_k_bucket_map().iter() {
                    ui.label(format!("Bucket {}: ", index));
                    for node in bucket.get_nodes() {
                        ui.label(format!("Node ID: {}", hex::encode(node.get_id())));
                    }
                }
            });
        }

        result
    }

    pub fn set_info(&mut self, ip: String, port: String, node_id: Vec<u8>) {
        self.ip = ip;
        self.port = port;
        self.node_id = node_id;
    }

    pub fn set_search_value(&mut self, value: String) {
        self.search_value = value;
    }

    pub fn get_node_id(&self) -> Vec<u8> {
        self.node_id.clone()
    }

    pub fn set_routing_table(&mut self, routing_table: kademlia::routing_table::RoutingTable) {
        self.routing_table = Some(routing_table);
    }
}
