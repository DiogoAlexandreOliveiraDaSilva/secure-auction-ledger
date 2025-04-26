// main.rs
use eframe::{egui, App, Frame};
use screens::join_screen::JoinScreen;
use screens::menu_screen::MenuScreen;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::kademlia;
use crate::routing_table;

mod screens;
use screens::initial_screen::{InitialScreen, InitialScreenEvent};
use screens::selection_screen::{SelectionScreenEvent, SelectionScreen};
use screens::join_screen::JoinScreenEvent;
use screens::AppState;

pub struct AuctionApp {
    pub(crate) state: AppState,
    routing_table: Option<Arc<RwLock<routing_table::RoutingTable>>>,
    initial_screen: InitialScreen,
    selection_screen: SelectionScreen,
    join_screen: JoinScreen,
    menu_screen: MenuScreen,
}

impl AuctionApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Initial,
            initial_screen: InitialScreen::default(),
            routing_table: None,
            selection_screen: SelectionScreen::default(),
            join_screen: JoinScreen::default(),
            menu_screen: MenuScreen::default(),
        }
    }

    pub fn update_menu_screen_info(&mut self) {
        if let Some(rt) = &self.routing_table {
            if let Ok(rt_read) = rt.try_read() {
                let node = rt_read.get_curr_node().clone();
                self.menu_screen.set_info(node.get_ip(), node.get_port().to_string(), node.get_id().to_vec());
            }
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
                                            } else {
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
                                            self.state = AppState::Join;
                                        }
                                        SelectionScreenEvent::Create => {
                                            self.update_menu_screen_info();
                                            self.state = AppState::Menu;
                                        }
                                    }
                                }
                            }
                AppState::Join => {
                                if let Some(event) = self.join_screen.ui(ui) {
                                    match event {
                                        JoinScreenEvent::Submitted(port) => {
                                            let routing_table = self.routing_table.clone().unwrap();
                                            let addr = "::1".to_string();
                                            tokio::spawn(async move {
                                                if let Err(e) = kademlia::join_kademlia_network(routing_table, addr, port).await {
                                                    eprintln!("Failed to join Kademlia network: {}", e);
                                                }
                                            });

                                            self.update_menu_screen_info();
                                            self.state = AppState::Menu;
                                        }
                                        JoinScreenEvent::Back => {
                                            self.state = AppState::Selection;
                                        }
                                    }
                                }
                            }
                AppState::Menu => {
                            self.menu_screen.ui(ui);
                },
            }
        });
    }

}
