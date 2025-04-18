pub(crate) mod routing_table;

// ARC and RwLock are used to allow multiple threads to access the routing table concurrently
use std::sync::{Arc, RwLock};


// Tonic GRPC server
use tonic::{transport::Server,Request,Response,Status};

// Protobuf generated code
pub mod communication{
    tonic::include_proto!("communication");
}
use communication::kademlia_server::{Kademlia, KademliaServer};
use communication::{PingRequest, PingResponse, FindNodeRequest, FindNodeResponse, StoreRequest, StoreResponse,FindValueRequest ,FindValueResponse, kademlia_client::KademliaClient};

// This is the main Kademlia service that will handle all the requests
pub struct MyKademliaService {
    pub routing_table: Arc<RwLock<routing_table::RoutingTable>>,
}


// This is the remote procedure call (RPC) implementation of the Kademlia service
#[tonic::async_trait]
impl Kademlia for MyKademliaService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        // Add node to routing table
        let id: [u8; 20] = request.get_ref().id.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        let node = routing_table::node::Node::with_id(id, request.remote_addr().unwrap().ip().to_string(), request.remote_addr().unwrap().port());
        let mut routing_table = self.routing_table.write().unwrap();
        routing_table.add_node(node);

        // Create a response
        let reply = PingResponse {
            id: routing_table.get_curr_node().get_id().to_vec(),
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
   
// This function starts the Kademlia server, this will process all calls made to it and update routing table
pub async fn start_kademlia_server(addr: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let kademlia_service = MyKademliaService {
        routing_table: Arc::new(RwLock::new(routing_table::RoutingTable::new(addr.clone(), port))),
    };
    let kademlia_server = KademliaServer::new(kademlia_service);

    let socket_addr = format!("[{}]:{}", addr, port).parse()?; 

    println!("Server listening on {}:{}", addr, port);
    Server::builder()
        .add_service(kademlia_server)
        .serve(socket_addr)
        .await?;

    Ok(())
}
