extern crate core;

use std::sync::mpsc;
use crate::wallet::Wallet;

pub mod transaction;
pub mod wallet;
pub mod blockchain;
pub mod network;

// https://bitcoin.org/bitcoin.pdf
#[tokio::main]
async fn main() {
    let chan = mpsc::channel();

    let wallet = Wallet::new();


}
