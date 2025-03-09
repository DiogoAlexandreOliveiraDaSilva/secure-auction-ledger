//! Block structure
mod block_header;
mod block_body;

struct Block {
    header: block_header::BlockHeader,
    body: block_body::BlockBody
}


