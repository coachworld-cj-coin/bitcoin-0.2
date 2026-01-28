// ─────────────────────────────────────────────
// CONSENSUS v3 — FROZEN
// Hard-coded Genesis Block (Revelation)
// DO NOT MODIFY WITHOUT A FORK
// ─────────────────────────────────────────────

use crate::core::block::{Block, BlockHeader};
use crate::revelation::revelation_tx;

pub fn genesis_block() -> Block {
    Block {
        header: BlockHeader {
            height: 0,
            timestamp: 1730000000,        // ← REPLACE if different
            prev_hash: vec![0u8; 32],
            nonce: 0,                     // ← REPLACE
            target: [0xff; 32],           // ← REPLACE if different
            merkle_root: hex::decode(
                "REPLACE_WITH_GENESIS_MERKLE_ROOT"
            ).expect("genesis merkle"),
        },
        transactions: vec![
            revelation_tx(),
        ],
        hash: hex::decode(
            "REPLACE_WITH_GENESIS_HASH"
        ).expect("genesis hash"),
    }
}
