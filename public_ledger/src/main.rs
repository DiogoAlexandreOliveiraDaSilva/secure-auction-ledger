use tonic::transport::Server;
use kademlia::{communication::kademlia_server, MyKademliaService};

// Tests
#[cfg(test)]
mod tests;
// Imports
mod blockchain;
mod kademlia;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let kademlia_service = MyKademliaService::default();
    let kademlia_server = kademlia_server::KademliaServer::new(kademlia_service);
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(kademlia_server)
        .serve(addr)
        .await
        .unwrap();
    Ok(())
}