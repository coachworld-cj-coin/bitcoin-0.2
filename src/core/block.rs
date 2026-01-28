use serde::{Serialize, Deserialize};
use super::transaction::Transaction;
use crate::consensus::serialize::serialize_block_header;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub timestamp: i64,
    pub prev_hash: Vec<u8>,
    pub nonce: u64,
    pub target: [u8; 32],
    pub merkle_root: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: Vec<u8>,
}

impl Block {
    /// Block header hash (CONSENSUS)
    pub fn hash_header(&self) -> Vec<u8> {
        let bytes = serialize_block_header(&self.header);
        crate::crypto::sha256(&crate::crypto::sha256(&bytes))
    }

    pub fn verify_pow(&self) -> bool {
        self.hash == self.hash_header()
            && crate::pow::valid_pow(
                &self.hash,
                &self.header.target,
            )
    }
}
