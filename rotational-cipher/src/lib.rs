pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|ch| {
        match ch {
            ch if !ch.is_alphabetic() => ch,
            ch  => {
                let offset = if ch.is_ascii_uppercase() { 65 } else { 97 };
                let code = (ch as i8) - offset; 
                let cipher_code = (code + (key % 26)) % 26;
                (cipher_code + offset) as u8 as char
            }
        }
    }).collect::<String>()
}
