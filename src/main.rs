extern crate core;

use std::sync::mpsc;
use libp2p::{identity, PeerId};
use crate::blockchain::{Block, BlockChain};
use crate::network::Network;
use crate::transaction::SignedTransaction;
use crate::wallet::Wallet;

pub mod transaction;
pub mod wallet;
pub mod blockchain;
pub mod network;

// https://bitcoin.org/bitcoin.pdf
#[tokio::main]
async fn main() {
    let strans_chan = mpsc::channel::<SignedTransaction>();
    let block_chan = mpsc::channel::<Block>();

    let wallet = Wallet::new(strans_chan.0.clone());

    let blockchain = BlockChain::new(wallet.get_public_key());
    let network = Network::new(blockchain);
}
