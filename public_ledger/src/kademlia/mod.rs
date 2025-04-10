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
   
// This function starts the Kademlia server and corresponding routing services
pub async fn start_kademlia_server(addr: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let routing_table = routing_table::RoutingTable::new(addr.clone(), port);
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

// // Connect to bootstrap node
// pub async fn connect_to_bootstrap_node(addr: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
//     let channel = tonic::transport::Channel::from_static(&format!("http://[{}]:{}", addr, port))
//         .connect()
//         .await?;

//     let mut client = KademliaClient::new(channel);

//     // Create the request object
//     let request = tonic::Request::new(PingRequest {});

//     let response = client.ping(request).await?;
//     println!("Response: {:?}", response.into_inner());

//     let inner_response = response.into_inner();
//     match inner_response.message.as_str() {
//         "Pong" => {
//             println!("Connected to bootstrap node at {}:{}", addr, port);
//             // Add Node to routing table
//             let node = routing_table::Node::new(addr.clone(), port);
//         },
//         _ => println!("Failed to connect to bootstrap node"),
//     }
    

//     Ok(())
// }