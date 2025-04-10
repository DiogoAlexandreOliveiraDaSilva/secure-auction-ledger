use kademlia::start_kademlia_server;
use tonic::transport::Server;
use kademlia::{communication::kademlia_server, MyKademliaService, communication::kademlia_client::KademliaClient};
use kademlia::communication::{FindValueRequest, PingRequest}; // Needed for proper request building

// Tests
#[cfg(test)]
mod tests;
// Imports
mod blockchain;
mod kademlia;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // User Inputs 1 or 0s
    println!("Enter 1 (server) or 0 (client):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    // Switch case
    match input {
        "1" => {
            let addr = "::1".to_string();
            let port = 50051;
            kademlia::start_kademlia_server(addr, port).await?;
        }
        "0" => {
            let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
                .connect()
                .await?;

            let mut client = KademliaClient::new(channel);

            // Create the request object
            let request = tonic::Request::new(PingRequest {});

            let response = client.ping(request).await?;
            println!("Response: {:?}", response.into_inner());
        }
        _ => {
            println!("Invalid input. Please enter 1 or 0.");
        }
    }

    Ok(())
}
