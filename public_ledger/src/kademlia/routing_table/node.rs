use crate::kademlia::communication;

// Node of Kademlia DHT Tree
use super::node_id::generate_node_id;

#[derive(Clone)]
pub(crate) struct Node {
  
    // Unique Identifier of the Node / TODO: Change to Key Struct
    id: [u8; 20], // 160-bit identifier
    // IP Address of the Node
    ip: String,
    // PORT of the Node
    port: u16,   
  }
  
  impl Node {
    // Constructor
    pub fn new(ip: String, port: u16) -> Node {
      Node {
      id: generate_node_id(),
      ip,
      port,
      }
    }

    pub fn with_id(id: [u8; 20], ip: String, port: u16) -> Node {
      Node {
      id,
      ip,
      port,
      }
    }

    // Get the Clone of the ID of the Node
    pub fn get_id(&self) -> &[u8; 20] {
      &self.id
    }

    // Get the IP Address of the Node
    pub fn get_ip(&self) -> String {
      self.ip.clone()
    }

    // Get the PORT of the Node
    pub fn get_port(&self) -> u16 {
      self.port
    }

    pub fn to_proto(&self) -> communication::Node {
      communication::Node {
          id: self.id.to_vec(),
          ip: self.ip.clone(),
          port: self.port as u32,
      }
    }

    pub fn from_proto(proto: &communication::Node) -> Node {
      Node {
          id: proto.id.clone().try_into().expect("Invalid ID length"),
          ip: proto.ip.clone(),
          port: proto.port as u16,
      }
    }

  }

  impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
      self.id == other.id
    }
  }
