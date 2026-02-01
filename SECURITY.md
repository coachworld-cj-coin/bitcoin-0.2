# Security Model

Bitcoin Revelation follows a **local-verification security model**.

Each node independently validates:

* blocks
* transactions
* Proof-of-Work
* consensus rules

No trust is placed in peers, infrastructure, or external services.

---

## Wallet Security

* Recovery phrase is displayed once at creation
* The phrase is never stored on disk
* Wallet data is encrypted using **AES-256-GCM**
* Key derivation uses **PBKDF2**

Loss of the recovery phrase or wallet password results in **permanent loss of funds**.

There is no recovery mechanism.

---

## Node Security

* All data is validated locally
* Invalid blocks or transactions are rejected
* Consensus rules are enforced deterministically

Nodes do not rely on:

* trusted peers
* checkpoints
* signed binaries
* centralized infrastructure

---

## Network Security

* No trusted bootstrap nodes
* No privileged peers
* No network-level authority

Network messages are treated as untrusted input.
Consensus validity is determined solely by local verification.

---

## Threat Model Notes

The system is designed to resist:

* invalid Proof-of-Work
* invalid transactions
* timestamp manipulation
* difficulty collapse
* transaction forgery

The system does **not** claim to prevent:

* 51% hash-power attacks
* deep historical re-organizations
* network partitions

These risks are inherent to Proof-of-Work systems.

---

## User Responsibility

Users are responsible for:

* backing up recovery phrases
* securing their machines
* running trusted builds
* understanding protocol risks

No guarantees are provided.

---

# 🧭 CONSENSUS v4 — **PLANNING ONLY (NO CODE)**

Now, **very important**:
CONSENSUS v4 should **not** exist yet — only as a *design envelope*.

This is healthy.

---

## Purpose of Consensus v4

Consensus v4 exists **only if** one or more of the following become true:

* sustained hash-rate volatility causes instability
* difficulty oscillation harms liveness
* long-range reorgs become a real economic issue
* network size materially increases

Until then, **v3 remains law**.

---

## Allowed Scope for v4 (Hard Limits)

Consensus v4 may consider **only**:

1. **Difficulty Adjustment Improvements**

   * Longer adjustment window
   * EMA-style smoothing
   * Better resistance to hash-rate cliffs

2. **Explicit Fork Activation Rules**

   * Fixed activation height
   * Explicit version gating
   * No silent behavior changes

3. **Formal Finality Signaling (Optional)**

   * Soft finality hints
   * No economic guarantees
   * No checkpoints

---

## Explicitly Out of Scope for v4

Consensus v4 must **not** include:

* Proof-of-Stake
* Checkpoints
* Trusted signers
* Governance mechanisms
* On-chain voting
* Monetary policy changes
* Genesis changes

These are **non-negotiable exclusions**.

---

## Activation Principles

If Consensus v4 ever exists:

* It must be opt-in
* It must be version-gated
* It must be documented before coding
* It must be announced well in advance
* It must tolerate permanent forks

No emergency forks.
No silent upgrades.

---

## Current Status

* Consensus v4: **NOT IMPLEMENTED**
* Consensus v3: **FINAL / FROZEN**

Any future work on v4 begins with:

1. written specification
2. public review
3. explicit fork height
4. independent implementations

---
