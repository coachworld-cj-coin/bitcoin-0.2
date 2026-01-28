// ─────────────────────────────────────────────
// CONSENSUS v3 — FROZEN
//
// Difficulty adjustment MUST operate on the full
// 256-bit target as a single integer.
// Any modification requires a version-gated fork.
// ─────────────────────────────────────────────

use crate::block::Block;
use crate::consensus::params::*;

use num_bigint::BigUint;

fn clamp_target_big(target: BigUint) -> BigUint {
    let max = BigUint::from_bytes_be(&MAX_TARGET);
    let min = BigUint::from_bytes_be(&MIN_TARGET);

    if target > max {
        max
    } else if target < min {
        min
    } else {
        target
    }
}

/// Calculate expected PoW target for NEXT block
///
/// Formula:
/// new_target = old_target * actual_time / expected_time
pub fn calculate_next_target(chain: &[Block]) -> [u8; 32] {
    // Genesis / empty chain
    if chain.is_empty() {
        return MAX_TARGET;
    }

    let height = chain.len();
    let last = chain.last().unwrap();

    // Not enough blocks yet
    if height < DIFFICULTY_ADJUSTMENT_INTERVAL + 1 {
        return last.header.target;
    }

    // Only adjust on interval
    if height % DIFFICULTY_ADJUSTMENT_INTERVAL != 0 {
        return last.header.target;
    }

    let first =
        &chain[height - DIFFICULTY_ADJUSTMENT_INTERVAL - 1];

    let actual_time =
        last.header.timestamp - first.header.timestamp;

    let expected_time =
        TARGET_BLOCK_TIME * DIFFICULTY_ADJUSTMENT_INTERVAL as i64;

    // Prevent division by zero or negative time
    if actual_time <= 0 {
        return last.header.target;
    }

    // Convert target to BigUint
    let old_target =
        BigUint::from_bytes_be(&last.header.target);

    // Scale target
    let scaled = (&old_target * BigUint::from(actual_time as u64))
        / BigUint::from(expected_time as u64);

    let new_target = clamp_target_big(scaled);

    // Convert back to [u8; 32]
    let mut out = [0u8; 32];
    let bytes = new_target.to_bytes_be();

    // Right-align (big-endian)
    let start = 32usize.saturating_sub(bytes.len());
    out[start..].copy_from_slice(&bytes);

    out
}
