use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

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
}