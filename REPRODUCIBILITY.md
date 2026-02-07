# 🔎 Reproducibility & Deterministic Wallet Recovery

This document explains how to **deterministically reproduce a Bitcoin wallet** generated with **bip39-dice**, step by step.

The goal is to ensure that **anyone**, at any point in the future, can reconstruct **the exact same wallet** using the same inputs — without trusting this software, the binary, or its author.

---

## 🧠 Core Principle

Wallet generation is **fully deterministic** given the following parameters:

1. Mnemonic size (12 or 24 words)
2. Dice sequence (manual or auto-generated)
3. Optional passphrase
4. Network (Mainnet or Testnet)

If **all inputs are identical**, the resulting wallet **will be identical**.

---

## 🎲 Dice Entropy (Manual Mode)

When using **manual dice input**, the dice sequence becomes a **human-verifiable source of entropy**.

Rules:

* Each character must be a number from **1 to 6**
* No separators (spaces, commas, etc.)

Example:

```bash
314626515245366152436615243661524366
```

Properties:

* Human-generated
* Verifiable by sight
* Reusable
* Independent from system RNG

---

## 🔐 Hybrid Entropy Model

`seedctl` uses a **hybrid entropy model** internally:

```bash
entropy_final = SHA256(dice_entropy || hex_entropy)
```

Where:

* `dice_entropy` is derived from the dice sequence
* `hex_entropy` is deterministic in manual mode and randomly generated in auto mode

### 🔒 Security Properties

* Manual dice mode is **fully reproducible**
* No hidden randomness is introduced
* Auto mode intentionally introduces non-reproducible entropy

This design allows the same software to be used for:

* Secure wallet generation
* Deterministic wallet recovery

---

## 🧪 Step-by-Step Reproduction Example

### Parameters

* Mnemonic size: **12 words**
* Dice mode: **Manual**
* Dice sequence: 314626515245366152436615243661524366
* Passphrase: *(empty)*
* Network: **Mainnet**

---

### Step 1 — Run offline

```bash
./seedctl
```

Recommended environment:

* Offline computer
* Air-gapped system
* Tails OS or similar

---

### Step 2 — Select mnemonic size

Choose:

```bash
12 words (128 bits)
```

---

### Step 3 — Dice mode

Choose:

```bash
Manual (inform sequence)
```

Paste the exact dice sequence.

---

### Step 4 — Visual confirmation

The program will display:

```bash
DICE USED (34 numbers): 314626515245366152436615243661524366
```

Verify carefully before confirming.

---

### Step 5 — Network

Choose:

```bash
Mainnet
```

---

### Step 6 — Passphrase

Press **Enter** for empty passphrase, or enter the **exact same passphrase** used originally.

⚠️ The passphrase is part of the seed. Any change results in a different wallet.

---

## 📇 Output Verification

The following outputs must match **exactly**:

* Mnemonic words
* Word indexes (BIP39, base-1)
* Derivation path
* ZPRV / ZPUB
* Generated addresses

If all values match, the wallet has been successfully reproduced.

No hashes, binaries or signatures are required for wallet reproduction.

---

## 🔁 Wallet Recovery in Other Software

This wallet can be imported into:

* Sparrow Wallet
* Electrum
* BlueWallet
* Bitcoin Core

Use:

* BIP39 mnemonic
* Same passphrase
* Derivation path:

```bash
  m/84'/0'/0'
```

---

## ⚠️ Common Errors

* Using auto mode instead of manual dice
* Incorrect dice sequence
* Forgotten passphrase
* Wrong network (Mainnet vs Testnet)
* Different derivation path

Any of these will generate a **different wallet**.

---

## 🧠 Final Notes

Reproducibility is a **security feature**, not a limitation.

If you can reproduce the wallet from scratch, you do **not need to trust this software**.

That is the point.
