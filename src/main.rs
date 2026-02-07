// Author: William C. Canin <https://williamcanin.github.io>

mod utils;
mod version;

use bip39::Mnemonic;

use console::style;
use dialoguer::{Confirm, Input, Select};

use bitcoin::{
  Address, Network,
  bip32::{ChildNumber, DerivationPath, Xpriv, Xpub},
  secp256k1::Secp256k1,
};

fn main() {
  println!(
    "\n{}\n",
    style(format!("SeedCTL - version {}", version::VERSION))
      .bold()
      .cyan()
  );

  println!(
    "{}\n\n",
    style("A deterministic, auditable, and security-focused Bitcoin wallet generator.").bold()
  );

  // MNEMONIC SIZE

  let mnemonic_choice = Select::new()
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

  let dice_mode = Select::new()
    .with_prompt("[ Dice (1-6) ]")
    .items(["Auto (random)", "Manual (inform sequence)"])
    .default(0)
    .interact()
    .unwrap();

  let dice: Vec<u8> = match dice_mode {
    0 => utils::generate_random_dice(min_dice),
    1 => {
      let input: String = Input::new()
        .with_prompt("Enter the data sequence (1–6)")
        .interact_text()
        .unwrap();

      let dice: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid entry") as u8)
        .collect();

      if dice.iter().any(|&d| !(1..=6).contains(&d)) {
        panic!("Only numbers from 1 to 6.");
      }

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
  println!(
    "{} {}\n",
    style(format!("DICE USED ({} numbers):", dice.len()))
      .bold()
      .yellow(),
    dice_str
  );

  if !Confirm::new()
    .with_prompt("Please confirm that the above information is correct.")
    .interact()
    .unwrap()
  {
    panic!("Aborted by the user");
  }

  // NETWORK

  let network_choice = Select::new()
    .with_prompt("\nNetwork")
    .items(["Mainnet", "Testnet"])
    .default(0)
    .interact()
    .unwrap();

  let (network, coin_type) = match network_choice {
    0 => (Network::Bitcoin, 0),
    1 => (Network::Testnet, 1),
    _ => unreachable!(),
  };

  // PASSPHRASE

  let passphrase_title = style("\nPassphrase (enter = empty)")
    .bold()
    .yellow()
    .to_string();

  let passphrase: String = Input::new()
    .with_prompt(passphrase_title)
    .allow_empty(true)
    .interact_text()
    .unwrap();

  // CRYPTO CORE (FIXED)

  let dice_entropy = utils::dice_hash(&dice);

  let final_entropy = match dice_mode {
    // AUTO → HYBRID
    0 => {
      let system_entropy = utils::generate_system_entropy(32);
      let combined = utils::combine_entropy(&dice_entropy, &system_entropy);
      println!(
        "{}",
        style("Entropy mode: HYBRID (dice + system RNG)").bold()
      );
      utils::truncate_entropy(&combined, bits)
    }

    // MANUAL → DETERMINISTIC
    1 => {
      println!(
        "{}",
        style("Entropy mode: DETERMINISTIC (dice only)").bold()
      );
      utils::truncate_entropy(&dice_entropy, bits)
    }

    _ => unreachable!(),
  };

  let mnemonic = Mnemonic::from_entropy(&final_entropy).unwrap();
  let words: Vec<&str> = mnemonic.words().collect();
  let indices = mnemonic.word_indices();
  let seed = mnemonic.to_seed(&passphrase);

  let secp = Secp256k1::new();
  let master = Xpriv::new_master(network, &seed).unwrap();

  let path: DerivationPath = format!("m/84'/{}'/0'", coin_type).parse().unwrap();
  let acc_xprv = master.derive_priv(&secp, &path).unwrap();
  let acc_xpub = Xpub::from_priv(&secp, &acc_xprv);

  // OUTPUT

  println!("\n\n\n{}\n", style("OUTPUT:").bold().blue());

  println!(
    "\n{}\n",
    style("[ IMPORTANT! Write down your BIP39 mnemonic and your passphrase if you used one. ]")
      .bold()
      .magenta()
  );

  println!("\n{}\n", style("POSITION  INDEXES  WORDS").bold());

  for (i, (word, idx)) in words.iter().zip(indices).enumerate() {
    println!(
      "{:02}.  {:04}  {}",
      i + 1,
      idx + 1,
      style(word).bold().yellow()
    );
  }

  println!("{} {}", style("\nDerivation path:").bold(), path);

  println!(
    "{} {}",
    style("ZPRV:").bold(),
    utils::xprv_to_zprv(&acc_xprv)
  );

  println!(
    "{} {}",
    style("ZPUB:").bold(),
    utils::xpub_to_zpub(&acc_xpub)
  );

  println!("\n{}", style("Address BIP84").bold());

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

    let addr = Address::p2wpkh(&bitcoin::CompressedPublicKey(child.public_key), network);
    println!("m/84'/{}'/0'/0/{} → {}", coin_type, i, addr);
  }
}
