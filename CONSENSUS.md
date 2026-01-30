# Consensus Rules

This document defines the rules used by nodes to determine
block, transaction, and chain validity.

Consensus is achieved through independent verification.
Nodes do not trust peers.
All rules are deterministic.

---

## Block Validity

A block is valid if all of the following conditions are met:

- Previous block hash matches the parent block
- Block height increments by exactly one
- At least one transaction exists (coinbase)
- Merkle root matches the included transactions
- Proof-of-Work hash satisfies the target
- Difficulty target matches the expected value
- Block size does not exceed the maximum limit

Invalid blocks are rejected without exception.

---

## Proof-of-Work

Proof-of-Work is performed by hashing the serialized block header
using double SHA-256 while varying the nonce.

A block is valid only if:

```

hash(block_header) <= target

```

Difficulty adjustment is deterministic and computed from prior blocks.

---

## Timestamp Rules (Consensus)

Block timestamps must satisfy:

- Timestamp > Median Time Past of recent blocks
- Timestamp <= current network time + maximum future drift

Blocks violating timestamp rules are invalid.

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

## Coinbase Transaction

Each block may contain exactly one coinbase transaction.

Coinbase transactions:

- Have no inputs
- Create new coins
- Must not exceed the block reward for the given height

Issuing more coins than allowed is invalid.

---

## Monetary Issuance

Block rewards follow a deterministic halving schedule.

The reward is a function of block height.
After sufficient halvings, the reward becomes zero.

Total supply is bounded.

---

## Coinbase Maturity

Coinbase outputs are locked for a fixed number of blocks.

Spending a coinbase output before maturity is invalid.

---

## Transaction Validity

A transaction is valid if:

- All inputs reference existing unspent outputs
- No UTXO is spent more than once
- Signatures are valid
- Input value >= output value
- Coinbase maturity rules are respected

Invalid transactions invalidate the block.

---

## Chain Selection

Nodes select the valid chain with the greatest accumulated proof-of-work.

Chain selection is objective and requires no coordination.

---

## Finality

Reorganizations may occur but become less likely as depth increases.

There is no explicit finality mechanism.

---

## Status

- Consensus Version: v3
- Specification Version: v1.0
- Status: FINAL / FROZEN

Any change requires a version-gated hard fork.
```

