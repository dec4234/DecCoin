use std::str::FromStr;
use uuid::Uuid;
use serde::{Deserialize, Deserializer, Serialize};
use anyhow::Result;

// https://hackernoon.com/rusty-chains-a-basic-blockchain-implementation-written-in-pure-rust-gk2m3uri
// https://github.com/emcthye/Proof-of-Stake-in-Rust
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub uuid: String,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string()
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