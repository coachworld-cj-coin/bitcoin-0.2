// As AI advances and human greed intensifies, our best defense is to secure and focus on Bitcoin’s singular, unified language. Satoshi Nakamoto did not create Bitcoin merely for finance; it was built to preserve history and establish a new digital physics—an immutable record that stands outside of human interference.

use ed25519_dalek::{SigningKey, Signature, Signer};
use rand::rngs::OsRng;

use crate::crypto::sha256;
use crate::transaction::{Transaction, TxInput, TxOutput};

/// Simple wallet holding a keypair
pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    /// Generate a new wallet (new identity)
    pub fn new() -> Self {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        Self { signing_key }
    }

    /// Load wallet from an existing private key (32 bytes)
    pub fn from_private_key(bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(bytes);
        Self { signing_key }
    }

    /// Export private key (KEEP SECRET)
    pub fn private_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Public key bytes (safe to share)
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.signing_key.verifying_key().to_bytes().to_vec()
    }

    /// Address = sha256(pubkey)
    pub fn address(&self) -> Vec<u8> {
        sha256(&self.public_key_bytes())
    }

    /// Sign arbitrary message (used for transactions)
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let sig: Signature = self.signing_key.sign(message);
        sig.to_bytes().to_vec()
    }

    /// Create and sign a transaction spending UTXOs
    pub fn create_transaction(
        &self,
        inputs: Vec<(Vec<u8>, usize)>, // (txid, index)
        outputs: Vec<TxOutput>,
    ) -> Transaction {
        let mut tx = Transaction {
            inputs: inputs
                .into_iter()
                .map(|(txid, index)| TxInput {
                    txid,
                    index,
                    signature: vec![],
                    pubkey: self.public_key_bytes(),
                })
                .collect(),
            outputs,
        };

        let sighash = tx.sighash();

        for input in &mut tx.inputs {
            input.signature = self.sign(&sighash);
        }

        tx
    }
}
