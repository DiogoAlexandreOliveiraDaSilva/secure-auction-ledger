// Node of Kademlia DHT Tree
pub(crate) struct Node {
    // Unique Identifier of the Node / TODO: Change to Key Struct
    id: String,
    // IP Address of the Node
    ip: String,
    // PORT of the Node
    port: u16,   
  }
  
  impl Node {
    // Constructor
    pub fn new(id: String, ip: String, port: u16) -> Node {
      Node {
        id,
        ip,
        port,
      }
    }
  }