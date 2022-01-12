#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    for ch in string_digits.chars() {
        if !ch.is_digit(10) {
            return Err(Error::InvalidDigit(ch));
        }
    }

    let digits: Vec<u8> = string_digits
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();

    let mut max = 0u64;
    digits.as_slice().windows(span).for_each(|win| {
        let mut acc = 1u64;
        for digit in win.to_vec() {
            acc *= digit as u64;
        }

        if acc > max {
            max = acc
        }
    });
    Ok(max)
}
