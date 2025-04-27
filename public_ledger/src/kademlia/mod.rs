pub(crate) mod routing_table;

use futures::future::join_all;
use routing_table::node::{self, Node};
use routing_table::node_id::distance;
// Parameters
use routing_table::params::{MAX_BUCKET_SIZE, ALPHA};

// ARC and RwLock are used to allow multiple threads to access the routing table concurrently
use tokio::sync::{RwLock};
use std::collections::HashSet;
use std::sync::Arc;

// Tonic GRPC server
use tonic::{transport::Server, Request, Response, Status};

// Protobuf generated code
pub mod communication {
    tonic::include_proto!("communication");
}
use communication::kademlia_server::{Kademlia, KademliaServer};
use communication::{PingRequest, PingResponse, FindNodeRequest, FindNodeResponse, StoreRequest, StoreResponse, FindValueRequest, FindValueResponse, kademlia_client::KademliaClient};

use crate::kademlia;
use ring::digest::{Context, SHA256};

// This is the main Kademlia service that will handle all the requests
pub struct MyKademliaService {
    pub routing_table: Arc<RwLock<routing_table::RoutingTable>>,
}

// This is the remote procedure call (RPC) implementation of the Kademlia service
#[tonic::async_trait]
impl Kademlia for MyKademliaService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
       let node = routing_table::node::Node::from_proto(request.get_ref().node.as_ref().unwrap());
    
        // Log the received ping
        println!(
            "Received ping from node with ID: {:?}, IP: {}, Port: {}",
            hex::encode(node.get_id()),
            node.get_ip(),
            node.get_port()
        );

        // Update the routing table with the new node
        update_routing_table_with_node(
            &self.routing_table,
            *node.get_id(),
            node.get_ip(),
           node.get_port(),
        ).await;
        
        // Create a response with the current node's ID
        let routing_table = self.routing_table.read().await;
        let reply = PingResponse {
            node: Some(routing_table.get_curr_node().to_proto()),
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
            let node = routing_table::node::Node::from_proto(request.get_ref().node.as_ref().unwrap());
    
            update_routing_table_with_node(
                &self.routing_table,
                node.get_id().clone(),
                node.get_ip(),
                node.get_port(),
            ).await;
        }
    
        // Response with the closest nodes
        let reply = FindNodeResponse { nodes };
        Ok(Response::new(reply))
    }
    

    async fn store(&self, request: Request<StoreRequest>) -> Result<Response<StoreResponse>, Status> {
        println!("Received store request");
        // Extract key
        let key: [u8; 20] = request.get_ref().key.clone().try_into().map_err(|_| Status::invalid_argument("Invalid ID length"))?;
        // Extract value
        let value: Vec<u8> = request.get_ref().value.clone();
    
        {
            // Scope the lock so it's dropped early
            let mut routing_table = self.routing_table.write().await;
            routing_table.store(key, value);
        } 
    
        // Update the routing table with the new node
        let node = routing_table::node::Node::from_proto(request.get_ref().node.as_ref().unwrap());
    
        update_routing_table_with_node(
            &self.routing_table,
            node.get_id().clone(),
            node.get_ip(),
            node.get_port(),
        ).await;
    
        println!("Stored value with key: {:?}, from node with ID: {:?}",
            hex::encode(key),
            hex::encode(node.get_id())
        );
    
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
            let node = routing_table::node::Node::from_proto(request.get_ref().node.as_ref().unwrap());
    
            update_routing_table_with_node(
                &self.routing_table,
                node.get_id().clone(),
                node.get_ip(),
                node.get_port(),
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

    // Get the current node(and release the read lock after)
    let curr_node = {
        let routing_table_read = routing_table.read().await;
        routing_table_read.get_curr_node().to_proto()
    };

    // Send a ping to the bootstrap node
    let request = tonic::Request::new(PingRequest {
        node: Some(curr_node),
    });

    let response = client.ping(request).await?;

    // Update the routing table with the bootstrap node's ID
    let node = routing_table::node::Node::from_proto(response.get_ref().node.as_ref().unwrap());

    println!(
        "Ping response from bootstrap node with ID: {:?}, IP: {}, Port: {}",
        hex::encode(node.get_id()),
        node.get_ip(),
        node.get_port()
    );

    update_routing_table_with_node(
        &routing_table,
        *node.get_id(),
        node.get_ip(),
        node.get_port(),
    ).await;

    // Refresh the routing table
    refresh_routing_table(routing_table.clone()).await;

    Ok(())
}


async fn update_routing_table_with_node(routing_table: &RwLock<routing_table::RoutingTable>, id: [u8; 20], ip: String, port: u16,) {
    let new_node = routing_table::node::Node::with_id(id, ip, port);
    let mut routing_table = routing_table.write().await;
    routing_table.add_node(new_node);
    println!("Added node with ID to Table: {:?}", hex::encode(id));
}


pub async fn store_value_dht(
    routing_table: &RwLock<routing_table::RoutingTable>,
    key: [u8; 20],
    value: Vec<u8>,
) {
    // Use a single read lock to get the current node and K-closest nodes
    let (curr_node, k_closest) = {
        let routing_table = routing_table.read().await;
        (
            routing_table.get_curr_node().clone(),
            routing_table.get_closest_k_nodes(&key, MAX_BUCKET_SIZE),
        )
    };

    // Iterate over the k-closest nodes and send the value to each node
    for node in k_closest {
        // Create a new Kademlia client
        
        let uri = format!("http://[{}]:{}", node.get_ip(), node.get_port());
        let mut client = KademliaClient::connect(uri).await.unwrap();
            
        // Send the value to the node
        let request = tonic::Request::new(StoreRequest {
            node: Some(curr_node.to_proto()),
            key: key.to_vec(),
            value: value.clone(),
        });

        match client.store(request).await {
            Ok(response) => {
                println!("Stored value on node with ID: {:?}, Response: {:?}", hex::encode(node.get_id()), response.into_inner().message);
            }
            Err(e) => {
                println!("Failed to store value on node with ID: {:?}, Error: {:?}", hex::encode(node.get_id()), e);
            }
        }
    }
}

pub fn string_to_hash_key(key: &str) -> [u8; 20] {
    let mut context = Context::new(&SHA256);
    context.update(key.as_bytes());
    let result = context.finish();
    let mut hash_key = [0u8; 20];
    hash_key.copy_from_slice(&result.as_ref()[..20]);
    hash_key
}

pub async fn find_value_dht(
    routing_table: &RwLock<routing_table::RoutingTable>,
    key: [u8; 20],
) -> Option<Vec<u8>> {
    let key = Arc::new(key);
    let curr_node = Arc::new({
        let rt = routing_table.read().await;
        rt.get_curr_node().clone()
    });

    let mut queried_nodes = HashSet::new();
    let mut closest_nodes = {
        let rt = routing_table.read().await;
        rt.get_closest_k_nodes(&key, MAX_BUCKET_SIZE)
    };

    while !closest_nodes.is_empty() {
        let mut futures = vec![];

        for node in closest_nodes.iter().take(ALPHA) {
            if !queried_nodes.contains(node.get_id()) {
                let node = node.clone();
                let key = Arc::clone(&key);
                queried_nodes.insert(*node.get_id());

                let curr_node = Arc::clone(&curr_node);
                futures.push(tokio::spawn(async move {
                    let uri = format!("http://[{}]:{}", node.get_ip(), node.get_port());
                    let mut client = KademliaClient::connect(uri).await.ok()?;

                    let request = tonic::Request::new(FindValueRequest {
                        key: key.to_vec(),
                        node: Some(curr_node.to_proto()), 
                    });

                    client.find_value(request).await.ok()
                }));
            }
        }

        if futures.is_empty() {
            break;
        }

        let results = join_all(futures).await;

        let mut new_nodes = vec![];

        for result in results {
            if let Ok(Some(response)) = result {
                let response = response.into_inner();
                if !response.value.is_empty() {
                    return Some(response.value);
                } else {
                    let nodes = response.nodes;
                    for node_proto in nodes {
                        let node: Node = Node::from_proto(&node_proto);
                        let clone = node.clone();
                        new_nodes.push(node);
                        {
                            let mut rt = routing_table.write().await;
                            rt.add_node(clone);
                        }
                    }
                }
            }
        }

        if new_nodes.is_empty() {
            break;
        }

        new_nodes.sort_by_key(|node| distance(&key, node.get_id()));
        closest_nodes = new_nodes;
    }

    None
}

pub async fn refresh_bucket(
    routing_table: &RwLock<routing_table::RoutingTable>,
    bucket_index: u8,
) {
    // Get curr_node
    let curr_node = Arc::new({
        let rt = routing_table.read().await;
        rt.get_curr_node().clone()
    });

    let bucket = {
        let routing_table = routing_table.read().await;
        routing_table.get_k_bucket_map().get(&bucket_index).cloned()
    };



    if let Some(bucket) = bucket {
        if let Some(random_node) = bucket.get_random_node() {
            let random_id = routing_table::node_id::generate_random_id_near_distance(curr_node.get_id(), bucket_index);

            let uri = format!("http://[{}]:{}", random_node.get_ip(), random_node.get_port());
            if let Ok(mut client) = KademliaClient::connect(uri.clone()).await {
                let request = tonic::Request::new(FindNodeRequest {
                    key: random_id,
                    node: Some(curr_node.to_proto()), // Send the current node info
                });

                // Send the FindNodeRequest and handle the response
                match client.find_node(request).await {
                    Ok(response) => {
                        let nodes = response.into_inner().nodes;

                        // Lock the routing table to update nodes
                        let mut routing_table = routing_table.write().await;
                        for node_proto in nodes {
                            let node: Node = Node::from_proto(&node_proto);
                            routing_table.add_node(node);
                        }

                        println!("Refreshed bucket nº {}", bucket_index);
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to refresh bucket with node ID: {:?}, Error: {:?}",
                            hex::encode(random_node.get_id()),
                            e
                        );
                    }
                }
            } else {
                eprintln!("Failed to connect to node at {}", uri.clone());
            }
        } else {
            eprintln!("No random node found in bucket {}", bucket_index);
        }
    } else {
        eprintln!("Bucket {} not found", bucket_index);
    }
}


pub async fn refresh_routing_table(routing_table: Arc<RwLock<routing_table::RoutingTable>>) {
    let buckets = routing_table.read().await.get_k_bucket_map().clone();
    let mut futures = vec![];

    for (bucket_index, _bucket) in buckets {
        let routing_table_clone = Arc::clone(&routing_table);
        futures.push(tokio::spawn(async move {
            refresh_bucket(&routing_table_clone, bucket_index).await;
        }));
    }

    // Wait for all tasks to complete
    join_all(futures).await;
}
