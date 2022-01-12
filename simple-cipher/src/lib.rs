extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn is_alphabetic_lowercase(ch: char) -> bool {
    (ch as u8) >= 'a' as u8
        && (ch as u8) <= 'z' as u8
}

fn encode_ch(key: char, ch: char) -> char {
    let shift: u8 = key as u8 - 'a' as u8;
    let ch = ch as u8 - 'a' as u8;
    let encrypted = (ch + shift) % 26;

    (encrypted + 'a' as u8) as char
}

fn decrypt_ch(key: char, ch: char) -> char {
    let shift: u8 = key as u8 - 'a' as u8;
    let ch = ch as u8 - 'a' as u8;
    let decrypted = (ch + (26 - shift)) % 26;

    (decrypted + 'a' as u8) as char
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.len() == 0 || s.len() == 0 {
        return None;
    }

    if !key.chars().all(is_alphabetic_lowercase) {
        return None;
    }

    if !s.chars().all(is_alphabetic_lowercase) {
        return None;
    }

    let encrypted: String = key.chars().cycle()
        .zip(s.chars())
        .map(|(k, ch)| encode_ch(k, ch))
        .collect();

    Some(encrypted)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.len() == 0 {
        return None;
    }

    if !key.chars().all(is_alphabetic_lowercase) {
        return None;
    }

    if !s.chars().all(is_alphabetic_lowercase) {
        return None;
    }

    let decrypted: String = key.chars().cycle()
        .zip(s.chars())
        .map(|(k, ch)| decrypt_ch(k, ch))
        .collect();

    Some(decrypted)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let k_iter = (0..100).map(|_| {
      let i = rng.gen_range(0..26) as u8;
      ('a' as u8 + i) as char
    });
    let key_str = k_iter.collect::<String>();
    let cipher_text = encode(&key_str, s).unwrap();

    (key_str, cipher_text)
}
