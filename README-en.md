# SeedCTL

🇧🇷 [**Ler em Português**](README.md)

Deterministic, auditable, and security-focused Bitcoin wallet generator, written in [**Rust**](https://rust-lang.org).

This program allows you to generate a Bitcoin wallet from **physical data (data/dice)** and/or **system entropy**, producing:

- BIP39 mnemonic (12 or 24 words)
- Support for **optional passphrase**
- BIP84 derivation (Native SegWit – bc1)**
- Support for **Mainnet and Testnet**
- Display of [**Word Indexes BIP39**](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt)
- Generation of **deterministic addresses**

The main objective is to allow **secure, verifiable, and offline generation** of Bitcoin seeds, with a high level of paranoia and total control of the process.

---

## 🔐 Security Philosophy

- No network dependency
- No data transmission
- No disk persistence
- Ideal for **offline / air-gapped** use
- Compatible with manual verification (dice, word indexes, derivation path)
- Clear separation between **deterministic mode** and **hybrid mode**

> ⚠️ **WARNING**
> This program **displays sensitive information** (mnemonic, passphrase, keys).
> Use **only in a secure and offline environment**. Recommended for use with [Tails](https://tails.net/)

---

## ✨ Features

- ✅ BIP39 – 12 or 24 words
- 🎲 Entropy via physical data (1–6)
- 🔀 Hybrid entropy (physical data + system RNG)
- 🔁 Automatic generation or manual data entry
- 🔍 Visual confirmation of the data sequence
- 🔐 Optional passphrase (BIP39)
- 🌐 Mainnet and Testnet
- 🧭 BIP84 (Native SegWit)
- 📇 Display of **Word Indexes** (base 1, format `0001`)
- 🏷️ Generation of `bc1` / `tb1` addresses

---

## 📚 Documentation

- 🔎 **Deterministic Portfolio Replication**

See [`REPRODUCIBILITY.md`](REPRODUCIBILITY.md)

- 🔐 **Binary and Release Verification (SHA256 + GPG)**

See [`VERIFYING_RELEASES.md`](VERIFYING_RELEASES.md)

---

## 🎲 Entropy Modes

The program offers **two distinct modes**, with different objectives.

### 1️⃣ Manual Mode (Deterministic)

Suitable for:

- Recovering an existing portfolio
- Auditing
- Reproducible generation ceremonies
- Independent verification

**How ​​it works:**

- The user manually enters the data sequence (1–6)

- No system entropy is used
- The same sequence + same passphrase ⇒ **always the same portfolio**

**Conceptual model:**

```bash
entropy = SHA256(dice_entropy)

```

📌 This mode is **100% deterministic and reproducible**.

---

### 2️⃣ Automatic (Hybrid) Mode

Suitable for:

- Creating new wallets
- Increasing entropy against human error

Defense in depth

**How ​​it works:**

- The program automatically generates:

- 🎲 Random physical data (1–6)

- 🔐 System-safe entropy (CSPRNG)

- The two sources are combined and hashed

**Conceptual model:**

```bash
entropy_final = SHA256(dice_entropy || hex_entropy)

```

✔ Even if one source fails, the other preserves security
✔ Not exclusively dependent on human error
✔ Not exclusively dependent on the system

⚠️ **Important:**
This mode is **not reproducible** if only the dice are annotated.

For future reproduction, the manual mode must be used.

---

## 📇 Word Indexes (BIP39)

Each word in the mnemonic is accompanied by its index in the BIP39 wordlist:

```bash
01. 0001 abandon
02. 1845 ability
03. 0097 able
```

## 🧭 Derivation Path

Mainnet: m/84'/0'/0'
Testnet: m/84'/1'/0'

---

## 🏷️ Addresses

Native SegWit address generation:

```bash
m/84'/0'/0'/0/0 → bc1...

```

---

## 🔎 Compatibility

- Sparrow Wallet
- Electrum
- BlueWallet
- Bitcoin Core

Any BIP39/BIP84 wallet Compatible

---

## ⚠️ Legal Notice

This software is provided “as is,” without warranties.

You are 100% responsible for the use, storage, and security of the generated keys.

---

## 🧠 Threat Model

**This software DOES NOT PROTECT against:**

- Malware in the operating system
- Keyloggers
- Screen capture
- Compromised firmware
- Supply-chain attacks

**This software PROTECTS against:**

- System RNG failures (via physical data)
- Dependence on external services
- Opaque seed generation
- Lack of auditability

For maximum security, use on a clean, temporary, offline computer.

---

## 🛠️ Development Requirements

- Rust 1.89

Check with:

```bash
rustc --version

```

---

## 🙏 Credits

This project is based on well-established Bitcoin standards and the work of the open-source community, especially:

### Bitcoin Improvement Proposals (BIPs)

- **BIP32** — Deterministic Hierarchical Wallets
- **BIP39** — Mnemonic code for deterministic key generation
- **BIP84** — Derivation scheme for native SegWit wallets

These specifications define the basis for deterministic key generation and wallet interoperability.

### Rust Ecosystem

This project uses high-quality open-source Rust libraries, including:

- `bitcoin` — Bitcoin data structures, keys, and derivation
- `bip39` — Mnemonic generation and validation
- `secp256k1` (via `bitcoin`) — Elliptic curve cryptography
- `dialoguer` — Secure and user-friendly interaction via command line
- `console` — Terminal styling and output formatting
- `rand` — Random number generation (using automatic entropy)

All credits belong to the authors and maintainers of these libraries.

### Community

Special thanks to:

- The developers and contributors of **Bitcoin Core**
- The **Bitcoin open source community** in general
- Researchers and developers who prioritize transparency, auditability, and user sovereignty

### Author/Maintainer

- **William C. Canin**

--- This project was built with a strong focus on **security, transparency, and verifiability**, aiming to give users complete control over their Bitcoin keys.
