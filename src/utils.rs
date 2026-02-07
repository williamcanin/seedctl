use bitcoin::{
  base58,
  bip32::{Xpriv, Xpub},
};
use crossterm::{
  event::{Event, KeyCode, read},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

use dialoguer::theme::ColorfulTheme;
use rand::Rng;
use sha2::{Digest, Sha256};

const BITS_PER_DIE: f64 = 2.584962500721156;

pub fn dialoguer_theme(arrow: &str) -> ColorfulTheme {
  use console::style;

  let mut theme = ColorfulTheme {
    active_item_prefix: style(arrow.to_string()),
    ..Default::default()
  };

  theme.active_item_prefix = style("►".to_string());
  theme
}

pub fn finished() {
  use console::style;

  println!(
    "\n{}\n",
    style("[ IMPORTANT! Before you leave, write down your mnemonic BIP39 (seed) and your Passphrase (if you used one). ]")
      .bold()
      .magenta()
  );

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
