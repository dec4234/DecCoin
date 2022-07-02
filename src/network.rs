// https://github.com/libp2p/rust-libp2p/blob/master/examples/chat.rs

use std::io;
use std::io::BufRead;
use std::sync::mpsc::{Receiver, Sender};
use crate::blockchain::{Block, BlockChain};
use anyhow::{anyhow, Result};
use ed25519_dalek::PublicKey;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::SwarmEvent,
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};
use libp2p::futures::select;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::task;
use crate::transaction::SignedTransaction;

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519() );
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()) );

pub struct Network {
    blockchain: Mutex<BlockChain>,
    pub chan: (Sender<Block>, Receiver<Block>),
}

impl Network {
    pub fn new(blockchain: BlockChain, chan: (Sender<Block>, Receiver<Block>)) -> Self {
        Self {
            blockchain: Mutex::new(blockchain),
            chan,
        }
    }

    pub async fn verify_block(&mut self, block: &Block) -> Result<()> {

        // Make sure that the hash of the block is mined correctly
        if !block.is_mined() {
            return Err(anyhow!("Block has not been mined."));
        }

        // Make sure hash of previous block is correct
        if let Some(last) = self.blockchain.lock().await.blocks.last() {
            if block.prev_hash != last.hash_of() {
                return Err(anyhow!("Block prev_hash does not match hash of previous block"));
            }
        }

        // Validate the block's transactions
        for trans in block.transactions.as_slice() {
            if !trans.verify_signature() {
                return Err(anyhow!("Invalid Signature: {}", trans.trans));
            }

            self.blockchain.lock().await.verify_transaction(&trans.trans)?;
        }


        Ok(())
    }

    /// Take incoming transactions and combine them together in a new block.
    /// Adds mined blocks to the end of the blockchain and broadcasts them to the network.
    pub async fn create_new_blocks(&mut self, recv: Receiver<SignedTransaction>, send: Sender<Block>, miner_public: PublicKey) -> Result<()> {
        const TRANSACTIONS_PER_BLOCK: u8 = 1;

        loop {
            let mut vec = Vec::new();

            while vec.len() < TRANSACTIONS_PER_BLOCK as usize {
                let incoming = recv.recv()?;

                if incoming.verify_signature() && self.blockchain.lock().await.verify_transaction(&incoming.trans).is_ok() {
                    vec.push(incoming);
                }
            }

            let mut block = Block::new(vec, self.blockchain.lock().await.hash_of_last(), miner_public.clone())?;

            block.mine()?;

            send.send(block.clone())?;

            self.blockchain.add_verified_block(block);
        }
    }
}