use kademlia::{routing_table, start_kademlia_server};
use tonic::transport::Server;
use kademlia::{communication::kademlia_server, MyKademliaService, communication::kademlia_client::KademliaClient};
use kademlia::communication::{FindValueRequest, PingRequest}; // Needed for proper request building
use app::AuctionApp;


// Tests
#[cfg(test)]
mod tests;
// Imports
mod blockchain;
mod kademlia;
mod app;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Auction App",
        options,
        Box::new(|_cc| Box::new(AuctionApp::new())),
    )
}