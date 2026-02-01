## ⛓ Bitcoin Revelation — Running a Node

This document describes how to **safely and correctly run a Bitcoin Revelation node**.

It applies to:

* full nodes
* miners
* long-running archival nodes
* research nodes

Consensus rules are **not** configurable.

---

## 1. System Requirements

### Minimum (development / test)

* 64-bit OS (Linux, macOS, Windows)
* Rust stable toolchain
* ≥ 2 GB RAM
* ≥ 5 GB disk space

### Recommended (long-running node)

* ≥ 8 GB RAM
* SSD storage
* Stable system clock
* Persistent network connectivity

---

## 2. Building the Node

Always build from a **release tag**, not `main`.

```bash
git clone https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2
cd bitcoin-0.2
git checkout v0.3.3
cargo build --release
```

Never run unreviewed binaries on a machine holding funds.

---

## 3. First Run

On first launch:

```bash
cargo run --release
```

The node will:

1. Create the data directory
2. Initialize the hard-coded genesis block
3. Create or load the wallet
4. Start P2P networking
5. Begin syncing and mining (if enabled)

You will be prompted for:

* wallet passphrase
* wallet password

**Do not lose these.**

---

## 4. Data Directory

The node stores data locally in:

```
./data/
```

This includes:

* blockchain data
* UTXO set
* wallet file
* configuration files

Deleting this directory resets the node state.

---

## 5. Wallet Operation

### Wallet creation

* A recovery phrase is shown once
* It is never stored
* Anyone with the phrase can spend your funds

Back it up **offline**.

### Wallet unlocking

* Required to create transactions
* Required to mine to a local address

### Wallet loss

There is **no recovery mechanism**.

Loss of the phrase or password results in permanent loss of funds.

---

## 6. Running as a Miner

Mining is enabled by default.

* Blocks are mined locally
* Rewards are paid to the configured wallet
* Coinbase outputs are locked by consensus

Mining does not require continuous connectivity.
Blocks propagate when peers are available.

---

## 7. Networking

The node supports multiple transports:

* TCP
* LAN mesh (GEO)
* Bluetooth (BLE)
* Satellite (receive-only)
* Offline / store-and-forward

All transports feed the same validation pipeline.
No transport bypasses consensus checks.

Firewalls may restrict connectivity but do not affect local validation.

---

## 8. REST API

A non-consensus REST API is available (default port: `8080`).

It provides:

* chain status
* block lookup
* transaction lookup
* address inspection
* transaction submission (mempool)

API behavior does **not** affect consensus.

---

## 9. Long-Running Node Guidelines

For stable operation:

* Keep system time accurate
* Avoid abrupt shutdowns
* Back up wallet data regularly
* Monitor disk usage
* Avoid modifying consensus code

Upgrades should only be performed at **explicit release tags**.

---

## 10. Upgrading

Before upgrading:

1. Back up `data/`
2. Read release notes
3. Verify consensus version compatibility

Consensus v3 is frozen.
Future versions may introduce **explicit forks**.

Never assume automatic compatibility.

---

## 11. Forking and Experiments

Running modified code creates a **local fork**.

This is allowed.

If experimenting:

* use a separate data directory
* clearly label binaries
* do not mix networks unintentionally

---

## 12. Responsibility

Running a node means:

* you verify your own data
* you choose your own rules
* you accept the risks

There is no authority to appeal to.

---

⛓ **Bitcoin Revelation**

---
