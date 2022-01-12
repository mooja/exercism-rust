/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
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
            continue
        }

        *digit = *digit * 2;
        if *digit > 9 {
            *digit -= 9
        }
    }

    let digit_sum = digits.into_iter().sum::<u8>();
    return digit_sum % 10 == 0
}
