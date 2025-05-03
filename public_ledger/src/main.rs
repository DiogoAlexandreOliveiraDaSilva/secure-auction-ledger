use app::AuctionApp;
use kademlia::communication::{FindValueRequest, PingRequest}; // Needed for proper request building
use kademlia::{
    MyKademliaService, communication::kademlia_client::KademliaClient,
    communication::kademlia_server,
};
use kademlia::{routing_table, start_kademlia_server};
use tonic::transport::Server;

// Tests
#[cfg(test)]
mod tests;
// Imports
mod app;
mod auction;
mod blockchain;
mod kademlia;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions::default();
    Ok(eframe::run_native(
        "Auction App",
        options,
        Box::new(|_cc| Ok(Box::new(AuctionApp::new()))),
    )?)
}
