use bitcoin::{
  base58,
  bip32::{Xpriv, Xpub},
};
use rand::Rng;
use sha2::{Digest, Sha256};

const BITS_PER_DIE: f64 = 2.584962500721156;

// ENTROPY FUNCTIONS

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
