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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions::default();
    Ok(eframe::run_native(
        "Auction App",
        options,
        Box::new(|_cc| Box::new(AuctionApp::new())),
    )?)
}