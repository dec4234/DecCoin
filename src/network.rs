// https://github.com/libp2p/rust-libp2p/blob/master/examples/chat.rs

use crate::blockchain::{Block, BlockChain};
use anyhow::Result;

pub struct Network {
    blockchain: BlockChain,
}

impl Network {
    pub fn new(blockchain: BlockChain) -> Self {
        Self {
            blockchain,
        }
    }

    pub fn verify_block(block: &Block) -> Result<bool> {
        todo!()
    }
}