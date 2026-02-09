use bitcoin::{
  base58,
  bip32::{Xpriv, Xpub},
  hashes::{Hash, sha256d},
};

use crossterm::{
  event::{Event, KeyCode, read},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

use console::{Color, style};
use dialoguer::theme::ColorfulTheme;
use rand::RngExt;
use sha2::{Digest, Sha256};

use crate::{meta, slogan};

const BITS_PER_DIE: f64 = 2.584962500721156;

// SLOGAN
pub fn slogan(show_doc: bool, show_version: bool) {
  let mut version = meta::VERSION;

  if !show_version {
    version = "";
  }

  println!(
    "{}{}\n{}",
    style(slogan::slogan())
      .bold()
      .fg(Color::TrueColor(255, 165, 0)),
    style(slogan::program_name_banner(version)).bold().green(),
    style(meta::PROJECT_DESCRIPTION).bold()
  );
  if show_doc {
    println!(
      "{}{}\n",
      style("Documentation: ").bold().yellow(),
      style(format!("{}/README.md", meta::PROJECT_REPOSITORY)).cyan()
    );
  }
}

pub fn dialoguer_theme(arrow: &str) -> ColorfulTheme {
  use console::style;

  let mut theme = ColorfulTheme {
    active_item_prefix: style(arrow.to_string()),
    ..Default::default()
  };

  theme.active_item_prefix = style("►".to_string());
  theme
}

pub fn exit_confirm() {
  #[cfg(target_os = "windows")]
  {
    println!(
      "{}\n",
      style("The program has ended. Press ENTER to exit.")
        .bold()
        .yellow()
    );
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
  }
}

pub fn show_important_card_with_confirm() -> anyhow::Result<bool> {
  use console::{self, style};
  use dialoguer::Confirm;

  // CLEAR TERMINAL
  // let term = console::Term::stdout();
  // term.clear_screen()?;

  println!(
    "\n{}",
    style("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓").red()
  );
  println!("┃{:^60}┃", style(" IMPORTANT! ").bold().red());
  println!(
    "{}",
    style("┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫").red()
  );
  println!("┃ ! DO NOT save your seed in online digital files.           ┃");
  println!("┃                                                            ┃");
  println!("┃ ! If you used a passphrase, memorize it.                   ┃");
  println!("┃   Without it, you will lose access to your wallet.         ┃");
  println!("┃                                                            ┃");
  println!("┃ ! As soon as you generate your seed, write it down         ┃");
  println!("┃   TEMPORARILY on a piece of paper and then use a           ┃");
  println!("┃   Cold Wallet or Steel Wallet.                             ┃");
  println!("┃                                                            ┃");
  println!("┃ ! Finally, exit the program.                               ┃");
  println!(
    "{}",
    style("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛").red()
  );

  println!();

  let confirmed = Confirm::with_theme(&dialoguer_theme("►"))
    .with_prompt("I have read and understood all the recommendations above.")
    .default(false)
    .interact()?;

  Ok(confirmed)
}

pub fn copyright_bottom() {
  let line_size = 50;
  println!("\n{}", "-".repeat(line_size));
  println!(
    "{}",
    console::style(format!(
      "{} © {} {} and collaborators.",
      meta::PROJECT_NAME,
      meta::COPYRIGHT_YEAR,
      meta::PROJECT_MAINTAINER
    ))
    .bold(),
  );
  println!("{}", "-".repeat(line_size));
}

// NETWORK / SECURITY

pub fn ensure_offline() {
  use if_addrs::get_if_addrs;

  let interfaces = match get_if_addrs() {
    Ok(ifaces) => ifaces,
    Err(_) => return, // If you can't read it, DO NOT block it.
  };

  for iface in interfaces {
    // ignore loopback
    if iface.is_loopback() {
      continue;
    }

    // If any non-loopback interface is encountered → ABORT
    eprintln!(
      "\n{}\n{}\n",
      console::style("[ SECURITY ABORT ]").bold().red(),
      console::style(
        "Active network interface detected.\n\
         This program MUST be used offline / air-gapped.\n\n\
         Disable Wi-Fi, Ethernet, VPNs and try again."
      )
      .yellow()
    );
    copyright_bottom();
    std::process::exit(1);
  }
}

// ENTROPY

pub fn read_manual_dice_with_feedback(bits_target: usize) -> Vec<u8> {
  use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{Clear, ClearType},
  };

  enable_raw_mode().unwrap();
  execute!(io::stdout(), Hide).unwrap();

  let mut dice: Vec<u8> = Vec::new();

  println!("\n[ Enter dice sequence (1–6) ]");

  loop {
    if let Event::Key(event) = read().unwrap() {
      match event.code {
        KeyCode::Char(c) if ('1'..='6').contains(&c) => {
          dice.push(c.to_digit(10).unwrap() as u8);
        }
        KeyCode::Backspace => {
          dice.pop();
        }
        KeyCode::Enter => {
          break;
        }
        _ => {}
      }

      let dice_count = dice.len();
      let bits = (dice_count as f64) * BITS_PER_DIE;
      let ready = bits >= bits_target as f64;

      let status = if ready {
        "✔ enough"
      } else {
        "… not enough"
      };

      let dice_str: String = dice.iter().map(|d| char::from(b'0' + *d)).collect();

      // Rewrite ONLY the current line
      print!("\r");
      execute!(io::stdout(), Clear(ClearType::CurrentLine)).unwrap();

      print!(
        "> Dice: {:3} | Bits: {:7.2} / {:3} | {} | [{}]",
        dice_count, bits, bits_target, status, dice_str
      );

      io::stdout().flush().unwrap();
    }
  }

  execute!(io::stdout(), Show).unwrap();
  disable_raw_mode().unwrap();
  println!();

  dice
}

pub fn dice_hash(dice: &[u8]) -> Vec<u8> {
  Sha256::digest(dice).to_vec()
}

pub fn generate_system_entropy(bytes: usize) -> Vec<u8> {
  let mut rng = rand::rng();
  (0..bytes).map(|_| rng.random::<u8>()).collect()
}

pub fn combine_entropy(a: &[u8], b: &[u8]) -> Vec<u8> {
  let mut hasher = Sha256::new();
  hasher.update(a);
  hasher.update(b);
  hasher.finalize().to_vec()
}

pub fn format_key_origin(fingerprint: [u8; 4], purpose: u32, coin_type: u32) -> String {
  format!(
    "[{:02x}{:02x}{:02x}{:02x}/{}h/{}h/0h]",
    fingerprint[0], fingerprint[1], fingerprint[2], fingerprint[3], purpose, coin_type
  )
}

pub fn output_descriptor(purpose: u32, key_origin: &str, xpub: &str, chain: u32) -> String {
  match purpose {
    // BIP84
    84 => format!("wpkh({}{}/{chain}/*)", key_origin, xpub),
    // BIP49
    49 => format!("sh(wpkh({}{}/{chain}/*))", key_origin, xpub),
    // BIP44
    44 => format!("pkh({}{}/{chain}/*)", key_origin, xpub),
    _ => unreachable!(),
  }
}

pub fn truncate_entropy(entropy: &[u8], bits: usize) -> Vec<u8> {
  entropy[..bits / 8].to_vec()
}

pub fn required_dice(bits: usize) -> usize {
  ((bits as f64) / BITS_PER_DIE).ceil() as usize
}

pub fn generate_random_dice(count: usize) -> Vec<u8> {
  let mut rng = rand::rng();
  (0..count).map(|_| rng.random_range(1..=6)).collect()
}

// KEY FORMATTERS

pub fn xprv_to_zprv(xprv: &Xpriv) -> String {
  let mut data = xprv.encode();
  data[0..4].copy_from_slice(&[0x04, 0xB2, 0x43, 0x0C]);
  base58::encode_check(&data)
}

pub fn xpub_to_zpub(xpub: &Xpub) -> String {
  let mut data = xpub.encode();
  data[0..4].copy_from_slice(&[0x04, 0xB2, 0x47, 0x46]);
  base58::encode_check(&data)
}

pub fn xprv_to_yprv(xprv: &Xpriv) -> String {
  let mut data = xprv.encode();
  // yprv prefix
  data[0..4].copy_from_slice(&[0x04, 0x9D, 0x78, 0x78]);
  base58::encode_check(&data)
}

/// Converte xpub → ypub / zpub (SLIP-132)
pub fn convert_xpub_prefix(xpub: &Xpub, version: u32) -> String {
  // Decode Base58Check
  let mut data = base58::decode_check(&xpub.to_string()).expect("Invalid Base58Check xpub");

  // Substitui version bytes
  data[0..4].copy_from_slice(&version.to_be_bytes());

  // Recalcula checksum
  let checksum = sha256d::Hash::hash(&data[..data.len() - 4]);

  let len = data.len();
  data[len - 4..len].copy_from_slice(&checksum[..4]);

  // Encode Base58Check
  base58::encode_check(&data)
}

pub fn xpub_to_ypub(xpub: &Xpub) -> String {
  convert_xpub_prefix(xpub, 0x049d7cb2) // ypub
}
