// ─────────────────────────────────────────────
// CONSENSUS v3 — FROZEN
// Manual serialization for consensus-critical hashing
// DO NOT MODIFY WITHOUT A FORK
// ─────────────────────────────────────────────

use crate::core::block::BlockHeader;
use crate::core::transaction::{Transaction, TxInput, TxOutput};

fn write_u64_le(v: u64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_i64_le(v: i64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_u32_le(v: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_bytes(bytes: &[u8], out: &mut Vec<u8>) {
    write_u32_le(bytes.len() as u32, out);
    out.extend_from_slice(bytes);
}

/// Serialize block header EXACTLY for hashing (CONSENSUS)
pub fn serialize_block_header(header: &BlockHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(128);

    write_u64_le(header.height, &mut out);
    write_i64_le(header.timestamp, &mut out);
    write_bytes(&header.prev_hash, &mut out);
    write_u64_le(header.nonce, &mut out);
    out.extend_from_slice(&header.target);
    write_bytes(&header.merkle_root, &mut out);

    out
}

/// Serialize transaction EXACTLY for txid / sighash (CONSENSUS)
pub fn serialize_transaction(tx: &Transaction) -> Vec<u8> {
    let mut out = Vec::new();

    write_u32_le(tx.inputs.len() as u32, &mut out);
    for i in &tx.inputs {
        serialize_input(i, &mut out);
    }

    write_u32_le(tx.outputs.len() as u32, &mut out);
    for o in &tx.outputs {
        serialize_output(o, &mut out);
    }

    out
}

fn serialize_input(i: &TxInput, out: &mut Vec<u8>) {
    write_bytes(&i.txid, out);
    write_u32_le(i.index, out);
    write_bytes(&i.pubkey, out);
    write_bytes(&i.signature, out);
    write_u32_le(i.address_index, out);
}

fn serialize_output(o: &TxOutput, out: &mut Vec<u8>) {
    write_u64_le(o.value, out);
    write_bytes(&o.pubkey_hash, out);
}
