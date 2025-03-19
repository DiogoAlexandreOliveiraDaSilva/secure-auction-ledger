#[cfg(test)]
mod tests {
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use ring::rand::SystemRandom;
    use crate::blockchain::transaction::transaction::{Transaction, NodeId};

    /// ✅ Test generating a new NodeId
    #[test]
    fn test_generate_node_id() {
        let (node_id, _keypair) = NodeId::generate();
        assert!(!node_id.id.is_empty(), "Generated NodeId should not be empty");
    }

    /// ✅ Test signing and verifying a transaction
    #[test]
    fn test_transaction_sign_and_verify() {
        let (_rng, node_id) = NodeId::generate();
        let receiver_id = NodeId::from_public_key(b"receiver_public_key");

        let keypair = Ed25519KeyPair::from_pkcs8(&node_id.id.as_bytes()).unwrap();
        let transaction = Transaction::sign(&keypair, receiver_id.clone(), 100);

        assert!(transaction.verify_signature(), "Transaction signature verification failed!");
    }

    /// ❌ Test invalid transaction verification (signature mismatch)
    #[test]
    fn test_invalid_signature() {
        let (_rng, sender_id) = NodeId::generate();
        let receiver_id = NodeId::from_public_key(b"receiver_public_key");
    
        // ✅ Fix: Generate a second keypair instead of using "another_fake_key"
        let (_rng2, fake_sender_id) = NodeId::generate();
    
        let sender_key_bytes = base64::decode(&sender_id.id).unwrap();
        let fake_sender_key_bytes = base64::decode(&fake_sender_id.id).unwrap();
    
        let keypair1 = Ed25519KeyPair::from_pkcs8(&sender_key_bytes).unwrap();
        let keypair2 = Ed25519KeyPair::from_pkcs8(&fake_sender_key_bytes).unwrap();
    
        let mut transaction = Transaction::sign(&keypair1, receiver_id.clone(), 50);
    
        // Modify transaction manually to simulate tampering
        transaction.trans_signature = keypair2.sign(b"tampered data").as_ref().to_vec();
    
        assert!(!transaction.verify_signature(), "Tampered transaction should fail verification!");
    }   
}
