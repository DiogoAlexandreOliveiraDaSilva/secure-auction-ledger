pub(crate) mod routing_table;
use tonic::{transport::Server,Request,Response,Status};

pub mod communication{
    tonic::include_proto!("communication");
}

use communication::kademlia_server::{Kademlia, KademliaServer};
use communication::{PingRequest, PingResponse, FindNodeRequest, FindNodeResponse, StoreRequest, StoreResponse,FindValueRequest ,FindValueResponse };

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
   