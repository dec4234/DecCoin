use std::str::FromStr;
use uuid::Uuid;
use serde::{Deserialize, Deserializer, Serialize};
use anyhow::Result;
use ed25519_dalek::PublicKey;

// https://hackernoon.com/rusty-chains-a-basic-blockchain-implementation-written-in-pure-rust-gk2m3uri
// https://github.com/emcthye/Proof-of-Stake-in-Rust
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub uuid: String,
    pub amount: f64,
    pub receiver_public_key: PublicKey,
    pub sender_public_key: PublicKey,
}

impl Transaction {
    pub fn new(amount: f64, receiver: PublicKey, sender: PublicKey) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            amount,
            receiver_public_key: receiver,
            sender_public_key: sender,
        }
    }

    pub fn broadcast(self) -> Result<()> {
        todo!()
    }

    pub fn get_uuid(&self) -> Uuid {
        Uuid::from_str(self.uuid.as_str()).unwrap()
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignedTransaction {
    pub trans: Transaction,
    pub signature: Vec<u8>,
}