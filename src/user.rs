use rand::rngs::OsRng;
use rand::RngCore;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};

pub struct User {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    pub address: String,
}

impl User {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;

        let mut entropy = [0u8; 32];
        rng.fill_bytes(&mut entropy);

        let private_key = SecretKey::from_slice(&entropy)
            .expect("Failed to create secret key from entropy");

        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        let mut hasher = Sha256::new();
        hasher.update(public_key.serialize());
        let address = format!("{:x}", hasher.finalize());

        User {
            private_key,
            public_key,
            address,
        }
    }
}