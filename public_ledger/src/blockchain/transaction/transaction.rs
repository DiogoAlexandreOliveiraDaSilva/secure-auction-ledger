use std::time::{SystemTime, UNIX_EPOCH};
use ring::{rand::SystemRandom, signature::{Ed25519KeyPair, KeyPair, Signature,UnparsedPublicKey, ED25519}};
use base64::{engine::general_purpose, Engine};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: NodeId,
    pub receiver: NodeId,
    pub amount: u64,
    pub trans_signature: Vec<u8>,
    pub timestamp: u64,
}

impl Transaction{
    pub fn new(
        sender: NodeId,
        receiver: NodeId,
        amount: u64,
        trans_signature: Signature,
    ) -> Transaction {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        Transaction{
            sender,
            receiver,
            amount,
            trans_signature: trans_signature.as_ref().to_vec(),
            timestamp,
        }
    }

    pub fn sign(sender_keypair: &Ed25519KeyPair, receiver: NodeId, amount: u64) -> Self {
         
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(); 
        let message = format!(
            "{}:{}:{}:{}",
            general_purpose::STANDARD.encode(sender_keypair.public_key().as_ref()),
            receiver.to_string(),
            amount,
            timestamp
        );

        let signature = sender_keypair.sign(message.as_bytes());

        Transaction {
            sender: NodeId::from_public_key(sender_keypair.public_key().as_ref()),
            receiver,
            amount,
            trans_signature: signature.as_ref().to_vec(),
            timestamp,
        }
    }
    
    pub fn verify_signature(&self) -> bool {
        let message = format!(
            "{}:{}:{}:{}",
            self.sender.to_string(),
            self.receiver.to_string(),
            self.amount,
            self.timestamp
        );

        let pub_key_bytes = general_purpose::STANDARD.decode(&self.sender.to_string()).unwrap();
        let peer_public_key = UnparsedPublicKey::new(&ED25519, pub_key_bytes);

        peer_public_key.verify(message.as_bytes(), self.trans_signature.as_ref()).is_ok()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeId {
    pub pub_id: String,
}

impl NodeId {
    pub fn from_public_key(pk: &[u8]) -> Self {
        NodeId {
            pub_id: general_purpose::STANDARD.encode(pk),
        }
    }

    pub fn to_string(&self) -> String {
        self.pub_id.clone()
    }

    pub fn generate() -> (Self, Vec<u8>) {
        let rng = SystemRandom::new();
        let keypair_pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let keypair = Ed25519KeyPair::from_pkcs8(keypair_pkcs8.as_ref()).unwrap();
        let node_id = NodeId::from_public_key(keypair.public_key().as_ref());

        (node_id, keypair_pkcs8.as_ref().to_vec())
    }
}