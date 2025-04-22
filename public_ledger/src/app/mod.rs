// main.rs
use eframe::{egui, App, Frame};
use std::sync::{Arc, RwLock};
use crate::kademlia;
use crate::routing_table;

mod screens;

use screens::{AppState, CreateNetworkScreen, InitialScreen, Screen};


pub struct AuctionApp {
    pub(crate) state: AppState,
    initial_screen: InitialScreen,
    create_network_screen: CreateNetworkScreen,
    routing_table: Option<Arc<RwLock<routing_table::RoutingTable>>>,
}

impl AuctionApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Initial,
            initial_screen: InitialScreen::default(),
            create_network_screen: CreateNetworkScreen::default(),
            routing_table: None,
        }
    }
}

impl App for AuctionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::Initial => {
                    if let Some(event) = self.initial_screen.ui(ui) {
                        match event {
                            screens::InitialScreenEvent::CreateNetwork => {
                                self.state = AppState::CreateNetwork;
                            }
                        }
                    }
                }
                AppState::CreateNetwork => {
                    if let Some(event) = self.create_network_screen.ui(ui) {
                        match event {
                            screens::CreateNetworkEvent::Submitted(port) => {
                                let addr = "::1".to_string();
                               self.routing_table = Some(Arc::new(RwLock::new(routing_table::RoutingTable::new(addr.clone(), port))));
                            if let Some(routing_table) = self.routing_table.clone() {
                                let routing_table_clone = routing_table.clone();
                                tokio::spawn(async move {
                                    if let Err(e) = kademlia::start_kademlia_server(routing_table_clone, addr, port).await {
                                        eprintln!("Failed to start Kademlia server: {}", e);
                                    }
                                });
                                self.state = AppState::Initial;
                            }
                            else {
                                println!("Routing table is not initialized");
                                self.state = AppState::Initial;
                            }
                            }
                            screens::CreateNetworkEvent::Back => {
                                self.state = AppState::Initial;
                            }
                        }
                    }
                }
            }
        });
    }
}
