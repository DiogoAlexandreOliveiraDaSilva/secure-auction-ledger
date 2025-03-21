mod routing_table;

pub fn start(port: u32) {
    let ip = "127.0.0.1"; // TODO:Change to function that get's local ip
    let curr_node = routing_table::RoutingTable::new("".to_string(), ip.to_string(), port as u16);
}