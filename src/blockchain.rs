use std::hash::Hash;
use std::ptr::hash;
use anyhow::anyhow;
use bincode::Options;
use crate::transaction::{SignedTransaction, Transaction, verify};
use sha2::{Sha256, Digest};
use sha2::digest::Update;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use ed25519_dalek::PublicKey;

// Reduced to save time
const ZEROES_NEEDED: u8 = 3; // The number of leading zeroes needed in a hash to successfully mine a block (Normally 19)

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub init_key: PublicKey,
}

impl BlockChain {
    pub fn new(init_key: PublicKey) -> Self {
        let blocks = Vec::new();



        Self {
            blocks,
            init_key,
        }
    }

    /// Returns the current balance of the user with the following
    /// public key. Replays all transactions in the block chain to determine
    /// their current balance.
    pub fn get_balance_of(&self, key: Vec<u8>) -> f64 {
        let mut bal = 0 as f64;

        for block in self.blocks.as_slice() {
            for trans in block.transactions.as_slice() {
                if trans.trans.receiver_public_key == key {
                    bal += trans.trans.amount;
                }

                if trans.trans.sender_public_key == key {
                    bal -= trans.trans.amount;
                }
            }
        }

        return bal;
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Block {
    pub transactions: Vec<SignedTransaction>,
    pub prev_hash: Vec<u8>,
    pub nonce: u32,
    pub reward: RewardBlock,
}

impl Block {
    pub fn new(transactions: Vec<SignedTransaction>, prev_hash: Vec<u8>, miner_public: PublicKey) -> Result<Self> {
        for st in &transactions {
            if !verify(st.trans.get_sender_public_key(), st) {
                return Err(anyhow!("Transaction invalid - {} :: Amount - {}", String::from_utf8_lossy(st.trans.hash_of().as_slice()), st.trans.amount));
            }
        }

        Ok(Self {
            transactions,
            prev_hash,
            nonce: 0,
            reward: RewardBlock::new(miner_public),
        })
    }

    pub fn create_genesis(init_key: PublicKey) -> Self {
        let transactions = Vec::new();

        // Add genesis transaction here


        Self {
            transactions,
            prev_hash: "GENESIS".as_bytes().to_vec(),
            nonce: 0,
            reward: RewardBlock::new(init_key),
        }
    }

    // https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
    pub fn hash_of(&self) -> Vec<u8> {
        to_hash(bincode::serialize(self).unwrap())
    }

    pub fn mine(&mut self) -> Result<(Block, Vec<u8>)> {

        while get_leading_zeroes(self.hash_of()) < ZEROES_NEEDED {
            self.nonce += 1;
        }

        return Ok((self.clone(), self.hash_of()));
    }
}

const BLOCK_REWARD: f64 = 10.0;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RewardBlock {
    pub public: Vec<u8>,
    pub amount: f64,
}

impl RewardBlock {
    /// Creates a new block reward using the miner's
    /// public key. The public key provided is the one that
    /// will receive the mining reward if the mine was successful.
    ///
    /// TO-DO: Decreasing block rewards based on number of mined blocks
    pub fn new(public: PublicKey) -> Self {
        Self {
            public: public.to_bytes().to_vec(),
            amount: BLOCK_REWARD,
        }
    }
}

pub fn to_hash(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    sha2::Digest::update(&mut hasher, data);
    hasher.finalize().as_slice().to_vec()
}

pub fn get_leading_zeroes(vec: Vec<u8>) -> u8 {
    let mut count = 0;

    for u in vec {
        count += u.leading_zeros();

        if u.leading_zeros() != 8 {
            return count as u8;
        }
    }

    return count as u8;
}

#[test]
fn test_block() {

}

#[test]
fn hash_test() {
    let mut hasher = Sha256::new();
    sha2::Digest::update(&mut hasher, b"test");
    let result = hasher.finalize();

    println!("{}", result.len());

    for u in result.as_slice() {
        let t = *u;

        print!("{}", t.leading_zeros());

        /*
        for i in 0..8 {
            print!("{}", t & (1 << i));
        }

        println!("");
         */
    }
    println!("");
}