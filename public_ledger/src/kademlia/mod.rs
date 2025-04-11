pub(crate) mod routing_table;
use tonic::{transport::Server,Request,Response,Status};

pub mod communication{
    tonic::include_proto!("communication");
}

use communication::kademlia_server::{Kademlia, KademliaServer};
use communication::{PingRequest, PingResponse, FindNodeRequest, FindNodeResponse, StoreRequest, StoreResponse,FindValueRequest ,FindValueResponse, kademlia_client::KademliaClient};

#[derive(Default)]
pub struct MyKademliaService;

#[tonic::async_trait]
impl Kademlia for MyKademliaService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let reply = PingResponse {
            message: format!("Pong"),
        };
        Ok(Response::new(reply))
    }

    async fn find_node(&self, request: Request<FindNodeRequest>) -> Result<Response<FindNodeResponse>, Status> {
        let reply = FindNodeResponse {
            nodes: vec![],
        };
        Ok(Response::new(reply))
    }

    async fn store(&self, request: Request<StoreRequest>) -> Result<Response<StoreResponse>, Status> {
        let reply = StoreResponse {
            message: "Stored successfully".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn find_value(&self, request: Request<FindValueRequest>) -> Result<Response<FindValueResponse>, Status> {
        let reply = FindValueResponse {
            value: "value".as_bytes().to_vec(),
            nodes: vec![],
        };
        Ok(Response::new(reply))
    }
}
   
// This function starts the Kademlia server
pub async fn start_kademlia_server(addr: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let kademlia_service = MyKademliaService::default();  
    let kademlia_server = KademliaServer::new(kademlia_service);

    let socket_addr = format!("[{}]:{}", addr, port).parse()?; // This fixes AddrParseError

    println!("Server listening on {}:{}", addr, port);
    Server::builder()
        .add_service(kademlia_server)
        .serve(socket_addr)
        .await?;

    Ok(())
}
