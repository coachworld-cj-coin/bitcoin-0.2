## Consensus Rules (Formal Specification)

This document defines the **consensus-critical rules** for **Bitcoin v0.2 — Revelation Edition**.

All nodes participating in consensus **MUST** enforce these rules exactly.
Any behavior not explicitly permitted by this document **MUST** be considered invalid.

This specification is **authoritative** over informal descriptions.

## 1. Terminology

The key words **MUST**, **MUST NOT**, **SHALL**, **SHALL NOT**, **SHOULD**, and **MAY** are to be interpreted as described in RFC 2119.

## 2. Block Structure

### 2.1 Block Header

Each block header **MUST** contain the following fields:

* Block height
* Timestamp (Unix epoch seconds)
* Previous block hash
* Difficulty parameter
* Nonce

The block header **MUST** be serialized deterministically.

### 2.2 Block Hash

The block hash **MUST** be computed as:

```
SHA256(SHA256(serialized_block_header))
```

Only the block header **SHALL** be included in the hash computation.

## 3. Revelation Block

### 3.1 Creation

* The revelation block **MUST** be deterministically generated on first execution.
* All nodes running identical software **MUST** produce the same revelation block.
* The revelation block **MUST NOT** reference a previous block.

### 3.2 Revelation Validity

The revelation block **SHALL** be considered valid without requiring Proof-of-Work verification against a prior block.

## 4. Proof-of-Work

### 4.1 Validity Condition

A block **MUST** satisfy the Proof-of-Work requirement.

A block hash is valid if and only if:

* The number of leading zero bits in the block hash is **greater than or equal to** the current difficulty value.

Big-integer target comparison **SHALL NOT** be used.

### 4.2 Mining Process

Miners **MUST**:

1. Increment the nonce field
2. Recompute the block hash
3. Repeat until the Proof-of-Work condition is satisfied

There **SHALL** be no upper bound on nonce iteration.

## 5. Block Timing

### 5.1 Target Interval

* The target block interval **SHALL** be 600 seconds.

### 5.2 Enforcement

* The target interval **SHALL NOT** be enforced on a per-block basis.
* Block timing **SHALL** influence difficulty adjustment only.

## 6. Difficulty Adjustment

### 6.1 Adjustment Interval

* Difficulty **SHALL** be adjusted every 2016 blocks.

### 6.2 Adjustment Rules

* Difficulty **MUST** increase if blocks are produced faster than the target interval.
* Difficulty **MUST** decrease if blocks are produced slower than the target interval.
* Difficulty adjustments **MUST** be bounded to prevent extreme oscillations.

The exact adjustment formula **SHALL** be defined by the implementation and treated as consensus-critical.

## 7. Transactions

### 7.1 Transaction Model

* The system **MUST** use a UTXO-based transaction model.
* Transactions **MUST** consume existing unspent outputs.
* Transactions **MUST NOT** double-spend any output.

### 7.2 Script and Signatures

* Script execution **SHALL NOT** be supported.
* Cryptographic signature validation **MAY** be simplified or omitted.

## 8. Coinbase Transaction

### 8.1 Coinbase Creation

Each block **MUST** contain exactly one coinbase transaction.

The coinbase transaction **MUST**:

* Create new coins according to the block subsidy rules
* Have no inputs

### 8.2 Block Subsidy

* Initial block subsidy **SHALL** be 50 coins.
* Subsidy **MUST** halve every 210,000 blocks.
* Subsidy calculation **MUST** be deterministic.

### 8.3 Coinbase Maturity

* Coinbase outputs **MUST NOT** be spent until they have at least 100 confirmations.

## 9. Monetary Supply

* Total coin supply **SHALL** asymptotically approach 21,000,000 coins.
* No mechanism **SHALL** exist to create coins outside the subsidy schedule.

## 10. Block Size

* The serialized block size **MUST NOT** exceed 1 megabyte.

Blocks exceeding this limit **MUST** be rejected.

## 11. Chain Selection

### 11.1 Fork Choice Rule

Nodes **MUST** select the chain with the greatest accumulated Proof-of-Work.

Accumulated work **SHALL** be derived from per-block difficulty values.

### 11.2 Reorganization Limits

* Chain reorganizations **MUST NOT** exceed 100 blocks in depth.
* Blocks causing deeper reorganizations **MUST** be rejected.

## 12. Validation Rules

A block **MUST** be rejected if any of the following are true:

* Proof-of-Work is invalid
* Previous block is unknown
* Transactions violate UTXO rules
* Coinbase subsidy is incorrect
* Block size exceeds limits
* Reorganization depth exceeds limits

All validation rules **MUST** be enforced deterministically.

## 13. Governance and Upgrades

* No governance mechanism **SHALL** exist.
* No protocol upgrade mechanism **SHALL** exist.
* Consensus rules **MUST NOT** change automatically.

Any rule change requires explicit software replacement.

## 14. Finality

Consensus emerges exclusively from:

* Deterministic rules
* Computational Proof-of-Work

There **SHALL** be no administrative overrides, trusted parties, or external authorities.

## 15. Canonical Principle

**Time + Energy = Truth**
