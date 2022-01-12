
pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let all_spaces_or_numerics = self
            .code
            .chars()
            .all(|ch| ch.is_ascii_digit() || ch == ' ');

        if !all_spaces_or_numerics {
            return false;
        }

        let mut digits: Vec<u8> = self
            .code
            .chars()
            .filter(|&ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - 48)
            .collect();

        if digits.len() < 2 {
            return false;
        }

        for (idx, digit) in digits.iter_mut().rev().enumerate() {
            if idx % 2 == 0 {
                continue;
            }

            *digit = *digit * 2;
            if *digit > 9 {
                *digit -= 9
            }
        }

        let digit_sum: u32 = digits.into_iter().map(|x| x as u32).sum();
        return digit_sum % 10 == 0;
    }
}

use std::fmt::Display;

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            code: format!("{}", input)
        }
    }
}