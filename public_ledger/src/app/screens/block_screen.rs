use egui::Ui;

use crate::blockchain::{self, chain::Chain};

#[derive(Default)]
pub struct BlockScreen {
    pub chain: Chain,
    pub transaction: String,
}

pub enum BlockScreenEvent {
    GetChain,
    Back,
    MineBlock { transaction: String },
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
                    ui.label(format!(
                        "Block Hash: {}",
                        &hex::encode(block.get_hash())
                            .get(0..20)
                            .unwrap_or(&hex::encode(block.get_hash()))
                    ));
                    ui.label(format!(
                        "Prev Block: {}",
                        hex::encode(block.header.get_parent_hash())
                            .get(0..20)
                            .unwrap_or(&hex::encode(block.header.get_parent_hash()))
                    ));
                    ui.label(format!(
                        "Transaction: {:?}",
                        String::from_utf8(block.body.get_transactions().clone()).unwrap()
                    ));
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

        // Mine Block
        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label("Mine Block:");
            ui.horizontal(|ui| {
                ui.label("Transaction:");
                ui.text_edit_singleline(&mut self.transaction);
            });
            ui.add_space(10.0);
            if ui.button("Mine").clicked() {
                result = Some(BlockScreenEvent::MineBlock {
                    transaction: self.transaction.clone(),
                });
            }
        });
        result
    }

    pub fn refresh_chain(&mut self, chain: Chain) {
        self.chain = chain;
    }
}
