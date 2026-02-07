// Author: William C. Canin <https://williamcanin.github.io>

mod slogan;
mod utils;
mod version;

use bip39::Mnemonic;

use console::{Color, style};
use dialoguer::{Confirm, Input, Select};

use bitcoin::{
  Address, Network,
  bip32::{ChildNumber, DerivationPath, Xpriv, Xpub},
  secp256k1::Secp256k1,
};

fn main() {
  // SLOGAN
  println!(
    "{}{}\n{}\n{}{}\n",
    style(slogan::slogan())
      .bold()
      .fg(Color::TrueColor(255, 165, 0)),
    style(slogan::banner(version::VERSION)).bold().green(),
    style("A deterministic, auditable, and security-focused Bitcoin wallet generator.").bold(),
    style("Documentation: ").bold().yellow(),
    style("https://github.com/williamcanin/seedctl/README.md").cyan()
  );

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
  println!(
    "{} {}\n",
    style(format!("DICE USED ({} numbers):", dice.len()))
      .bold()
      .yellow(),
    dice_str
  );

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

  let passphrase_title = style("[Optional] Passphrase (enter = skip)")
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

  // OUTPUT / Your wallet

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

  println!("{} {}", style("\nDerivation path:").bold(), path);

  println!(
    "{} {}",
    style("\nZPRV:").bold(),
    utils::xprv_to_zprv(&acc_xprv)
  );

  println!(
    "{} {}",
    style("\nZPUB:").bold(),
    utils::xpub_to_zpub(&acc_xpub)
  );

  println!("\n{}", style("Address BIP84:").bold());

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

  println!("\n{}\n", style("-".repeat(60)).bold().blue());

  utils::finished();
}
