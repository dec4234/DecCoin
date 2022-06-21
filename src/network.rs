// https://github.com/libp2p/rust-libp2p/blob/master/examples/chat.rs

use crate::blockchain::{Block, BlockChain};
use anyhow::{anyhow, Result};

pub struct Network {
    blockchain: BlockChain,
}

impl Network {
    pub fn new(blockchain: BlockChain) -> Self {
        Self {
            blockchain,
        }
    }

    pub fn verify_block(&mut self, block: &Block) -> Result<()> {

        // Make sure that the hash of the block is mined correctly
        if !block.is_mined() {
            return Err(anyhow!("Block has not been mined."));
        }

        // Make sure hash of previous block is correct
        if let Some(last) = self.blockchain.blocks.last() {
            if block.prev_hash != last.hash_of() {
                return Err(anyhow!("Block prev_hash does not match hash of previous block"));
            }
        }

        for trans in block.transactions.as_slice() {
            if !trans.is_valid() {
                return Err(anyhow!("Invalid: {}", trans.trans));
            }
        }


        Ok(())
    }
}