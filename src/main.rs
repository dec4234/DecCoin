extern crate core;

use std::sync::mpsc;
use crate::blockchain::Block;
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

    let wallet = Wallet::new();


}
