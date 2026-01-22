
# Bitcoin v0.2 – Revelation Edition

Bitcoin v0.2 – Revelation Edition is an experimental peer-to-peer electronic cash system written in Rust.

It is a **standalone proof-of-work network** intended to demonstrate the essential mechanics of decentralized consensus, block validation, and chain synchronization **without reliance on any central authority**.

This software is intended for **study, experimentation, and long-running private node operation**.
It is deliberately minimal.

---

## Design Intent

The objective of this project is not feature completeness, but **correctness of the base protocol**.

The system provides:

* independent block validation by every node
* proof-of-work–based chain selection
* automatic fork resolution
* deterministic reconstruction of state from genesis

There are no privileged nodes and no external coordinators.
Any node may leave or rejoin the network at any time.

---

## Network Model

Each node maintains a full copy of the blockchain and enforces all consensus rules locally.

A node may:

* validate blocks independently
* mine new blocks
* relay blocks to peers
* synchronize missing blocks automatically
* reorganize when a stronger chain is discovered

Temporary divergence between nodes is expected and resolves naturally through accumulated proof-of-work.

---

## Consensus Rules

Consensus follows a Bitcoin-style longest-chain-by-work rule:

* blocks must reference a known parent
* blocks must satisfy the current proof-of-work difficulty
* multiple competing chains are permitted
* the chain with the **most accumulated work** is selected
* nodes reorganize automatically when a stronger chain appears

Block height is derived from ancestry and is not enforced independently.

There is no global clock and no checkpoint authority.

---

## Revelation Block

The initial block (height 0) is referred to as the **Revelation Block**.

* it has no parent (`prev_hash = 0x00…00`)
* it contains a single deterministic transaction
* it establishes the initial state of the system

Functionally, it serves the role of a genesis block.
The name is used to distinguish intent, not behavior.

All nodes share the same Revelation Block.

---

## Monetary Policy

The monetary rules are fixed and enforced by consensus:

* block subsidy is height-based
* the halving schedule is deterministic
* total supply is capped at **21 million units**
* coinbase outputs require maturity before spending

No node may create coins outside these rules.

---

## Features

* proof-of-work mining
* fork-capable consensus with reorganization
* Merkle-root-based block structure
* persistent blockchain and UTXO storage
* peer-to-peer networking over raw TCP
* automatic block synchronization
* optional HTTP interface for inspection

---

## Network Ports

Default ports:

* **8333** — peer-to-peer network
* **8080** — HTTP status interface (optional)

The P2P port does not speak HTTP and should not be accessed with a web browser.

When running multiple nodes on a single machine, each node must use a unique P2P port and data directory.

---

## Running on Mobile (Android / Termux)

A full validating node can be run on a mobile device using **Termux**.

This is possible because the node:

* maintains no indexes by default
* stores only minimal state (blocks and UTXOs)
* reconstructs all state deterministically

There is no mobile mode.
A phone running this software is **a full node**.

### Requirements

* Android device (ARM64 recommended)
* Termux installed from **F-Droid**
* approximately 200 MB of free storage
* stable internet connection

> Termux from Google Play is deprecated. Use the F-Droid distribution.

---

### Installation

Install Termux:
[https://f-droid.org/packages/com.termux/](https://f-droid.org/packages/com.termux/)

Update packages:

```bash
pkg update && pkg upgrade
```

Install dependencies:

```bash
pkg install git rust clang
```

Verify:

```bash
rustc --version
cargo --version
```

Clone the repository:

```bash
git clone https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2.git
cd bitcoin-0.2
```

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

On first execution, the node will:

* create the Revelation Block
* initialize local storage
* connect to peers
* synchronize the blockchain

Validation behavior is identical to desktop operation.

---

## Building and Running (Desktop)

Build:

```bash
cargo build
```

Run a single node:

```bash
cargo run -- --port 8333 --mine
```

On startup, a node will:

* load the local blockchain from disk
* create the Revelation Block if none exists
* listen for peer connections
* request missing blocks from peers
* begin mining once synchronization stabilizes

---

## Running Multiple Nodes (Private Network)

Node A:

```bash
cargo run -- --port 8333 --data-dir nodeA --mine
```

Node B:

```bash
cargo run -- --port 8334 --data-dir nodeB --peers 127.0.0.1:8333 --mine
```

Additional nodes may be started by assigning unique ports and data directories.

Nodes converge automatically on the strongest chain.

---

## HTTP Interface

The HTTP interface is provided for inspection and development only.

Example endpoints:

* `/status` — chain height, difficulty, UTXO count
* `/blocks` — known blocks
* `/supply` — current monetary supply

Only one node should bind to port 8080 on a given machine.

---

## Data Storage

State is stored locally:

```
data/
├── blocks.json
└── utxos.json
```

Nodes may be stopped and restarted without loss of state.

All state can be reconstructed from the blockchain.

---

## Local Search and Indexing

The node does **not** maintain transaction, address, or block-height indexes by default.

It stores only:

* the blockchain
* the current UTXO set

These are sufficient to:

* validate blocks
* enforce consensus
* mine
* synchronize

Search and indexing are policy layers, not consensus requirements.

---

## Limitations

This software is experimental.

It intentionally omits:

* mandatory indexes
* advanced mempool policies
* network encryption
* denial-of-service protections
* fast synchronization or snapshots
* production-grade peer discovery

These may be added without modifying consensus rules.

---

## Release Status

This release **freezes the consensus rules**.

* independent network
* fixed monetary policy
* stable proof-of-work and fork-selection rules

**Tag:** `v0.2-consensus-stable`

---

## Purpose

This project demonstrates that a peer-to-peer proof-of-work system can be implemented clearly, compactly, and without trusted parties.

If the software continues to run unchanged, the rules were sufficient.

---

## License

Open source.
Free to use, modify, and redistribute.
