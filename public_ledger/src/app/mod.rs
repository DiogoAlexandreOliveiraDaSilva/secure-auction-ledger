use crate::auction;
// main.rs
use crate::blockchain;
use crate::blockchain::block::Block;
use crate::blockchain::chain::Chain;
use crate::kademlia;
use crate::kademlia::find_value_dht;
use crate::kademlia::store_value_dht;
use crate::kademlia::string_to_hash_key;
use crate::routing_table::{self, RoutingTable};
use eframe::{App, Frame, egui};
use screens::auction_screen::AuctionScreenEvent;
use screens::block_screen::BlockScreen;
use screens::join_screen::JoinScreen;
use screens::menu_screen::MenuScreen;
use screens::menu_screen::MenuScreenEvent;

use crate::auction::Auction;

use std::result;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::sync::oneshot;

mod screens;
use screens::AppState;
use screens::auction_screen::AuctionScreen;
use screens::create_screen::CreateScreen;
use screens::initial_screen::{InitialScreen, InitialScreenEvent};
use screens::join_screen::JoinScreenEvent;
use screens::selection_screen::{SelectionScreen, SelectionScreenEvent};

pub struct AuctionApp {
    pub(crate) state: AppState,
    routing_table: Option<Arc<RwLock<routing_table::RoutingTable>>>,
    initial_screen: InitialScreen,
    selection_screen: SelectionScreen,
    join_screen: JoinScreen,
    menu_screen: MenuScreen,
    auction_screen: AuctionScreen,
    create_screen: CreateScreen,
    block_screen: BlockScreen,
    result_string: Arc<Mutex<String>>,
    latest_auction: Arc<Mutex<Auction>>,
    auction_list: Arc<Mutex<Vec<Auction>>>,
    blockchain: Arc<Mutex<blockchain::chain::Chain>>,
}

impl AuctionApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Initial,
            initial_screen: InitialScreen::default(),
            routing_table: None,
            selection_screen: SelectionScreen::default(),
            join_screen: JoinScreen::default(),
            menu_screen: MenuScreen::default(),
            auction_screen: AuctionScreen::default(),
            create_screen: CreateScreen::default(),
            block_screen: BlockScreen::default(),
            result_string: Arc::new(Mutex::new("".to_string())),
            latest_auction: Arc::new(Mutex::new(Auction::default())),
            auction_list: Arc::new(Mutex::new(Vec::new())),
            blockchain: Arc::new(Mutex::new(blockchain::chain::Chain::new())),
        }
    }

    pub fn update_menu_screen_info(&mut self) {
        if let Some(rt) = &self.routing_table {
            if let Ok(rt_read) = rt.try_read() {
                let node = rt_read.get_curr_node().clone();
                self.menu_screen.set_info(
                    node.get_ip(),
                    node.get_port().to_string(),
                    node.get_id().to_vec(),
                );
            }
        }
    }
}

impl App for AuctionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::Initial => {
                    if let Some(event) = self.initial_screen.ui(ui) {
                        match event {
                            InitialScreenEvent::Submitted(port) => {
                                let addr = "::1".to_string();
                                self.routing_table = Some(Arc::new(RwLock::new(
                                    routing_table::RoutingTable::new(addr.clone(), port),
                                )));
                                if let Some(routing_table) = self.routing_table.clone() {
                                    let routing_table_clone = routing_table.clone();
                                    let addr_clone = addr.clone();
                                    tokio::spawn({
                                        let addr_clone = addr_clone.clone();
                                        async move {
                                            if let Err(e) = kademlia::start_kademlia_server(
                                                routing_table_clone,
                                                addr_clone.clone(),
                                                port,
                                            )
                                            .await
                                            {
                                                eprintln!("Failed to start Kademlia server: {}", e);
                                            }
                                        }
                                    });
                                    println!("Kademlia server started on {}:{}", addr, port);
                                    self.state = AppState::Selection;
                                } else {
                                    println!("Routing table is not initialized");
                                    self.state = AppState::Initial;
                                }
                            }
                        }
                    }
                }
                AppState::Selection => {
                    if let Some(event) = self.selection_screen.ui(ui) {
                        match event {
                            SelectionScreenEvent::Join => {
                                self.state = AppState::Join;
                            }
                            SelectionScreenEvent::Create => {
                                let blockchain = self.blockchain.clone();
                                let routing_table = self.routing_table.clone().unwrap();

                                tokio::spawn(async move {
                                    // Add Genesis Block
                                    let genesis_block = blockchain::block::Block::genesis();

                                    {
                                        let mut blockchain = blockchain.lock().await;
                                        blockchain.add_block(genesis_block.clone());
                                    }

                                    let clone_genesis_block = genesis_block.clone();

                                    // Store Genesis Block under its truncated hash
                                    let truncated_hash = &genesis_block.get_hash()[0..20];
                                    let block_dht_key =
                                        kademlia::string_to_hash_key(&hex::encode(truncated_hash));

                                    let genesis_serialized =
                                        genesis_block.serialized().as_bytes().to_vec();

                                    let routing_table_clone = routing_table.clone();
                                    store_value_dht(
                                        &routing_table_clone,
                                        block_dht_key,
                                        genesis_serialized,
                                    )
                                    .await;
                                    println!(
                                        "Genesis block stored under key {:?}",
                                        hex::encode(truncated_hash)
                                    );

                                    // Store Latest Block pointer
                                    let latest_block_key =
                                        kademlia::string_to_hash_key("latest_block");

                                    let blockchain = blockchain.lock().await;

                                    store_value_dht(
                                        &routing_table,
                                        latest_block_key,
                                        clone_genesis_block.serialized().as_bytes().to_vec(),
                                    )
                                    .await;
                                    println!("Latest block updated");
                                });

                                self.update_menu_screen_info();
                                self.state = AppState::Menu;
                            }
                        }
                    }
                }
                AppState::Join => {
                    if let Some(event) = self.join_screen.ui(ui) {
                        match event {
                            JoinScreenEvent::Submitted(port) => {
                                let routing_table = self.routing_table.clone().unwrap();
                                let addr = "::1".to_string();
                                tokio::spawn(async move {
                                    if let Err(e) =
                                        kademlia::join_kademlia_network(routing_table, addr, port)
                                            .await
                                    {
                                        eprintln!("Failed to join Kademlia network: {}", e);
                                    }
                                });

                                self.update_menu_screen_info();
                                self.state = AppState::Menu;
                            }
                            JoinScreenEvent::Back => {
                                self.state = AppState::Selection;
                            }
                        }
                    }
                }
                AppState::Menu => {
                    let result_string = self.result_string.try_lock().unwrap(); // Lock to read the result string
                    self.menu_screen.set_search_value(result_string.clone());
                    if let Some(event) = self.menu_screen.ui(ui) {
                        match event {
                            MenuScreenEvent::SubmittedStore { key, value } => {
                                let routing_table = self.routing_table.clone().unwrap();
                                let routing_table_clone = routing_table.clone();
                                let hash = kademlia::string_to_hash_key(&key.clone());
                                tokio::spawn(async move {
                                    store_value_dht(
                                        &routing_table_clone,
                                        hash,
                                        value.as_bytes().to_vec(),
                                    )
                                    .await;
                                });
                            }
                            MenuScreenEvent::SubmittedSearch { key } => {
                                let routing_table = self.routing_table.clone().unwrap();
                                let result_string = self.result_string.clone(); // Clone Arc for async task
                                tokio::spawn({
                                    let routing_table = routing_table.clone(); // Clone for async task
                                    let result_string: Arc<Mutex<String>> =
                                        Arc::clone(&result_string); // Clone Arc for async task

                                    async move {
                                        let hash = kademlia::string_to_hash_key(&key);
                                        let value = find_value_dht(&routing_table, hash).await;
                                        let mut result_string = result_string.lock().await; // Lock to update the result string

                                        match value {
                                            Some(bytes) => {
                                                *result_string = format!(
                                                    "{}",
                                                    from_utf8(&bytes).unwrap_or("Invalid UTF-8")
                                                );
                                                println!("Value found: {:?}", from_utf8(&bytes)); // Debugging
                                            }
                                            None => {
                                                *result_string = "Value not found".to_string();
                                                println!("Value not found"); // Debugging
                                            }
                                        }
                                    }
                                });
                            }
                            MenuScreenEvent::Auction => {
                                self.state = AppState::Auction;
                            }
                            MenuScreenEvent::Block => {
                                self.state = AppState::Block;
                            }
                        }
                    }
                }
                AppState::Auction => {
                    let auction_list = self.auction_list.try_lock().unwrap(); // Lock to read the auction list
                    self.auction_screen.refresh_auctions(auction_list.clone());

                    let chain = self.blockchain.try_lock().unwrap().clone();
                    self.auction_screen.set_chain(chain);

                    if let Some(event) = self.auction_screen.ui(ui) {
                        match event {
                            AuctionScreenEvent::Create => {
                                self.state = AppState::Create;
                            }
                            AuctionScreenEvent::Back => {
                                self.state = AppState::Menu;
                            }
                            AuctionScreenEvent::GetAuctions => {
                                // Get latest auction id
                                let routing_table = self.routing_table.clone().unwrap();
                                let latest_auction = self.latest_auction.clone();
                                tokio::spawn({
                                    let routing_table = routing_table.clone(); // Clone for async task
                                    let latest_auction = latest_auction.clone(); // Clone for async task

                                    async move {
                                        let hash = kademlia::string_to_hash_key("last_auction"); // Predefined key
                                        let value = find_value_dht(&routing_table, hash).await;
                                        let mut latest_auction = latest_auction.lock().await; // Lock to update the result string

                                        match value {
                                            Some(bytes) => {
                                                *latest_auction = Auction::deserialized(
                                                    from_utf8(&bytes).unwrap(),
                                                );
                                                println!("Auction found: {:?}", latest_auction); // Debugging
                                            }
                                            None => {
                                                println!("Auction not found"); // Debugging
                                            }
                                        }
                                    }
                                });

                                // Get all auctions and verify
                                let routing_table = self.routing_table.clone().unwrap();
                                let auction_list = self.auction_list.clone();
                                tokio::spawn({
                                    let routing_table = routing_table.clone(); // Clone for async task
                                    let auction_list = auction_list.clone(); // Clone for async task

                                    async move {
                                        let mut auctions = Vec::new();
                                        for i in 0..=latest_auction.lock().await.id {
                                            let hash = kademlia::string_to_hash_key(
                                                &("auction:".to_string() + &i.to_string()),
                                            );
                                            let value = find_value_dht(&routing_table, hash).await;
                                            match value {
                                                Some(bytes) => {
                                                    let auction = Auction::deserialized(
                                                        from_utf8(&bytes).unwrap(),
                                                    );
                                                    auctions.push(auction);
                                                }
                                                None => {
                                                    println!("Auction not found"); // Debugging
                                                }
                                            }
                                        }
                                        let mut auction_list = auction_list.lock().await; // Lock to update the result string
                                        *auction_list = auctions;
                                    }
                                });

                                // Get Chain
                                let routing_table = self.routing_table.clone().unwrap();
                                let blockchain = self.blockchain.clone();
                                let routing_table_clone = routing_table.clone();
                                tokio::spawn(async move {
                                    if let Some(fetched_chain) =
                                        fetch_full_chain(&routing_table_clone).await
                                    {
                                        let mut blockchain = blockchain.lock().await;
                                        *blockchain = fetched_chain;
                                        println!("Chain fetched successfully");
                                    } else {
                                        println!("Failed to fetch chain");
                                    }
                                });
                            }
                        }
                    }
                }
                AppState::Create => {
                    if let Some(event) = self.create_screen.ui(ui) {
                        match event {
                            screens::create_screen::CreateScreenEvent::Back => {
                                self.state = AppState::Auction;
                            }
                            screens::create_screen::CreateScreenEvent::Submitted(
                                item_name,
                                starting_price,
                                duration_hours,
                            ) => {
                                // Get Last Auction ID
                                let routing_table = self.routing_table.clone().unwrap();
                                let latest_auction = self.latest_auction.clone();
                                tokio::spawn({
                                    let routing_table = routing_table.clone(); // Clone for async task
                                    let latest_auction = latest_auction.clone(); // Clone for async task

                                    async move {
                                        let hash = kademlia::string_to_hash_key("last_auction"); // Predefined key
                                        let value = find_value_dht(&routing_table, hash).await;
                                        let mut latest_auction = latest_auction.lock().await; // Lock to update the result string

                                        match value {
                                            Some(bytes) => {
                                                *latest_auction = Auction::deserialized(
                                                    from_utf8(&bytes).unwrap(),
                                                );
                                                println!("Auction found: {:?}", latest_auction); // Debugging
                                            }
                                            None => {
                                                println!("Auction not found"); // Debugging
                                            }
                                        }
                                    }
                                });

                                // Increment Auction ID
                                let latest_auction = self.latest_auction.clone();
                                let auction_id = tokio::task::block_in_place(|| {
                                    let rt = tokio::runtime::Handle::current();
                                    rt.block_on(async {
                                        let mut latest_auction = latest_auction.lock().await; // Lock to update the result string
                                        latest_auction.id.clone() + 1
                                    })
                                });

                                // Create Auction
                                let auction = Auction::new_with_duration(
                                    auction_id,
                                    item_name,
                                    starting_price,
                                    duration_hours,
                                );

                                let auction_hash = auction.get_hash();

                                // Store Auction
                                let routing_table = self.routing_table.clone().unwrap();
                                let routing_table_clone = routing_table.clone();
                                let hash = kademlia::string_to_hash_key(
                                    &("auction:".to_string() + &auction_id.to_string()),
                                );
                                let auction_clone = auction.clone();
                                tokio::spawn(async move {
                                    store_value_dht(
                                        &routing_table_clone,
                                        hash,
                                        auction_clone.serialized().as_bytes().to_vec(),
                                    )
                                    .await;
                                });

                                // Update Latest Auction
                                let routing_table = self.routing_table.clone().unwrap();
                                let routing_table_clone = routing_table.clone();
                                let hash =
                                    kademlia::string_to_hash_key(&("last_auction".to_string()));
                                tokio::spawn(async move {
                                    store_value_dht(
                                        &routing_table_clone,
                                        hash,
                                        auction.serialized().as_bytes().to_vec(),
                                    )
                                    .await;
                                });

                                // Create Auction Signature
                                let auction_signature = auction::signature::AuctionSignature::new(
                                    auction_id.to_string(),
                                    auction_hash,
                                );

                                // Create Block with Auction Signature as transaction
                                let routing_table = self.routing_table.clone().unwrap();
                                let blockchain = self.blockchain.clone();
                                let routing_table_clone = routing_table.clone();
                                let auction_signature_clone = auction_signature.clone();
                                tokio::spawn(async move {
                                    //Fetch the latest chain
                                    if let Some(fetched_chain) =
                                        fetch_full_chain(&routing_table_clone).await
                                    {
                                        let mut blockchain_lock = blockchain.lock().await;
                                        *blockchain_lock = fetched_chain;
                                        println!("Chain fetched successfully");
                                    } else {
                                        println!("Failed to fetch chain, aborting mine");
                                        return;
                                    }

                                    //Prepare the new block
                                    let mut blockchain_lock = blockchain.lock().await;
                                    let last_block_hash =
                                        blockchain_lock.get_first_block().get_hash();

                                    let header = blockchain::block::block_header::BlockHeader::new(
                                        last_block_hash.into(),
                                    );
                                    let body = blockchain::block::block_body::BlockBody::new(
                                        auction_signature_clone.serialized_to_bytes().unwrap(),
                                    );
                                    let mut block = blockchain::block::Block::new(header, body);

                                    // Mine the block
                                    block.mine();

                                    drop(blockchain_lock); // Release lock before DHT operations

                                    // Store the block in the DHT under its truncated hash
                                    let truncated_hash = &block.get_hash()[0..20]; // First 20 bytes
                                    let block_dht_key =
                                        kademlia::string_to_hash_key(&hex::encode(truncated_hash));

                                    store_value_dht(
                                        &routing_table,
                                        block_dht_key,
                                        block.serialized().as_bytes().to_vec(),
                                    )
                                    .await;

                                    println!(
                                        "Block stored under key {:?}",
                                        hex::encode(truncated_hash)
                                    );

                                    // Update 'latest_block' pointer in DHT
                                    let latest_block_key =
                                        kademlia::string_to_hash_key("latest_block");

                                    store_value_dht(
                                        &routing_table,
                                        latest_block_key,
                                        block.serialized().as_bytes().to_vec(),
                                    )
                                    .await;

                                    println!("Latest block updated");
                                });

                                // Change state to Auction
                                self.state = AppState::Auction;
                            }
                        }
                    }
                }
                AppState::Block => {
                    if let Some(event) = self.block_screen.ui(ui) {
                        match event {
                            screens::block_screen::BlockScreenEvent::Back => {
                                self.state = AppState::Menu;
                            }
                            screens::block_screen::BlockScreenEvent::GetChain => {
                                let chain = tokio::task::block_in_place(|| {
                                    let rt = tokio::runtime::Handle::current();
                                    rt.block_on(self.blockchain.lock()).clone()
                                });
                                self.block_screen.refresh_chain(chain);
                                println!("Getting chain");
                                let current_chain = blockchain::chain::Chain::new();

                                // Fetch the full chain
                                let routing_table = self.routing_table.clone().unwrap();
                                let routing_table_clone = routing_table.clone();
                                let blockchain_clone = self.blockchain.clone();
                                tokio::spawn(async move {
                                    if let Some(chain) =
                                        fetch_full_chain(&routing_table_clone).await
                                    {
                                        let mut blockchain = blockchain_clone.lock().await;
                                        *blockchain = chain;
                                        println!("Chain fetched successfully");
                                    } else {
                                        println!("Failed to fetch chain");
                                    }
                                });
                            }
                            screens::block_screen::BlockScreenEvent::MineBlock { transaction } => {
                                let routing_table = self.routing_table.clone().unwrap();
                                let blockchain = self.blockchain.clone();

                                tokio::spawn(async move {
                                    //Fetch the latest chain
                                    if let Some(fetched_chain) =
                                        fetch_full_chain(&routing_table).await
                                    {
                                        let mut blockchain_lock = blockchain.lock().await;
                                        *blockchain_lock = fetched_chain;
                                        println!("Chain fetched successfully");
                                    } else {
                                        println!("Failed to fetch chain, aborting mine");
                                        return;
                                    }

                                    //Prepare the new block
                                    let mut blockchain_lock = blockchain.lock().await;
                                    let last_block_hash =
                                        blockchain_lock.get_first_block().get_hash();

                                    let header = blockchain::block::block_header::BlockHeader::new(
                                        last_block_hash.into(),
                                    );
                                    let body = blockchain::block::block_body::BlockBody::new(
                                        transaction.as_bytes().to_vec(),
                                    );
                                    let mut block = blockchain::block::Block::new(header, body);

                                    // Mine the block
                                    block.mine();

                                    drop(blockchain_lock); // Release lock before DHT operations

                                    // Store the block in the DHT under its truncated hash
                                    let truncated_hash = &block.get_hash()[0..20]; // First 20 bytes
                                    let block_dht_key =
                                        kademlia::string_to_hash_key(&hex::encode(truncated_hash));

                                    store_value_dht(
                                        &routing_table,
                                        block_dht_key,
                                        block.serialized().as_bytes().to_vec(),
                                    )
                                    .await;

                                    println!(
                                        "Block stored under key {:?}",
                                        hex::encode(truncated_hash)
                                    );

                                    // Update 'latest_block' pointer in DHT
                                    let latest_block_key =
                                        kademlia::string_to_hash_key("latest_block");

                                    store_value_dht(
                                        &routing_table,
                                        latest_block_key,
                                        block.serialized().as_bytes().to_vec(),
                                    )
                                    .await;

                                    println!("Latest block updated");
                                });
                            }
                        }
                    }
                }
            }
        });
    }
}

pub async fn fetch_full_chain(routing_table: &RwLock<RoutingTable>) -> Option<Chain> {
    let mut chain = Chain::new();
    let mut current_hash = string_to_hash_key("latest_block");

    loop {
        // Fetch the block from DHT
        let value = find_value_dht(routing_table, current_hash).await;

        match value {
            Some(bytes) => {
                let block_str = std::str::from_utf8(&bytes).ok()?;
                let block = Block::deserialized(block_str);

                // Add block to the chain
                chain.add_block(block.clone());

                // If we hit the genesis block, we stop
                if block.header.get_parent_hash() == vec![0; 64] {
                    println!("Genesis block reached");
                    break;
                }

                // Move to the previous block
                current_hash = {
                    let mut hash = block.header.get_parent_hash();
                    hash.truncate(20); // Truncate to 20 bytes
                    string_to_hash_key(&hex::encode(hash))
                };
            }
            None => {
                println!("Block not found for hash {:?}", hex::encode(current_hash));
                return None; // Early exit if something is broken
            }
        }
    }

    Some(chain)
}
