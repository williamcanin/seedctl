# SeedCTL

🇺🇸 [**Read in English**](README-en.md)

Gerador de carteiras Bitcoin **determinístico, auditável e focado em segurança**, escrito em [**Rust**](https://rust-lang.org/).

Este programa permite gerar uma carteira Bitcoin a partir de **dados físicos (dados/dice)** e/ou **entropia do sistema**, produzindo:

- Mnemonic BIP39 (12 ou 24 palavras)
- Suporte a **passphrase opcional**
- Derivação **BIP84 (Native SegWit – bc1)**
- Suporte a **Mainnet e Testnet**
- Exibição de [**Word Indexes BIP39**](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt)
- Geração de **endereços determinísticos**

O objetivo principal é permitir **geração segura, verificável e offline** de seeds Bitcoin, com alto nível de paranoia e controle total do processo.

---

## 🔐 Filosofia de Segurança

- Nenhuma dependência de rede
- Nenhum envio de dados
- Nenhuma persistência em disco
- Ideal para uso **offline / air-gapped**
- Compatível com verificação manual (dice, word indexes, derivation path)
- Separação clara entre **modo determinístico** e **modo híbrido**

> ⚠️ **ATENÇÃO**
> Este programa **exibe informações sensíveis** (mnemonic, passphrase, chaves).
> Utilize **somente em ambiente seguro e offline**. Recomendável usar com [Tails](https://tails.net/)

---

## ✨ Funcionalidades

- ✅ BIP39 – 12 ou 24 palavras
- 🎲 Entropia via dados físicos (1–6)
- 🔀 Entropia híbrida (dados físicos + RNG do sistema)
- 🔁 Geração automática ou entrada manual de dados
- 🔍 Confirmação visual da sequência de dados
- 🔐 Passphrase opcional (BIP39)
- 🌐 Mainnet e Testnet
- 🧭 BIP84 (Native SegWit)
- 📇 Exibição dos **Word Indexes** (base 1, formato `0001`)
- 🏷️ Geração de endereços `bc1` / `tb1`

---

## 📚 Documentação

- 🔎 **Reprodução determinística de carteiras**
  Veja [`REPRODUCIBILITY.md`](REPRODUCIBILITY.md)

- 🔐 **Verificação de binários e releases (SHA256 + GPG)**
  Veja [`VERIFYING_RELEASES.md`](VERIFYING_RELEASES.md)

---

## 🎲 Modos de Entropia

O programa oferece **dois modos distintos**, com objetivos diferentes.

### 1️⃣ Modo Manual (Determinístico)

Indicado para:

- Recuperar uma carteira existente
- Auditoria
- Cerimônias de geração reproduzíveis
- Verificação independente

**Como funciona:**

- O usuário informa manualmente a sequência de dados (1–6)
- Nenhuma entropia do sistema é utilizada
- A mesma sequência + mesma passphrase ⇒ **sempre a mesma carteira**

**Modelo conceitual:**

```bash
entropy = SHA256(dice_entropy)
```

📌 Este modo é **100% determinístico e reproduzível**.

---

### 2️⃣ Modo Automático (Híbrido)

Indicado para:

- Criar carteiras novas
- Aumentar entropia contra falhas humanas
- Defesa em profundidade

**Como funciona:**

- O programa gera automaticamente:
  - 🎲 Dados físicos aleatórios (1–6)
  - 🔐 Entropia segura do sistema (CSPRNG)
- As duas fontes são combinadas e hashadas

**Modelo conceitual:**

```bash
entropy_final = SHA256(dice_entropy || hex_entropy)
```

✔ Mesmo que uma fonte falhe, a outra preserva a segurança
✔ Não depende exclusivamente do humano
✔ Não depende exclusivamente do sistema

⚠️ **Importante:**
Este modo **não é reproduzível** se apenas o dice for anotado.
Para reprodução futura, o modo manual deve ser utilizado.

---

## 📇 Word Indexes (BIP39)

Cada palavra do mnemonic é acompanhada de seu índice na wordlist BIP39:

```bash
01. 0001 abandon
02. 1845 ability
03. 0097 able
```

## 🧭 Derivation Path

Mainnet: m/84'/0'/0'
Testnet: m/84'/1'/0'

---

## 🏷️ Endereços

Geração de endereços Native SegWit:

```bash
m/84'/0'/0'/0/0 → bc1...
```

---

## 🔎 Compatibilidade

- Sparrow Wallet
- Electrum
- BlueWallet
- Bitcoin Core

Qualquer wallet BIP39/BIP84 compatível

---

## ⚠️ Aviso Legal

Este software é fornecido “como está”, sem garantias.

Você é 100% responsável pelo uso, armazenamento e segurança das chaves geradas.

---

## 🧠 Threat Model

**Este software NÃO PROTEGE contra:**

- Malware no sistema operacional
- Keyloggers
- Screen capture
- Firmware comprometido
- Supply-chain attacks

**Este software PROTEGE contra:**

- Falhas de RNG do sistema (via dados físicos)
- Dependência de serviços externos
- Seed generation opaca
- Falta de auditabilidade

Para máxima segurança, use em um computador offline, limpo e temporário.

---

## 🛠️ Requisitos para desenvolvimento

- Rust 1.89

Verifique com:

```bash
rustc --version
```

---

## 🙏 Créditos

Este projeto é baseado em padrões bem estabelecidos do Bitcoin e no trabalho da comunidade de código aberto, especialmente:

### Propostas de Melhoria do Bitcoin (BIPs)

- **BIP32** — Carteiras Hierárquicas Determinísticas
- **BIP39** — Código mnemônico para geração de chaves determinísticas
- **BIP84** — Esquema de derivação para carteiras SegWit nativas

Essas especificações definem a base para a geração de chaves determinísticas e a interoperabilidade de carteiras.

### Ecossistema Rust

Este projeto utiliza bibliotecas Rust de código aberto de alta qualidade, incluindo:

- `bitcoin` — Estruturas de dados, chaves e derivação do Bitcoin
- `bip39` — Geração e validação de mnemônicos
- `secp256k1` (via `bitcoin`) — Criptografia de curva elíptica
- `dialoguer` — Interação segura e amigável via linha de comando
- `console` — Estilização do terminal e formatação de saída
- `rand` — Geração de números aleatórios (ao usar entropia automática)

Todos os créditos são dos autores e mantenedores dessas bibliotecas.

### Comunidade

Agradecimentos especiais a:

- Os desenvolvedores e colaboradores do **Bitcoin Core**
- A **comunidade de código aberto do Bitcoin** em geral
- Pesquisadores e desenvolvedores que priorizam a transparência, a auditabilidade e a soberania do usuário

### Autor/Mantenedor

- **William C. Canin**

---
Este projeto foi construído com um forte foco em **segurança, transparência e verificabilidade**, visando dar aos usuários controle total sobre suas chaves Bitcoin.
