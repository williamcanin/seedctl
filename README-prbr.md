<p align="center">
  <img src=".github/assets/seedctl.png" alt="SeedCTL" width="350"/>
</p>

[![Build and Release (Linux & Windows)](https://github.com/williamcanin/seedctl/actions/workflows/release.yml/badge.svg)](https://github.com/williamcanin/seedctl/actions/workflows/release.yml)
![Release](https://img.shields.io/github/v/release/williamcanin/seedctl?label=latest&color=blue)
![License](https://img.shields.io/github/license/williamcanin/seedctl)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![Offline](https://img.shields.io/badge/works-offline-important)
![Deterministic](https://img.shields.io/badge/deterministic-yes-success)
![No network](https://img.shields.io/badge/network-none-lightgrey)

🇺🇸 [**Read in English**](README-en.md)

**SeedCTL** é um gerador de carteiras Bitcoin **determinístico, auditável e focado em segurança**, escrito em [**Rust**](https://rust-lang.org/).

Este programa permite gerar uma carteira Bitcoin a partir de **dados físicos (dado/dice) 🎲** e/ou **entropia do sistema**, produzindo:

- Mnemonic BIP39 (12 ou 24 palavras)
- Suporte a **passphrase opcional**
- Derivação **BIP84 (Native SegWit – bc1)**
- Suporte a **Mainnet e Testnet**
- Exibição de [**Word Indexes BIP39**](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt)
- Geração de **endereços determinísticos**

O objetivo principal é permitir **geração segura, verificável e offline** de seeds Bitcoin, com alto nível de paranoia e controle total do processo.

---

## Mirrors

Este repositório é mantido principalmente no **GitHub**.

Um mirror sincronizado está disponível no **GitLab**:

- **GitHub (canonical)**: https://github.com/williamcanin/seedctl
- **GitLab (mirror)**: https://gitlab.com/williamcanin/seedctl

---

## Status do projeto

Indicadores de manutenção e atividade para o repositório canônico do **GitHub**.

![Last commit](https://img.shields.io/github/last-commit/williamcanin/seedctl)
![Issues](https://img.shields.io/github/issues/williamcanin/seedctl)
![Stars](https://img.shields.io/github/stars/williamcanin/seedctl)
![Forks](https://img.shields.io/github/forks/williamcanin/seedctl)

---

## Filosofia de Segurança

- Nenhuma dependência de rede
- Nenhum envio de dados
- Nenhuma persistência em disco
- Ideal para uso **offline / air-gapped**
- Compatível com verificação manual (dice, word indexes, derivation path)
- Separação clara entre **modo determinístico** e **modo híbrido**

> **ATENÇÃO**
> Este programa **exibe informações sensíveis** (mnemonic, passphrase, chaves).
> Utilize **somente em ambiente seguro e offline**. Recomendável usar com [Tails](https://tails.net/)

---

## Funcionalidades

- BIP39 – 12 ou 24 palavras
- Entropia via dados físicos (1–6)
- Entropia híbrida (dados físicos + RNG do sistema)
- Geração automática ou entrada manual de dados
- Confirmação visual da sequência de dados
- Passphrase opcional (BIP39)
- Mainnet e Testnet
- BIP84 (Native SegWit)
- Exibição dos **Word Indexes** (base 1, formato `0001`)
- Geração de endereços `bc1` / `tb1`

---

## Documentação

- **Reprodução determinística de carteiras**
  Veja [`REPRODUCIBILITY.md`](REPRODUCIBILITY.md)

- **Verificação de binários e releases (SHA256 + GPG)**
  Veja [`VERIFYING_RELEASES.md`](VERIFYING_RELEASES.md)

---

## Modos de Entropia

O programa oferece **dois modos distintos**, com objetivos diferentes.

### Modo Manual (Determinístico)

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

Este modo é **100% determinístico e reproduzível**.

---

### Modo Automático (Híbrido)

Indicado para:

- Criar carteiras novas
- Aumentar entropia contra falhas humanas
- Defesa em profundidade

**Como funciona:**

- O programa gera automaticamente:
  - Dados físicos aleatórios (1–6)
  - Entropia segura do sistema (CSPRNG)
- As duas fontes são combinadas e hashadas

**Modelo conceitual:**

```bash
entropy_final = SHA256(dice_entropy || hex_entropy)
```

✔ Mesmo que uma fonte falhe, a outra preserva a segurança
✔ Não depende exclusivamente do humano
✔ Não depende exclusivamente do sistema

**Importante:**
Este modo **não é reproduzível** se apenas o dice for anotado.
Para reprodução futura, o modo manual deve ser utilizado.

---

## Word Indexes (BIP39)

Cada palavra do mnemonic é acompanhada de seu índice na wordlist BIP39:

```bash
01. 0001 abandon
02. 1845 ability
03. 0097 able
```

## Derivation Path

Mainnet: m/84'/0'/0'
Testnet: m/84'/1'/0'

---

## Endereços

Geração de endereços Native SegWit:

```bash
m/84'/0'/0'/0/0 → bc1...
```

---

## Compatibilidade

- Sparrow Wallet
- Electrum
- BlueWallet
- Bitcoin Core

Qualquer wallet BIP39/BIP84 compatível

---

## Aviso Legal

Este software é fornecido “como está”, sem garantias.

Você é 100% responsável pelo uso, armazenamento e segurança das chaves geradas.

---

## Threat Model

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

## Requisitos para desenvolvimento

- Rust 1.89

Verifique com:

```bash
rustc --version
```

---

## Créditos

Este projeto foi construído com base em padrões bem estabelecidos do Bitcoin e no esforço coletivo da comunidade de código aberto.

### Autor e Colaboradores

- **William C. Canin** — Criador e Mantenedor
- **[Seu Nome Aqui]** — Torne-se um colaborador! Envie uma solicitação de pull request ou relate um problema.

### Propostas de Melhoria do Bitcoin (BIPs)

- **BIP32**: Carteiras Hierárquicas Determinísticas.

- **BIP39**: Código mnemônico para geração de chaves determinísticas.

- **BIP84**: Esquema de derivação para carteiras SegWit nativas.

### Ecossistema Rust

O SeedCTL foi construído usando bibliotecas de código aberto de alta qualidade da comunidade Rust. Nos apoiamos nos ombros de gigantes para garantir segurança e desempenho.

Você pode encontrar a lista completa de bibliotecas e suas versões em nosso [Cargo.toml](./Cargo.toml).

Agradecimentos especiais aos mantenedores do `bitcoin`, `bip39` e de todos os outros crates que tornam este projeto possível.

### Agradecimentos à Comunidade

Agradecimentos especiais aos desenvolvedores do **Bitcoin Core** e à comunidade global de código aberto por priorizarem a transparência e a soberania do usuário.

---

## Suporte para este projeto

[![Donate](https://img.shields.io/badge/Donate-Bitcoin%20|%20Pix%20|%20PayPal-F5C400?style=for-the-badge)](
https://github.com/williamcanin/donations
)
[![Sponsor](https://img.shields.io/badge/Sponsor-GitHub-%23ea4aaa?style=for-the-badge)](
https://github.com/sponsors/williamcanin
)

> Você aparecerá nos colaboradores.

---

Este projeto foi construído com um forte foco em **segurança, transparência e verificabilidade**, visando dar aos usuários controle total sobre suas chaves Bitcoin.
