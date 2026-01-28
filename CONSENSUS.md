# Consensus Rules

This document defines the rules used by nodes to determine block and chain validity.

Consensus is achieved through independent verification.  
Nodes do not trust peers.

---

## Block Validity

A block is valid if:

- Previous block hash matches
- Height increments by exactly one
- At least one transaction exists
- Merkle root matches transactions
- Proof-of-work satisfies target
- Difficulty matches expected value

Invalid blocks are rejected.

---

## Proof-of-Work

Proof-of-work is performed by hashing the block header with a changing nonce.

A block is valid only if its hash satisfies the difficulty target.

Difficulty adjustment is deterministic.

---

## Genesis Block (Consensus Law)

The Genesis block is hard-coded.

All nodes must agree on:
- Timestamp
- Target
- Merkle root
- Hash

Chains that do not match the Genesis definition are invalid.

---

## Coinbase Maturity

Coinbase outputs are locked for a fixed number of blocks.

Spending before maturity is invalid.

---

## Chain Selection

Nodes select the valid chain with the greatest accumulated proof-of-work.

---

## Finality

Reorganizations may occur but become less likely as depth increases.

---

## Status

Consensus Version: v3  
Status: FINAL / FROZEN

Any change requires a version-gated fork.
