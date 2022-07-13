extern crate core;

use std::borrow::Borrow;
use std::sync::mpsc;
use libp2p::{identity, PeerId};
use libspartan::{Instance, SNARK, SNARKGens};
use crate::blockchain::{Block, BlockChain, hash_string, to_hash};
use crate::network::Network;
use crate::transaction::SignedTransaction;
use crate::wallet::Wallet;
use merlin::Transcript;

pub mod transaction;
pub mod wallet;
pub mod blockchain;
pub mod network;

// rustup default <toolchain>

// https://bitcoin.org/bitcoin.pdf
#[tokio::main]
async fn main() {
    let strans_chan = mpsc::channel::<SignedTransaction>();
    let block_chan = mpsc::channel::<Block>();

    let wallet = Wallet::new(strans_chan.0.clone());

    let blockchain = BlockChain::new(wallet.get_public_key());
    let network = Network::new(blockchain, block_chan);
}

#[tokio::test]
async fn snark_test() { // https://github.com/Microsoft/Spartan/ example
    // specify the size of an R1CS instance
    let num_vars = 1024;
    let num_cons = 1024;
    let num_inputs = 10;
    let num_non_zero_entries = 1024;

    // produce the public key and verification key
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);

    // ask the library to produce a synthentic R1CS instance
    let (inst, vars, inputs) = Instance::produce_synthetic_r1cs(num_cons, num_vars, num_inputs);

    // combine whatever the hell a R1SC instance is with the keys
    let (comm, decomm) = SNARK::encode(&inst, &gens);

    // The person who has the item that needs to be proved creates the SNARK for the verifier to look at
    // todo!();

    let mut prover_transcript = Transcript::new(b"TEST STRING"); // this is where the hash of the item goes
    let proof = SNARK::prove(&inst, &decomm, vars, &inputs, &gens, &mut prover_transcript);

    let mut verifier_transcript = Transcript::new(b"TEST STRING"); // hash of item goes here

    // need to be able to convert Vec<u8> to &'static [u8]

    // Verifier proves that prover has the item that produces that correspnding hash
    assert!(proof.verify(&comm, &inputs, &mut verifier_transcript, &gens).is_ok());
    println!("proof verification successful!");
}
