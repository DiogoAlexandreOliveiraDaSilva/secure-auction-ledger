// Methods related to node_id manipulation

use rand::Rng;

// Generates a random node id
pub fn generate_node_id() -> [u8; 20] {
    let mut rng = rand::rng();
    let mut id = [0u8; 20];
    rng.fill(&mut id);
    id
}

// Calculates the distance between two node ids by XORing them
pub fn distance(node_id1: &[u8; 20], node_id2: &[u8; 20]) -> [u8; 20] {
    let mut dist = [0u8; 20];
    for i in 0..20 {
        dist[i] = node_id1[i] ^ node_id2[i];
    }
    dist
}

// K-Index function: returns the value of the k-th bit of the XOR distance
pub fn k_index(xor_distance: &[u8; 20], bit_index: usize) -> u8 {
    let byte_index = bit_index / 8;
    let bit_in_byte = bit_index % 8;
    (xor_distance[byte_index] >> (7 - bit_in_byte)) & 1
}

// Calculates the k-bucket index of a node id, starts at 0
pub fn find_k_bucket_index(xor_distance: &[u8; 20]) -> usize {
    for bit_index in 0..160 {
        // Check if the bit at `bit_index` is 1 (use k_index to check)
        if k_index(xor_distance, bit_index) == 1 {
            return bit_index;  // Return the index of the first '1' bit
        }
    }
    160  // If no bit is '1', return 160 (indicating no bucket, should not happen)
}

pub fn generate_random_id_near_distance(my_id: &[u8], bucket_index: u8) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut random_id = my_id.to_vec();

    // Calculate which bit to flip: the bit at position (ID length * 8 - 1 - bucket_index)
    let bit_pos = (random_id.len() * 8 - 1) - bucket_index as usize;
    let byte_pos = bit_pos / 8;
    let bit_in_byte = bit_pos % 8;

    // Flip that bit
    if byte_pos < random_id.len() {
        random_id[byte_pos] ^= 1 << bit_in_byte;
    }

    // Randomize bits *after* the flipped bit to make it more "random" in that region
    for i in (byte_pos + 1)..random_id.len() {
        random_id[i] = rng.random();
    }

    random_id
}
