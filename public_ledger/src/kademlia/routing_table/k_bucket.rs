use std::collections::VecDeque;

use super::{node::Node, params::MAX_BUCKET_SIZE};

// K_Bucket is a struct that represents a bucket in the Kademlia DHT Tree
#[derive(Clone)]
pub(crate) struct K_Bucket{
    // k is the maximum number of nodes that a bucket can hold
    k: usize,
    // nodes is a vector queue that holds the nodes in the bucket, this structure holds recent used nodes at the front of the queue
    nodes: VecDeque<Node>,
}

impl K_Bucket {
    // Constructor
    pub fn new(k: usize) -> K_Bucket {
        K_Bucket {
            k: MAX_BUCKET_SIZE,
            nodes: VecDeque::new(),
        }
    }

    // Get the number of nodes in the bucket
    pub fn get_size(&self) -> usize {
        self.nodes.len()
    }

    // Add a node to the bucket
    pub fn add_node(&mut self, node: Node) {
        // Check If the node is already in the bucket
        if self.nodes.contains(&node) {
            // Move the node to the front of the bucket
            let index = self.nodes.iter().position(|x| *x == node).unwrap();
            self.nodes.remove(index);
            self.nodes.push_front(node);
        } else {
            // Check if the bucket is full
            if self.is_full() {
                // Remove the last node in the bucket
                self.nodes.pop_back();
            }
            // Add the node to the front of the bucket
            self.nodes.push_front(node);
        }
    }

    // Checks if the bucket is full
    pub fn is_full(&self) -> bool {
        self.nodes.len() == self.k
    }

    // Get the nodes in the bucket
    pub fn get_nodes(&self) -> &VecDeque<Node> {
        &self.nodes
    }

    pub fn get_random_node(&self) -> Option<Node> {
        if self.nodes.is_empty() {
            return None;
        }
        use rand::Rng;
        let random_index = rand::rng().random_range(0..self.nodes.len());
        Some(self.nodes[random_index].clone())
    }
    
}