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
// Normally adjusted as needed to reach a certain number of target blocks per hour
// This way the same number of blocks are created every hour regardless of the total mining capacity of the blockchain

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub init_key: Vec<u8>,
}

impl BlockChain {
    pub fn new(init_key: PublicKey) -> Self {
        let mut blocks = Vec::new();

        // Add genesis block here
        blocks.push(Block::create_genesis(init_key.to_bytes().to_vec()));

        Self {
            blocks,
            init_key: init_key.as_bytes().to_vec(),
        }
    }

    /// Returns the current balance of the user with the following
    /// public key. Replays all transactions in the block chain to determine
    /// their current balance.
    pub fn get_balance_of(&self, key: &Vec<u8>) -> f64 {
        let mut bal = 0 as f64;

        let key = key.clone();

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

    pub fn verify_transaction(&self, trans: &Transaction) -> Result<()> {
        if self.get_balance_of(&trans.sender_public_key) < trans.amount {
            return Err(anyhow!("Balance of sender is not sufficient for amount in the transaction."));
        }

        Ok(())
    }

    pub fn add_verified_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn hash_of_last(&self) -> Vec<u8> {
        self.blocks.last().unwrap().hash_of()
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

    pub fn create_genesis(init_key: Vec<u8>) -> Self {
        let transactions = Vec::new();

        // Add genesis transaction here



        Self {
            transactions,
            prev_hash: "GENESIS".as_bytes().to_vec(),
            nonce: 0,
            reward: RewardBlock::new_from_bytes(init_key),
        }
    }

    // https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
    pub fn hash_of(&self) -> Vec<u8> {
        to_hash(bincode::serialize(self).unwrap())
    }

    pub async fn mine(&mut self) -> Result<(Block, Vec<u8>)> {

        while get_leading_zeroes(self.hash_of()) < ZEROES_NEEDED {
            self.nonce += 1;
        }

        return Ok((self.clone(), self.hash_of()));
    }

    /// Ensures that the block has been mined
    /// enough to meet the requirements of the proof-of-work
    /// structure.
    pub fn is_mined(&self) -> bool {
        get_leading_zeroes(self.hash_of()) >= ZEROES_NEEDED
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

    pub fn new_from_bytes(public: Vec<u8>) -> Self {
        Self {
            public,
            amount: BLOCK_REWARD,
        }
    }
}

pub fn to_hash(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    sha2::Digest::update(&mut hasher, data);
    hasher.finalize().to_vec()
}

pub fn hash_string(data: &'static str) -> Vec<u8> {
    to_hash(data.as_bytes())
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

    for u in result.to_vec() {

        print!("{}", u.leading_zeros());

        /*
        for i in 0..8 {
            print!("{}", t & (1 << i));
        }

        println!("");
         */
    }
    println!("");
}