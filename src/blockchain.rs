use std::hash::Hash;
use std::ptr::hash;
use anyhow::anyhow;
use bincode::Options;
use crate::transaction::{SignedTransaction, Transaction, verify};
use sha2::{Sha256, Digest};
use sha2::digest::Update;
use serde::{Deserialize, Serialize};
use anyhow::Result;

// Reduced to save time
const ZEROES_NEEDED: u8 = 3; // The number of leading zeroes needed in a hash to successfully mine a block (Normally 19)

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Block {
    pub transactions: Vec<SignedTransaction>,
    pub prev_hash: Vec<u8>,
    pub nonce: u32,
}

impl Block {
    pub fn new(transactions: Vec<SignedTransaction>, prev_hash: Vec<u8>) -> Result<Self> {
        for st in transactions {
            if !verify(st.trans.get_sender_public_key(), st) {
                return Err(anyhow!("Transaction invalid - {} :: Amount - {}", String::from_utf8_lossy(st.trans.hash_of().as_slice()), st.trans.amount));
            }
        }

        Ok(Self {
            transactions,
            prev_hash,
            nonce: 0,
        })
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
    let b = Block::new();
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