pub fn mirror_char(c: char) -> char {
    match c {
        '0'..='9' | ' ' => return c,
        _ => ()
    }

    let is_capital = c > 'A' && c <= 'Z';
    let offset = if is_capital { 65 } else { 97 };
    let ch_rank = c as u8 - offset;
    let encrypted_rank = 25 - ch_rank;
    (97 + encrypted_rank) as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let ciphertext = plain
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| mirror_char(c))
        .collect::<Vec<char>>();
    
    let mut rv = String::new();
    for (i, c) in ciphertext.iter().enumerate() {
        if i != 0 && i % 5 == 0 {
            rv.push(' ');
        }
        rv.push(*c);
    }

    rv
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| mirror_char(c))
        .collect()
}
