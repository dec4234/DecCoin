use uuid::Uuid;
use serde::{Deserialize, Serialize};

// https://hackernoon.com/rusty-chains-a-basic-blockchain-implementation-written-in-pure-rust-gk2m3uri
// https://github.com/emcthye/Proof-of-Stake-in-Rust
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub uuid: Uuid,
}

impl Transaction {
    pub fn new() {
        // uuid: Uuid::default()
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}