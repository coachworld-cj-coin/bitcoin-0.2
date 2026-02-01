
## ⛓ Bitcoin Revelation v0.3.3

**Stable Full Node · Wallet · Mining · Public P2P Network**
**Consensus v3 — FINAL / FROZEN**

Repository:
[https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2](https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2)

---

## Overview

Bitcoin Revelation v0.3.3 is a complete, self-validating implementation of a
Proof-of-Work, peer-to-peer blockchain system.

The Layer-1 consensus rules are **finalized and frozen**.
All nodes independently verify blocks, transactions, and Proof-of-Work.
There is no central coordination, checkpointing, or trusted infrastructure.

The network is permissionless and forkable by design.

---

## Canonical Specification

**The protocol specification is the source of truth.**

All consensus rules — including:

* block validity
* Proof-of-Work
* difficulty adjustment
* monetary issuance
* fork-choice logic

are defined in the canonical specification file:

```
src/spec.rs
```

Any change to this specification **constitutes a hard fork**.

Human-readable documentation can be generated with:

```bash
cargo doc --no-deps
```

---

## Consensus Status

* Consensus Version: **v3**
* Specification Version: **v1.0**
* Status: **FINAL / FROZEN**
* Genesis Block: **Hard-coded and self-verified**
* Difficulty Adjustment: **Full 256-bit target**
* Fork Choice: **Cumulative Proof-of-Work**
* Coinbase Maturity: **Enforced**
* Halving Schedule: **Fixed**

No chain reset is required.
All existing coins remain valid under Consensus v3.

---

## Network Model

* Fully peer-to-peer
* No trusted nodes
* No checkpoints
* No mandatory bootstrap infrastructure

Nodes may join, sync, mine, or fork independently.
Network partitions resolve solely via accumulated Proof-of-Work.

---

## What This Repository Contains

* Full node implementation
* Proof-of-Work mining
* Deterministic UTXO-based transactions
* Encrypted HD wallet (BIP-39)
* Coinbase maturity enforcement
* Mempool validation (policy)
* Transport-agnostic P2P networking
* REST API (non-consensus)
* Persistent chain and UTXO storage

---

## Mining

Mining requires valid Proof-of-Work.

* Block rewards are created by coinbase transactions
* Newly mined coins are locked for a fixed maturity period
* Spendability is enforced by consensus rules

Mining does not require continuous connectivity.
Blocks are accepted strictly by validity and accumulated work.

---

## Forking

Forking is explicitly permitted.

Anyone may:

* run a node
* mine blocks
* fork the code
* deploy alternative rule sets

Consensus is voluntary and local.
There is no obligation to follow any fork.

---

## Release Information

* Client: Bitcoin Revelation v0.3.3
* Release Tag: **v0.3.3**
* Consensus: **v3 (FINAL)**
* Specification: **v1.0 (FROZEN)**

Build from release tags only:
[https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2/tags](https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2/tags)

---

## Disclaimer

This software is provided **as-is** for research and independent operation.

There is:

* no warranty
* no central authority
* no permission system
* no guarantee of value

All use is voluntary.

Time is the final judge.

---

⛓ **Satoshi Nakamoto**

---
