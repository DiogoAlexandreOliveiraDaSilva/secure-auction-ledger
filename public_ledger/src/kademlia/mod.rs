pub(crate) mod routing_table;

// Parameters
use routing_table::params::MAX_BUCKET_SIZE;

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
        // Update the routing table with the new node
        let id: [u8; 20] = request.get_ref().id.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        let node = routing_table::node::Node::with_id(id, request.remote_addr().unwrap().ip().to_string(), request.remote_addr().unwrap().port());
        let mut routing_table = self.routing_table.write().unwrap();
        routing_table.add_node(node);

        // Create a response with the current node's ID
        let reply = PingResponse {
            id: routing_table.get_curr_node().get_id().to_vec(),
            message: format!("Pong"),
        };
        Ok(Response::new(reply))
    }

    async fn find_node(&self, request: Request<FindNodeRequest>) -> Result<Response<FindNodeResponse>, Status> {
        // Extract the ID from the request
        let key: [u8; 20] = request.get_ref().key.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;

        // Get K-closest nodes
        let routing_table = self.routing_table.read().unwrap();
        let closest_nodes = routing_table.get_closest_k_nodes(&key, MAX_BUCKET_SIZE);

        // Convert the nodes to the protobuf format
        let mut nodes = Vec::new();
        for node in closest_nodes {
            nodes.push(communication::Node {
                id: node.get_id().to_vec(),
                ip: node.get_ip(),
                port: node.get_port() as u32,
            });
        }

        // Update the routing table with the new node
        let mut routing_table = self.routing_table.write().unwrap();
        let id: [u8; 20] = request.get_ref().id.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        let new_node = routing_table::node::Node::with_id(id, request.remote_addr().unwrap().ip().to_string(), request.remote_addr().unwrap().port());
        routing_table.add_node(new_node);

        // Create a response with the closest nodes
        let reply = FindNodeResponse {
            nodes: nodes,
        };
        Ok(Response::new(reply))
    }

    async fn store(&self, request: Request<StoreRequest>) -> Result<Response<StoreResponse>, Status> {
        // Extract key
        let key: [u8; 20] = request.get_ref().key.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        // Extract value
        let value: Vec<u8> = request.get_ref().value.clone();
        // Store the value in the local storage
        let mut routing_table = self.routing_table.write().unwrap();
        routing_table.store(key, value);

        // Update the routing table with the new node
        let id: [u8; 20] = request.get_ref().id.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        let new_node = routing_table::node::Node::with_id(id, request.remote_addr().unwrap().ip().to_string(), request.remote_addr().unwrap().port());
        routing_table.add_node(new_node);

        // Create a response with the stored message
        let reply = StoreResponse {
            message: "Stored successfully".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn find_value(&self, request: Request<FindValueRequest>) -> Result<Response<FindValueResponse>, Status> {
        // Extract the key from the request
        let key: [u8; 20] = request.get_ref().key.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        
        // Get the value from the local storage (from the routing table or a storage system)
        let routing_table = self.routing_table.read().unwrap();
        
        // Check if the value is in local storage
        if let Some(value) = routing_table.get(key) {
            // If the value is found, return the value
            let reply = FindValueResponse {
                value: value.clone(),
                nodes: vec![], // No need to send nodes if the value is found
            };
            return Ok(Response::new(reply));
        } else {
            // If the value is not found, get the k-closest nodes from the routing table
            let closest_nodes = routing_table.get_closest_k_nodes(&key, MAX_BUCKET_SIZE);
    
            // Convert the nodes to the protobuf format
            let mut nodes = Vec::new();
            for node in closest_nodes {
                nodes.push(communication::Node {
                    id: node.get_id().to_vec(),
                    ip: node.get_ip(),
                    port: node.get_port() as u32,
                });
            }

            // Update the routing table with the new node
            let id: [u8; 20] = request.get_ref().id.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
            let new_node = routing_table::node::Node::with_id(id, request.remote_addr().unwrap().ip().to_string(), request.remote_addr().unwrap().port());
            let mut routing_table = self.routing_table.write().unwrap();
            routing_table.add_node(new_node);
            
    
            // Return the response with nodes to continue the search
            let reply = FindValueResponse {
                value: vec![],  // No value found, so return an empty vector for value
                nodes,  // Include the closest nodes to continue the search
            };
            
            Ok(Response::new(reply))
        }
    }    
}

// This function starts the Kademlia server, this will process all calls made to it and update routing table
pub async fn start_kademlia_server(addr: String, port: u16) -> Result<Arc<RwLock<routing_table::RoutingTable>>, Box<dyn std::error::Error>> {
    let routing_table = Arc::new(RwLock::new(routing_table::RoutingTable::new(addr.clone(), port)));
    let kademlia_service = MyKademliaService {
        routing_table: routing_table.clone(),
    };
    let kademlia_server = KademliaServer::new(kademlia_service);

    let socket_addr = format!("[{}]:{}", addr, port).parse()?; 

    println!("Server listening on {}:{}", addr, port);
    Server::builder()
        .add_service(kademlia_server)
        .serve(socket_addr)
        .await?;

    Ok(routing_table)
}
