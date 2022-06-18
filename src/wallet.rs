use std::sync::mpsc::Sender;
use ed25519_dalek::{Keypair, PublicKey};
use rand::rngs::OsRng;
use anyhow::{anyhow, Result};
use crate::transaction::{SignedTransaction, Transaction};

pub struct Wallet {
    keypair: Keypair,
    pub balance: f64,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csrng = OsRng{};

        Self {
            keypair: Keypair::generate(&mut csrng),
            balance: 0.0,
        }
    }

    pub fn prepare_transaction(&mut self, amount: f64, receiver: PublicKey, sender: Sender<String>) -> Result<()> {
        if amount > self.balance {
            return Err(anyhow!("Amount exceeds current balance!"));
        }

        self.balance -= amount;

        let trans = Transaction::new(amount, receiver, self.keypair.public.clone());

        let signed = SignedTransaction::sign(trans, &self.keypair);

        // broadcast to network


        Ok(())
    }
}