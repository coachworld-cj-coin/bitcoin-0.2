## ⛓ Bitcoin Revelation — Testing & Reproducibility

This document describes how consensus behavior in Bitcoin Revelation
can be **reproduced, tested, and independently verified**.

The goal is determinism, not convenience.

---

## 1. Philosophy

Bitcoin Revelation follows a **verify-don’t-trust** model.

Reproducibility means:

* the same inputs produce the same outputs
* independent nodes reach the same conclusions
* consensus behavior does not depend on environment, peers, or timing (within bounds)

Testing is about **detecting divergence**, not proving correctness.

---

## 2. Consensus Determinism

Consensus behavior is deterministic given:

* the same genesis block
* the same sequence of blocks
* the same consensus rules

Non-deterministic components (networking, mempool policy, mining order)
**do not affect block validity**.

---

## 3. What Must Be Reproducible

The following must be reproducible across machines and implementations:

* Block header hashing
* Proof-of-Work validation
* Difficulty adjustment
* Fork-choice selection
* Transaction validation
* UTXO set evolution
* Coinbase maturity enforcement

Any divergence here indicates a **consensus failure**.

---

## 4. What Is Explicitly Not Reproducible

The following are **not consensus** and may vary:

* Transaction ordering within blocks (policy)
* Mempool contents
* Fee prioritization
* Mining nonce discovery
* Peer connectivity
* Network latency

Differences here are expected and acceptable.

---

## 5. Building Reproducible Binaries

To maximize reproducibility:

* Build from tagged releases only
* Use a stable Rust toolchain
* Avoid local code modifications
* Prefer `--release` builds for long-running nodes

Example:

```bash
git checkout v0.3.3
cargo build --release
```

Binary-level reproducibility is desirable but not guaranteed across platforms.

Consensus-level reproducibility **is required**.

---

## 6. Chain Reproduction Test (Manual)

A basic consensus sanity test:

1. Start two nodes independently
2. Use the same release version
3. Allow them to sync from genesis
4. Verify:

   * identical chain height
   * identical block hashes at each height
   * identical UTXO totals

Any discrepancy indicates a bug or fork.

---

## 7. Offline Reproduction

Consensus rules do not require live networking.

A node should be able to:

* load blocks from disk
* validate them deterministically
* reject invalid history

This property enables:

* offline audits
* air-gapped verification
* delayed synchronization

---

## 8. Independent Implementations

An independent implementation must:

* follow the canonical specification
* honor all consensus invariants
* reproduce genesis exactly
* match block validity decisions bit-for-bit

Language, architecture, and performance choices are irrelevant.
Only validity decisions matter.

---

## 9. Regression Testing Philosophy

Consensus regressions are unacceptable.

Before any release:

* existing blocks must remain valid
* existing invalid blocks must remain invalid
* no historical behavior may change

If a change alters past validity, it is a **hard fork**.

---

## 10. Fork Testing

When developing a fork (e.g. v4):

* use a separate data directory
* explicitly change version identifiers
* test divergence intentionally
* never mix forked and non-forked data

Accidental forks are worse than explicit ones.

---

## 11. Time-Related Testing

Consensus uses timestamps with bounds.

Testing should account for:

* Median Time Past
* Maximum future drift
* local clock skew

Nodes with incorrect clocks may reject valid blocks locally.
This does not alter consensus rules.

---

## 12. Responsibility

Testing and verification are the responsibility of:

* node operators
* miners
* developers
* independent auditors

There is no central test authority.

Consensus correctness emerges from **many independent verifications**.

---

## 13. Final Notes

This document exists to:

* encourage independent validation
* enable alternative implementations
* detect divergence early
* preserve consensus integrity

Reproducibility is a feature.
Determinism is non-negotiable.

---

⛓ **Bitcoin Revelation**

---