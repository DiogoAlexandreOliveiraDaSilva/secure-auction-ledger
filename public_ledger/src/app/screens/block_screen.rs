use egui::Ui;

use crate::blockchain::{self, chain::Chain};

#[derive(Default)]
pub struct BlockScreen {
    pub chain: Chain,
}

pub enum BlockScreenEvent {
    GetChain,
    Back,
    // Add other events as needed
}

impl BlockScreen {
    pub fn ui(&mut self, ui: &mut Ui) -> Option<BlockScreenEvent> {
        let mut result = None;

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Blockchain");
        });

        // Display the Chain
        ui.group(|ui| {
            ui.label("Blocks:");
            for block in self.chain.get_chain() {
                ui.horizontal(|ui| {
                    ui.label(format!("Block Hash: {}", &block.get_hash()[0..20]));
                    ui.label(format!("Prev Block: {}", block.header.get_parent_hash()));
                });
            }
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("Get Chain").clicked() {
                result = Some(BlockScreenEvent::GetChain);
            }
            if ui.button("Back").clicked() {
                result = Some(BlockScreenEvent::Back);
            }
        });

        result
    }

    pub fn refresh_chain(&mut self, chain: Chain) {
        self.chain = chain;
    }
}
