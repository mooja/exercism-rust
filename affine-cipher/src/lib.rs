/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn ch_index(c: char) -> u32 {
    ASCII_LOWER.iter().position(|x| *x == c).unwrap() as u32
}

fn index_to_ch(idx: u32) -> char {
    ASCII_LOWER[idx as usize]
}

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while a != b {
        let greater = a.max(b);
        let smaller = a.min(b);
        a = greater - smaller;
        b = smaller;
    }
    a
}

fn coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

fn mod_inverse(a: u32, m: u32) -> Option<u32> {
    for b in 1..m {
        if (a * b) % 26 == 1 {
            return Some(b);
        }
    }
    None
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a as u32, ASCII_LOWER.len() as u32) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let encrypt = |c: char| -> char {
        if c.is_ascii_digit() {
            return c;
        }

        let idx = ch_index(c);
        let a = a as u32;
        let b = b as u32;
        let cipher_idx = (a * idx + b) % 26;
        index_to_ch(cipher_idx)
    };

    let ciphertext = plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(encrypt)
        .collect::<String>();

    let mut spaced_ciphertext = String::new();
    for (idx, c) in ciphertext.chars().enumerate() {
        if idx % 5 == 0 && idx != 0 {
            spaced_ciphertext.push(' ');
        }
        spaced_ciphertext.push(c);
    }

    Ok(spaced_ciphertext)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let a_inverse: u32 = match mod_inverse(a as u32, ASCII_LOWER.len() as u32) {
        None => return Err(AffineCipherError::NotCoprime(a)),
        Some(n) => n,
    };


    // decode a character
    let m = ASCII_LOWER.len() as u32;
    let b = b as u32;
    let plaintext = ciphertext
        .chars()
        .filter(|c| *c != ' ')
        .map(|c| {
            if !c.is_alphabetic() {
                return c;
            }

            let mut idx: u32 = ASCII_LOWER.iter().position(|&x| x == c).unwrap() as u32;
            while b > idx {
                idx += 26;
            }

            let plaintext_idx = (a_inverse * ((idx - b) % m)) % m;
            ASCII_LOWER[plaintext_idx as usize]
        })
        .collect::<String>();

    Ok(plaintext)
}