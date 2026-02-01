## ⛓ Bitcoin Revelation — Forking Policy

**Applies to:** All versions
**Current consensus:** v3 (FINAL / FROZEN)

This document describes **how forks are expected to occur** within the Bitcoin Revelation ecosystem.

Forking is permitted, expected, and neutral.

---

## 1. Philosophy

Bitcoin Revelation follows a **voluntary consensus model**.

* No node is required to follow any fork
* No fork is “official” by decree
* Consensus emerges from local verification and accumulated Proof-of-Work

Forks are not failures.
Forks are a feature of permissionless systems.

---

## 2. Definitions

* **Consensus version:** A set of rules defining block and transaction validity
* **Hard fork:** Any change that makes previously valid blocks invalid (or vice versa)
* **Policy change:** A non-consensus change that does not affect validity
* **Version gating:** Explicit activation of new rules at a defined height or version

---

## 3. Status of Consensus v3

Consensus v3 is **final and frozen**.

This means:

* No consensus rules may be changed
* No refactors affecting consensus behavior are permitted
* No “cleanup” changes are allowed
* No emergency modifications exist

Any deviation from Consensus v3 **is a hard fork**, regardless of intent.

---

## 4. When a Fork Is Required

A new consensus version (e.g. v4) is required if any of the following change:

* Proof-of-Work rules
* Difficulty adjustment
* Transaction validation
* Serialization
* Fork-choice logic
* Coinbase maturity
* Monetary issuance
* Genesis parameters

If unsure, **assume it is a fork**.

---

## 5. How to Create a Fork (Expected Process)

A responsible fork follows these steps:

1. **Write a specification**

   * Human-readable
   * Explicit rule changes
   * No code first

2. **Assign a new consensus version**

   * Example: Consensus v4
   * Never reuse version numbers

3. **Define activation**

   * Fixed block height or
   * Explicit version signaling
   * No silent activation

4. **Implement version-gated code**

   * Old rules remain intact
   * New rules activate explicitly

5. **Accept permanent splits**

   * No rollback expectations
   * No claims of authority

---

## 6. What Is Not a Fork

The following do **not** require a consensus fork:

* Mempool policy changes
* Fee policy
* Mining transaction selection
* Networking transports
* APIs or RPCs
* Wallet behavior
* UI or CLI changes

These are **local policy decisions**.

---

## 7. Coexistence of Forks

Multiple forks may coexist indefinitely.

* Nodes choose which rules to run
* Markets choose which chain to value
* No coordination is required
* No reconciliation is guaranteed

There is no obligation to converge.

---

## 8. Naming & Branding

Fork authors are encouraged to:

* clearly rename their fork
* change version identifiers
* document rule differences

This avoids user confusion and accidental cross-network usage.

---

## 9. No Emergency Forks

There are:

* no emergency powers
* no trusted signers
* no hidden switches

All forks must be explicit, deliberate, and documented.

---

## 10. Final Notes

This document exists to:

* prevent accidental forks
* reduce social conflict
* encourage responsible experimentation
* protect users and developers

Forking is neutral.
Consensus is voluntary.
Responsibility is local.

---

⛓ **Bitcoin Revelation**

---

# 🏁 WHY THIS WAS THE RIGHT MOVE

✔️ Eliminates ambiguity
✔️ Prevents social power grabs
✔️ Protects future contributors
✔️ Makes v4 possible *without pressure*
✔️ Aligns code, README, and philosophy
✔️ Zero risk to the running network

At this point, your project has:

* frozen consensus
* explicit invariants
* honest security model
* clear forking rules

This is **protocol adulthood**.

---
