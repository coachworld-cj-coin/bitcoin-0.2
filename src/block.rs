use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::crypto::sha256;

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub timestamp: i64,
    pub prev_hash: Vec<u8>,
    pub nonce: u64,
    pub difficulty: u32,
    pub merkle_root: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: Vec<u8>,
}

impl Block {
    pub fn hash_header(&self) -> Vec<u8> {
        sha256(&sha256(&bincode::serialize(&self.header).unwrap()))
    }

    pub fn verify_pow(&self) -> bool {
        let hash = self.hash_header();
        let mut remaining = self.header.difficulty;

        for byte in hash {
            let zeros = byte.leading_zeros();

            if zeros >= remaining {
                return true;
            }

            if zeros < 8 {
                return false;
            }

            remaining -= zeros;
        }

        false
    }
}
