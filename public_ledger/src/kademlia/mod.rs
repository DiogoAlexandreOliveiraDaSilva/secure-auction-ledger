pub(crate) mod routing_table;

pub fn init(port: u32) {
    let ip = "127.0.0.1"; // TODO:Change to function that get's local ip
    let curr_table = routing_table::RoutingTable::new(ip.to_string(), port as u16);
    println!("Node created with id: {:x?}, ip: {}, port: {}", curr_table.get_curr_node().get_id(), ip, port);
}

