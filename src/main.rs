// Author: William C. Canin <https://williamcanin.github.io>

mod args;
mod export;
mod meta;
mod slogan;
mod utils;

use bip39::Mnemonic;
use console::style;
use dialoguer::{Confirm, Input, Select};
use std::error::Error;

use bitcoin::{
  Address, Network,
  bip32::{ChildNumber, DerivationPath, Xpriv, Xpub},
  secp256k1::Secp256k1,
};
use serde_json::to_string_pretty;

fn main() -> Result<(), Box<dyn Error>> {
  // FLAGS
  match args::parse_args() {
    args::CliAction::Version => {
      args::print_version();
      return Ok(());
    }
    args::CliAction::About => {
      args::print_about();
      return Ok(());
    }
    args::CliAction::Run => {}
  }

  // SLOGAN
  utils::slogan(true, true);

  // SECURITY CHECK — MUST BE FIRST
  // utils::ensure_offline();

  // SECURITY CARD — CONFIRM TO PROCEED
  let confirmed = utils::show_important_card_with_confirm()?;
  if !confirmed {
    eprintln!("Error: User did not confirm reading the recommendations.");
    utils::copyright_bottom();
    std::process::exit(1);
  }

  // MNEMONIC SIZE
  let mnemonic_choice = Select::with_theme(&utils::dialoguer_theme("►"))
    .with_prompt("[ Mnemonic size (seed) ]")
    .items(["12 words (128 bits)", "24 words (256 bits)"])
    .default(0)
    .interact()
    .unwrap();

  let bits = match mnemonic_choice {
    0 => 128,
    1 => 256,
    _ => unreachable!(),
  };

  let min_dice = utils::required_dice(bits);
  println!("{} {} bits\n", style("Selected entropy:").bold(), bits);

  // DICE MODE
  let dice_mode = Select::with_theme(&utils::dialoguer_theme("►"))
    .with_prompt("[ Dice (1-6) ]")
    .items(["Auto (random)", "Manual (inform sequence)"])
    .default(0)
    .interact()
    .unwrap();

  let dice: Vec<u8> = match dice_mode {
    0 => utils::generate_random_dice(min_dice),
    1 => {
      let dice = utils::read_manual_dice_with_feedback(bits);

      if dice.len() < min_dice {
        panic!(
          "Insufficient data: {} provided, minimum {}",
          dice.len(),
          min_dice
        );
      }

      dice
    }
    _ => unreachable!(),
  };

  // VISUAL CONFIRMATION
  let dice_str: String = dice.iter().map(|d| char::from(b'0' + d)).collect();
  println!("{} {}\n", style("DICE USED:").bold().yellow(), dice_str);

  if !Confirm::with_theme(&utils::dialoguer_theme("►"))
    .with_prompt("Please confirm that the above information is correct.")
    .interact()
    .unwrap()
  {
    panic!("Aborted by the user");
  }

  // NETWORK
  let network_choice = Select::with_theme(&utils::dialoguer_theme("►"))
    .with_prompt("Network")
    .items(["Bitcoin (Mainnet)", "Bitcoin (Testnet)"])
    .default(0)
    .interact()
    .unwrap();

  let (network, coin_type) = match network_choice {
    0 => (Network::Bitcoin, 0),
    1 => (Network::Testnet, 1),
    _ => unreachable!(),
  };

  // PASSPHRASE
  let passphrase_title = style("[Optional] Passphrase (enter = empty)")
    .bold()
    .yellow()
    .to_string();

  let passphrase: String = Input::with_theme(&utils::dialoguer_theme("►"))
    .with_prompt(passphrase_title)
    .allow_empty(true)
    .interact_text()
    .unwrap();

  // CRYPTO CORE (FIXED)
  let dice_entropy = utils::dice_hash(&dice);
  let final_entropy = match dice_mode {
    // Auto → Hybrid
    0 => {
      let system_entropy = utils::generate_system_entropy(32);
      let combined = utils::combine_entropy(&dice_entropy, &system_entropy);
      println!(
        "{}",
        style("Entropy mode: HYBRID (dice + system RNG)").bold()
      );
      utils::truncate_entropy(&combined, bits)
    }

    //  Manual → Deterministic
    1 => {
      println!(
        "{}",
        style("Entropy mode: DETERMINISTIC (dice only)").bold()
      );
      utils::truncate_entropy(&dice_entropy, bits)
    }

    _ => unreachable!(),
  };

  // ADDRESS TYPE
  let address_type = Select::with_theme(&utils::dialoguer_theme("►"))
    .with_prompt("Address type")
    .items([
      "Native SegWit (BIP84) (recommended)",
      "Nested SegWit (BIP49)",
      "Legacy (BIP44)",
    ])
    .default(0)
    .interact()
    .unwrap();

  let (purpose, addr_label) = match address_type {
    0 => (84, "Address BIP84 (Native SegWit)"),
    1 => (49, "Address BIP49 (Nested SegWit)"),
    2 => (44, "Address BIP44 (Legacy)"),
    _ => unreachable!(),
  };

  let mnemonic = Mnemonic::from_entropy(&final_entropy).unwrap();
  println!(
    "{} {}",
    style("Mnemonic checksum:").bold(),
    style("valid (BIP39)").green()
  );
  let words: Vec<&str> = mnemonic.words().collect();
  let indices = mnemonic.word_indices();
  let seed = mnemonic.to_seed(&passphrase);

  let secp = Secp256k1::new();
  let master = Xpriv::new_master(network, &seed).unwrap();

  let path: DerivationPath = format!("m/{}'/{}'/0'", purpose, coin_type).parse().unwrap();
  let acc_xprv = master.derive_priv(&secp, &path).unwrap();
  let acc_xpub = Xpub::from_priv(&secp, &acc_xprv);

  let account_xpub = match address_type {
    0 => utils::xpub_to_zpub(&acc_xpub), // BIP84
    1 => utils::xpub_to_ypub(&acc_xpub), // BIP49
    2 => acc_xpub.to_string(),           // BIP44 (xpub)
    _ => unreachable!(),
  };

  let account_xprv = match address_type {
    0 => utils::xprv_to_zprv(&acc_xprv), // BIP84 → zprv
    1 => utils::xprv_to_yprv(&acc_xprv), // BIP49 → yprv
    2 => acc_xprv.to_string(),           // BIP44 → xprv
    _ => unreachable!(),
  };

  let fingerprint = master.fingerprint(&secp);
  let fingerprint_bytes = [
    fingerprint[0],
    fingerprint[1],
    fingerprint[2],
    fingerprint[3],
  ];

  let key_origin = utils::format_key_origin(fingerprint_bytes, purpose, coin_type);

  let desc_receive = utils::output_descriptor(purpose, &key_origin, &account_xpub, 0);

  let desc_change = utils::output_descriptor(purpose, &key_origin, &account_xpub, 1);

  // OUTPUT / YOUR WALLET
  println!(
    "\n\n{}\n",
    style(format!("Your wallet: {}", "-".repeat(46)))
      .bold()
      .blue()
  );

  println!("{}\n", style("POSITION  INDEXES  SEED").bold());

  for (i, (word, idx)) in words.iter().zip(indices).enumerate() {
    println!(
      "{:02}.  {:04}  {}",
      i + 1,
      idx + 1,
      style(word).bold().yellow()
    );
  }

  // println!("{} {}", style("\nDerivation path:").bold(), path);
  println!(
    "{} m/{}'/{}'/0'",
    style("\nDerivation path:").bold(),
    purpose,
    coin_type
  );

  println!(
    "\n{} {:02x}{:02x}{:02x}{:02x}",
    style("Master fingerprint:").bold(),
    fingerprint[0],
    fingerprint[1],
    fingerprint[2],
    fingerprint[3],
  );

  println!(
    "{} {}",
    style("\nAccount Private Key:").bold(),
    account_xprv
  );

  println!("{} {}", style("\nAccount Public Key:").bold(), account_xpub);

  println!("\n{}", style("Output Descriptor (receive):").bold());
  println!("{}", desc_receive);

  println!("\n{}", style("Output Descriptor (change):").bold());
  println!("{}", desc_change);

  println!("\n{}", style(addr_label).bold());

  for i in 0..10 {
    let child = acc_xpub
      .derive_pub(
        &secp,
        &[
          ChildNumber::Normal { index: 0 },
          ChildNumber::Normal { index: i },
        ],
      )
      .unwrap();

    let addr = match address_type {
      // BIP84 – Native SegWit
      0 => Address::p2wpkh(&bitcoin::CompressedPublicKey(child.public_key), network),

      // BIP49 – Nested SegWit
      1 => Address::p2shwpkh(&bitcoin::CompressedPublicKey(child.public_key), network),

      // BIP44 – Legacy
      2 => {
        let pk = bitcoin::PublicKey::new(child.public_key);
        Address::p2pkh(pk, network)
      }

      _ => unreachable!(),
    };

    println!("m/{}'/{}'/0'/0/{} → {}", purpose, coin_type, i, addr);
  }

  let script_type = match purpose {
    84 => "bip84",
    49 => "bip49",
    44 => "bip44",
    _ => unreachable!(),
  };

  let watch_only = true; // ou true se remover xprv

  let export = export::WalletExport {
    software: export::SoftwareInfo {
      name: "seedctl".to_string(),
      version: meta::VERSION.to_string(),
      repository: meta::PROJECT_REPOSITORY.to_string(),
    },
    network: match network {
      Network::Bitcoin => "bitcoin".to_string(),
      Network::Testnet => "testnet".to_string(),
      _ => "unknown".to_string(),
    },
    script_type: script_type.to_string(),
    key_origin: export::KeyOrigin {
      fingerprint: format!(
        "{:02x}{:02x}{:02x}{:02x}",
        fingerprint[0], fingerprint[1], fingerprint[2], fingerprint[3],
      ),
      derivation_path: format!("m/{}'/{}'/0'", purpose, coin_type),
    },
    watch_only,
    keys: export::Keys {
      account_xpub: account_xpub.clone(),
      account_xprv: if watch_only {
        None
      } else {
        Some(account_xprv.clone())
      },
    },
    descriptors: export::Descriptors {
      receive: desc_receive,
      change: desc_change,
    },
  };

  let json = to_string_pretty(&export).unwrap();

  println!("\n{}", style("Export JSON:").bold());
  println!("{}", json);

  println!("\n{}\n", style("-".repeat(60)).bold().blue());

  utils::copyright_bottom();
  utils::exit_confirm();

  Ok(())
}
