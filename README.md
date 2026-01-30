## ⛓ Bitcoin v0.3.3 — Revelation Edition

Stable Node, Wallet, Mining & Public P2P Network  
Consensus Specification v1.0 — FINAL / FROZEN

Repository:
https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2

---

## Overview

Bitcoin Revelation v0.3.3 is a stable implementation of a Proof-of-Work
peer-to-peer blockchain with a finalized Layer-1 consensus.

All consensus rules are frozen and defined by a single canonical
specification. The network is permissionless, forkable, and requires
no central coordination.

There is no planned roadmap for consensus changes.

---

## Canonical Specification (v1.0)

**The protocol specification is the source of truth.**

All consensus rules, monetary issuance, fork-choice logic,
and validation requirements are defined here:

## src/spec.rs

Any change to the specification constitutes a hard fork.

Generated HTML documentation can be produced with:

cargo doc --no-deps

---

## Consensus Status

- Consensus Version: **v3**
- Specification Version: **v1.0**
- Status: **FINAL / FROZEN**
- Genesis: **Hard-coded**
- Halving schedule: **Fixed**
- Coinbase maturity: **Enforced**
- Fork-choice: **Cumulative Proof-of-Work**

No chain reset is required.
All existing coins remain valid.

---

## Network Status

A public seed node was previously provided for bootstrapping.
The network is fully peer-to-peer and does not depend on any single host.

Nodes may join, sync, mine, or fork independently.

---

## What This Repository Contains

- Full node implementation
- Proof-of-Work mining
- Deterministic UTXO-based transactions
- Encrypted HD wallet (BIP39)
- Coinbase maturity enforcement
- Mempool validation
- P2P networking (transport-agnostic)
- REST API (non-consensus)
- Persistent chain and UTXO storage

---

## Mining

Mining requires valid Proof-of-Work.

- Rewards are created by coinbase transactions
- Newly mined coins are locked for a fixed maturity period
- Spendability is enforced by consensus rules

Mining may occur with intermittent connectivity.
Blocks are accepted solely by validity and accumulated work.

---

## Forking

Forking is permitted.

Anyone may:
- Run a node
- Mine blocks
- Fork the code
- Compete with alternative rule sets

Consensus is voluntary and local.

---

## Release Information

- Client: Bitcoin Revelation v0.3.3
- Tag: v0.3.3
- Consensus: v3 (FINAL)
- Specification: v1.0 (FROZEN)

Build from release tags only:
https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2/tags

---

## Disclaimer

This software is provided as-is for research and independent operation.

There is:
- No warranty
- No central authority
- No permission system
- No guarantee of value

Time is the final judge.

## ⛓ Satoshi-Nakamoto
