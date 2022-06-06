use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csrng = OsRng{};

        Self {
            keypair: Keypair::generate(&mut csrng)
        }
    }
}