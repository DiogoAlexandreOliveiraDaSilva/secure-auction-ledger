use kademlia::init;

// Tests
#[cfg(test)]
mod tests;
// Imports
mod blockchain;
mod kademlia;


fn main() {
    init(8080);
}
