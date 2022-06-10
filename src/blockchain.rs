use std::hash::Hash;
use std::ptr::hash;
use bincode::Options;
use crate::transaction::Transaction;
use sha2::{Sha256, Digest};
use sha2::digest::Update;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub prev_hash: Vec<u8>,
}

impl Block {
    pub fn new() -> Self {
        todo!()
    }

    // https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
    pub fn hash_of(&self) -> Vec<u8> {
        to_hash(bincode::serialize(self).unwrap())
    }
}

pub fn to_hash(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    sha2::Digest::update(&mut hasher, data);
    hasher.finalize().as_slice().to_vec()
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