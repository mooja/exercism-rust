pub fn number(user_number: &str) -> Option<String> {
    let mut digits = user_number
        .chars()
        .filter(|ch| ch.is_digit(10))
        .map(|ch| ch as u8 - 48)
        .collect::<Vec<u8>>();

    if digits.len() == 11 {
        if digits[0] != 1 {
            return None
        }
        digits = digits[1..].to_vec();
    }

    if digits.len() != 10 {
        return None;
    }

    if digits[0] < 2 || digits[3] < 2 {
        return None;
    }

    Some(
        digits.iter()
            .map(|i| (i + 48) as char)
            .collect()
    )
}
