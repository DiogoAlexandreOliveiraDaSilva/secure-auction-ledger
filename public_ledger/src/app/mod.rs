// main.rs
use eframe::{egui, App, Frame};
use std::sync::{Arc, RwLock};
use crate::kademlia;
use crate::routing_table;

mod screens;
use screens::initial_screen::{InitialScreen, InitialScreenEvent};
use screens::selection_screen::{SelectionScreenEvent, SelectionScreen};
use screens::AppState;


pub struct AuctionApp {
    pub(crate) state: AppState,
    routing_table: Option<Arc<RwLock<routing_table::RoutingTable>>>,
    initial_screen: InitialScreen,
    selection_screen: SelectionScreen,
}

impl AuctionApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Initial,
            initial_screen: InitialScreen::default(),
            routing_table: None,
            selection_screen: SelectionScreen::default(),
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
                                            InitialScreenEvent::Submitted(port) => {
                                            let addr = "::1".to_string();
                                            self.routing_table = Some(Arc::new(RwLock::new(routing_table::RoutingTable::new(addr.clone(), port))));
                                        if let Some(routing_table) = self.routing_table.clone() {
                                            let routing_table_clone = routing_table.clone();
                                            let addr_clone = addr.clone();
                                            tokio::spawn({
                                                let addr_clone = addr_clone.clone();
                                                async move {
                                                    if let Err(e) = kademlia::start_kademlia_server(routing_table_clone, addr_clone.clone(), port).await {
                                                        eprintln!("Failed to start Kademlia server: {}", e);
                                                    }
                                                }
                                            });
                                            println!("Kademlia server started on {}:{}", addr, port);
                                            self.state = AppState::Selection;
                                        }
                                        else {
                                            println!("Routing table is not initialized");
                                            self.state = AppState::Initial;
                                        }
                                        }
                                    }
                                }
                            }
                AppState::Selection => {  
                    if let Some(event) = self.selection_screen.ui(ui) {
                        match event {
                            SelectionScreenEvent::Join => {
                                println!("Join button clicked");
                                // Handle join action
                            }
                            SelectionScreenEvent::Create => {
                                println!("Create button clicked");
                                // Handle create action
                            }
                        }
                    }
                }
            }
        });
    }
}
