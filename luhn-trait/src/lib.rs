pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl <T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string();

        let all_spaces_or_numerics = code
            .chars()
            .all(|ch| ch.is_ascii_digit() || ch == ' ');
        if !all_spaces_or_numerics {
            return false;
        }

        let mut digits: Vec<u8> = code
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

        let digit_sum: u32 = digits.iter().map(|&x| x as u32).sum();
        return digit_sum % 10 == 0;
    }
}