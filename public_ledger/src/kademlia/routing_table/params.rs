// MAX_BUCKET_SIZE is the maximum number of nodes that can be stored in a single bucket.
// This is a constant that can be adjusted based on the expected size of the network and the desired performance characteristics.
pub const MAX_BUCKET_SIZE: usize = 4;

// ALPHA is the number of nodes that will be contacted in parallel when searching for a node.
// If higher, it will increase the load on the network, but may also speed up the search.
// If lower, it will reduce the load on the network, but may slow down the search.
pub const ALPHA: usize = 3;
