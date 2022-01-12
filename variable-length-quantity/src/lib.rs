#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

use std::primitive::u8;
use std::collections::VecDeque;


fn num_to_octets(n: u32) -> Vec<u8> {
    let mut n = n;
    let mut octets: VecDeque<u8> = VecDeque::new() ;

    let last_7_bits_mask: u32 = 0b0111_1111;
    let fst_bit_mask: u8 = 0b1000_0000;

    let mut seen_gsd = false;
    while n != 0 {
        let septet = (n & last_7_bits_mask) as u8;
        if septet != 0 {
            seen_gsd = true;
        }

        if seen_gsd {
            octets.push_front(septet | fst_bit_mask );
        }
        n = n >> 7;
    }


    if octets.iter().all(|&x| x == 0) {
        return vec![0]
    }

    // flip front bit to 0 on last octet
    let mut octets = octets.into_iter().collect::<Vec<u8>>();
    let last_entry = octets.last_mut().unwrap();
    *last_entry &= 0b0111_1111;
    octets
}

fn num_to_octets2(n: u32) -> Vec<u8> {
    let as_bin_str = format!("{:b}", n);
    let septets = as_bin_str
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .rev() // string reversed
        .collect::<Vec<char>>()
        .chunks(7) // string split into chunks of 7 characters
        .map(|chunk| {
            // chunks are reversed back and convered to u8's
            let s = chunk.into_iter().rev().collect::<String>();
            u8::from_str_radix(&s, 2).unwrap()
        })
        .rev() // u8's are converted back to correct order
        .collect::<Vec<u8>>();
    let mut rv = vec![];
    let mut seen_gsd = false;
    for septet in septets {
        if septet != 0 {
            seen_gsd = true;
        }

        if seen_gsd {
            rv.push(septet | 0b10000000)
        }
    }

    if !seen_gsd {
        rv.push(0u8);
    }

    let last = rv.last_mut().unwrap();
    *last &= 0b01111111;
    rv
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().fold(vec![], |mut acc, val| {
        acc.extend(num_to_octets(*val));
        acc
    })
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.iter().all(|octet| octet & 0b10000000 != 0) {
        return Err(Error::IncompleteNumber);
    }

    let mut chunks: Vec<Vec<u8>> = vec![];
    let mut current_chunk: Vec<u8> = vec![];
    for byte in bytes.iter() {
        let byte_starts_with_1 = byte & 0b10000000 != 0;
        if byte_starts_with_1 {
            current_chunk.push(byte - 0b10000000);
        } else {
            current_chunk.push(*byte);
            chunks.push(current_chunk);
            current_chunk = vec![];
        }
    }

    let mut rv = vec![];
    for chunk in chunks {
        let mut n = 0u32;
        for septet in chunk {
            n = match n.checked_mul(128) {
                Some(result) => result,
                None => return Err(Error::Overflow),
            };
            n += septet as u32;
        }

        rv.push(n);
    }

    Ok(rv)
}
