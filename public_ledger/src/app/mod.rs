// main.rs
use eframe::{egui, App, Frame};

mod screens;

use screens::{AppState, CreateNetworkScreen, InitialScreen, Screen};

pub struct AuctionApp {
    pub(crate) state: AppState,
    initial_screen: InitialScreen,
    create_network_screen: CreateNetworkScreen,
}

impl AuctionApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Initial,
            initial_screen: InitialScreen::default(),
            create_network_screen: CreateNetworkScreen::default(),
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
                                println!("Creating network at port: {}", port);
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

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Auction App",
        options,
        Box::new(|_cc| Box::new(AuctionApp::new())),
    )
}
