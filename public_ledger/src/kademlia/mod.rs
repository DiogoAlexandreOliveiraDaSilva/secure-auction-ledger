pub(crate) mod routing_table;

// Parameters
use routing_table::params::MAX_BUCKET_SIZE;

// ARC and RwLock are used to allow multiple threads to access the routing table concurrently
use tokio::sync::RwLock;
use std::sync::Arc;

// Tonic GRPC server
use tonic::{transport::Server, Request, Response, Status};

// Protobuf generated code
pub mod communication {
    tonic::include_proto!("communication");
}
use communication::kademlia_server::{Kademlia, KademliaServer};
use communication::{PingRequest, PingResponse, FindNodeRequest, FindNodeResponse, StoreRequest, StoreResponse, FindValueRequest, FindValueResponse, kademlia_client::KademliaClient};

// This is the main Kademlia service that will handle all the requests
pub struct MyKademliaService {
    pub routing_table: Arc<RwLock<routing_table::RoutingTable>>,
}

// This is the remote procedure call (RPC) implementation of the Kademlia service
#[tonic::async_trait]
impl Kademlia for MyKademliaService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let id: [u8; 20] = request.get_ref().id.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;

        // Log the received ping
        println!("Received ping from node with ID: {:?}", hex::encode(id));

        // Update the routing table with the new node
        update_routing_table_with_node(
            &self.routing_table,
            id,
            request.remote_addr().unwrap().ip().to_string(),
            request.remote_addr().unwrap().port(),
        ).await;
        
        // Create a response with the current node's ID
        let routing_table = self.routing_table.read().await;
        let reply = PingResponse {
            id: routing_table.get_curr_node().get_id().to_vec(),
            message: format!("Pong"),
        };
        Ok(Response::new(reply))
    }

    async fn find_node(&self, request: Request<FindNodeRequest>) -> Result<Response<FindNodeResponse>, Status> {
        // Extract the ID from the request
        let key: [u8; 20] = request.get_ref().key.clone().try_into()
            .map_err(|_| Status::invalid_argument("Invalid ID length"))?;
    
        // Scope 1: Read lock to get the closest nodes
        let closest_nodes = {
            let routing_table = self.routing_table.read().await;
            routing_table.get_closest_k_nodes(&key, MAX_BUCKET_SIZE)
        };
    
        // Convert to protobuf nodes
        let nodes: Vec<communication::Node> = closest_nodes
            .into_iter()
            .map(|node| communication::Node {
                id: node.get_id().to_vec(),
                ip: node.get_ip(),
                port: node.get_port() as u32,
            })
            .collect();
    
        // Scope 2: Write lock to update the routing table
        {
            let id: [u8; 20] = request.get_ref().id.clone().try_into()
                .map_err(|_| Status::invalid_argument("Invalid ID length"))?;
    
            update_routing_table_with_node(
                &self.routing_table,
                id,
                request.remote_addr().unwrap().ip().to_string(),
                request.remote_addr().unwrap().port(),
            ).await;
        }
    
        // Response with the closest nodes
        let reply = FindNodeResponse { nodes };
        Ok(Response::new(reply))
    }
    

    async fn store(&self, request: Request<StoreRequest>) -> Result<Response<StoreResponse>, Status> {
        // Extract key
        let key: [u8; 20] = request.get_ref().key.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        // Extract value
        let value: Vec<u8> = request.get_ref().value.clone();
        // Store the value in the local storage
        let mut routing_table = self.routing_table.write().await;
        routing_table.store(key, value);
        // Update the routing table with the new node
        let id: [u8; 20] = request.get_ref().id.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        update_routing_table_with_node(
            &self.routing_table,
            id,
            request.remote_addr().unwrap().ip().to_string(),
            request.remote_addr().unwrap().port(),
        ).await;

        // Create a response with the stored message
        let reply = StoreResponse {
            message: "Stored successfully".to_string(),
        };
        Ok(Response::new(reply))
    }

    async fn find_value(&self, request: Request<FindValueRequest>) -> Result<Response<FindValueResponse>, Status> {
        // Extract the key from the request
        let key: [u8; 20] = request.get_ref().key.clone().try_into()
            .map_err(|_| Status::invalid_argument("Invalid ID length"))?;
    
        // Scope 1: Read lock to try to get the value
        if let Some(value) = {
            let routing_table = self.routing_table.read().await;
            routing_table.get(key).cloned()
        } {
            // If value is found
            let reply = FindValueResponse {
                value,
                nodes: vec![],
            };
            return Ok(Response::new(reply));
        }
    
        // Scope 2: Read lock to get closest nodes (value not found above)
        let closest_nodes = {
            let routing_table = self.routing_table.read().await;
            routing_table.get_closest_k_nodes(&key, MAX_BUCKET_SIZE)
        };
    
        // Convert closest nodes into protobuf format
        let nodes: Vec<communication::Node> = closest_nodes
            .into_iter()
            .map(|node| communication::Node {
                id: node.get_id().to_vec(),
                ip: node.get_ip(),
                port: node.get_port() as u32,
            })
            .collect();
    
        // Scope 3: Write lock to add new node to the routing table
        {
            let id: [u8; 20] = request.get_ref().id.clone().try_into()
                .map_err(|_| Status::invalid_argument("Invalid ID length"))?;
    
            update_routing_table_with_node(
                &self.routing_table,
                id,
                request.remote_addr().unwrap().ip().to_string(),
                request.remote_addr().unwrap().port(),
            ).await;
        }
    
        // Return response with closest nodes to continue the search
        let reply = FindValueResponse {
            value: vec![],
            nodes,
        };
    
        Ok(Response::new(reply))
    }    
}

// This function starts the Kademlia server, this will process all calls made to it and update routing table
pub async fn start_kademlia_server(
    routing_table: Arc<RwLock<routing_table::RoutingTable>>,
    addr: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let kademlia_service = MyKademliaService {
        routing_table: routing_table.clone(),
    };
    let kademlia_server = KademliaServer::new(kademlia_service);

    let socket_addr = format!("[{}]:{}", addr, port).parse()?;

    Server::builder()
        .add_service(kademlia_server)
        .serve(socket_addr)
        .await?;

    Ok(())
}

pub async fn join_kademlia_network(
    routing_table: Arc<RwLock<routing_table::RoutingTable>>,
    boot_addr: String,
    boot_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("http://[{}]:{}", boot_addr, boot_port);

    // Create a new Kademlia client
    let mut client = KademliaClient::connect(uri).await?;

    // Get the current node's ID (and release the read lock after)
    let curr_node_id = {
        let routing_table_read = routing_table.read().await;
        routing_table_read.get_curr_node().get_id().to_vec()
    };

    // Send a ping to the bootstrap node
    let request = tonic::Request::new(PingRequest {
        id: curr_node_id.clone(),
    });

    let response = client.ping(request).await?;

    // Update the routing table with the bootstrap node's ID
    let boot_node_id: [u8; 20] = response
        .get_ref()
        .id
        .clone()
        .try_into()
        .map_err(|_| "Invalid ID length")?;

    println!("Ping response from bootstrap node with ID: {:?}", hex::encode(boot_node_id));

    update_routing_table_with_node(
        &routing_table,
        boot_node_id,
        boot_addr.clone(),
        boot_port,
    ).await;

    Ok(())
}


async fn update_routing_table_with_node(routing_table: &RwLock<routing_table::RoutingTable>, id: [u8; 20], ip: String, port: u16,) {
    let new_node = routing_table::node::Node::with_id(id, ip, port);
    let mut routing_table = routing_table.write().await;
    routing_table.add_node(new_node);
    println!("Added node with ID to Table: {:?}", hex::encode(id));
}
