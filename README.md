# Public Ledger for Auctions

This project was developed in the context of the **Master in Segurança Informática (MSI)** at the **Faculty of Sciences of the University of Porto (FCUP)**. It was part of the final assignment for the **Secure and Distributed Systems (SSD)** course, under the supervision of **Prof. Rolando da Silva Martins**.

The goal of the project is to design and implement a secure, decentralized auction system based on a blockchain and a peer-to-peer (P2P) network. It explores concepts in distributed systems, security, cryptography, and consensus protocols.

## Overview

The system is built around a custom blockchain and a secure P2P network based on SKademlia. It supports:

- A proof-of-work (PoW) consensus mechanism
- Distributed storage using a Kademlia DHT
- Real-time auction creation and bidding
- Cryptographic integrity and authenticity of bids and auctions
- A GUI for user interaction and network monitoring

## Features

### Blockchain
- Custom block structure with header and body
- SHA-512 based PoW hashing
- Adjustable difficulty for mining
- Thread-safe local storage
- Genesis block creation and mining logic

### Distributed Network
- Secure P2P network using SKademlia
- Node IDs derived from cryptographic public keys
- XOR-based distance metric and routing table
- gRPC-based communication (ping, store, find_node, find_value)
- Protection against Sybil and eclipse attacks

### Auction System
- Auctions and bids stored immutably in the blockchain
- Timestamp and value verification
- Cryptographic signatures for bid authenticity
- Key-based hierarchical storage (`auction:<id>`, `auction:<id>:bid:<x>`)
- Real-time updates using a publisher/subscriber model

### Technical Stack

- Language: Rust
- Crypto: `ring`, SHA-256, Ed25519
- Networking: `tonic`, `tokio`, gRPC
- GUI: `eframe`, `egui`
- Serialization: `serde`, `serde_json`
- Randomness: `rand`

## Authors

- Diogo Silva — up202105327  
- Leandro Costa — up202408816

## Supervisor

- Prof. Rolando da Silva Martins

## Course

- Secure and Distributed Systems (SSD)  
- Master in Segurança Informática  
- Faculdade de Ciências da Universidade do Porto (FCUP)  
- May 2025

