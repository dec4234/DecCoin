use std::str::FromStr;
use uuid::Uuid;
use serde::{Deserialize, Deserializer, Serialize};
use anyhow::Result;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use ed25519_dalek::ed25519::signature::Signature;

// https://hackernoon.com/rusty-chains-a-basic-blockchain-implementation-written-in-pure-rust-gk2m3uri
// https://github.com/emcthye/Proof-of-Stake-in-Rust
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub uuid: String,
    pub amount: f64,
    pub receiver_public_key: Vec<u8>,
    pub sender_public_key: Vec<u8>,
}

impl Transaction {
    pub fn new(amount: f64, receiver: PublicKey, sender: PublicKey) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            amount,
            receiver_public_key: receiver.to_bytes().to_vec(),
            sender_public_key: sender.to_bytes().to_vec(),
        }
    }

    pub fn get_receiver_public_key(&self) -> PublicKey {
        PublicKey::from_bytes(self.receiver_public_key.as_slice()).unwrap()
    }

    pub fn get_sender_public_key(&self) -> PublicKey {
        PublicKey::from_bytes(self.sender_public_key.as_slice()).unwrap()
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

impl SignedTransaction {
    pub fn sign(trans: Transaction, pair: Keypair) -> SignedTransaction {
        let sign = pair.sign(bincode::serialize(&trans).unwrap().as_slice());

        SignedTransaction {
            trans,
            signature: sign.as_bytes().to_vec(),
        }
    }
}