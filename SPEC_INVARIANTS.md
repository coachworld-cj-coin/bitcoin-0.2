## ⛓ Bitcoin Revelation — Consensus v3 Invariants

**Status:** Binding
**Applies to:** Consensus v3
**Change policy:** Version-gated hard fork only

This document defines **consensus invariants** — rules and values that **must not change** within Consensus v3 under any circumstances.

Any violation of these invariants results in a **hard fork**, whether intentional or accidental.

---

## 1. Scope

This document applies **only** to consensus-critical behavior.

Anything not listed here is:

* non-consensus
* policy
* implementation detail
* or local preference

Mempool rules, mining policy, networking behavior, APIs, and UI logic are **out of scope**.

---

## 2. Genesis Invariants

The following **must remain identical forever** under Consensus v3:

* Genesis block height
* Genesis timestamp
* Genesis previous hash
* Genesis nonce
* Genesis target
* Genesis merkle root
* Genesis block hash

All nodes **must**:

* hard-code the genesis block
* verify its hash
* verify its Proof-of-Work
* verify its merkle root

Any deviation is invalid.

---

## 3. Proof-of-Work Invariants

* Hash function: **double SHA-256**
* Hash length: **32 bytes**
* Target length: **32 bytes**
* Endianness: **big-endian**
* Validity rule:
  **`hash ≤ target`**

Hash and target are interpreted as **unsigned 256-bit integers**.

Any change to:

* hash function
* comparison rule
* byte order
* target encoding

is a **hard fork**.

---

## 4. Difficulty Adjustment Invariants

* Difficulty operates on the **full 256-bit target**
* No compact encoding
* No floating-point math
* Integer arithmetic only
* Adjustment occurs only at fixed intervals
* Formula:

```
new_target = old_target × actual_time / expected_time
```

* Target is clamped to `[MIN_TARGET, MAX_TARGET]`

Any change to:

* interval length
* formula
* arithmetic precision
* rounding behavior
* clamping rules

is a **hard fork**.

---

## 5. Block Validity Invariants

A block is valid if and only if all of the following hold:

* Correct height
* Correct previous hash
* Valid Proof-of-Work
* Correct target for its height
* Timestamp > Median Time Past
* Timestamp ≤ now + MAX_FUTURE_DRIFT
* Merkle root matches transactions
* Block size ≤ MAX_BLOCK_SIZE

No optional rules exist.

---

## 6. Transaction Invariants

* UTXO-based accounting
* Inputs must reference unspent outputs
* Signatures must verify
* Input sum ≥ output sum
* Fees are implicit
* Coinbase transactions:

  * have no inputs
  * are only valid as the first transaction in a block
  * are subject to maturity rules

Any change to transaction semantics is a **hard fork**.

---

## 7. Coinbase Maturity Invariants

* Coinbase outputs are locked for a fixed number of blocks
* Coinbase maturity is enforced by consensus
* Immature coinbase spends are invalid

Changing the maturity rule is a **hard fork**.

---

## 8. Fork Choice Invariants

* Fork choice is based on **cumulative Proof-of-Work**
* Cumulative work is defined as:

```
Σ (2^256 / (target + 1))
```

* Longest chain by height is **not** authoritative
* Lowest target alone is **not** authoritative

Any change to fork-choice logic is a **hard fork**.

---

## 9. Serialization Invariants

* Consensus serialization is manual and deterministic
* Field order, byte order, and length prefixes are fixed
* Serialization is part of consensus hashing

Any change to consensus serialization is a **hard fork**.

---

## 10. Explicit Non-Invariants (Policy Only)

The following are **not consensus** and may vary between nodes:

* Mempool acceptance rules
* Fee policy
* Transaction prioritization
* Reorg depth limits
* Mining transaction selection
* Networking transports
* APIs and RPCs
* User interfaces

Differences here **must not** affect block validity.

---

## 11. Change Rule

Any change to an invariant listed in this document:

* requires a new consensus version
* requires explicit version gating
* constitutes a hard fork
* must tolerate permanent network splits

There are no emergency exceptions.

---

## 12. Finality

Consensus v3 is **final and frozen**.

This document exists to ensure that:

* contributors know what not to change
* alternative implementations can interoperate
* accidental forks are prevented
* future forks are explicit and intentional

---

⛓ **Bitcoin Revelation — Consensus v3**

---
