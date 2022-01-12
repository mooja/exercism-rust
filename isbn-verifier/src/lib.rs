fn _atoi(c: char) -> u32 {
    if c.is_digit(10) {
        return c.to_digit(10).unwrap();
    } else if c == 'x' || c == 'X' {
        return 10
    } else {
        panic!("Couldn't parse {}", c)
    }
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // filter out dashes
    let isbn: String = 
        isbn
        .chars()
        .filter(|&ch| ch != '-')
        .collect();

    // check number of digits
    if isbn.len() != 10 {
        return false;
    }

    // first nine digits must be between 0-9
    if !isbn.chars().take(9).all(|ch| ch.is_digit(10)) {
        return false
    }

    // last digit must be a digit or X
    let last_ch = isbn.chars().last().unwrap();
    if !last_ch.is_digit(10) && last_ch != 'X' {
        return false
    }

    // parse digits
    let digits: Vec<u32> = 
        isbn
        .chars()
        .map(|c| _atoi(c))
        .collect();

    // verify checksum
    let checksum_correct = (
        digits[0] * 10 
        + digits[1] * 9
        + digits[2] * 8
        + digits[3] * 7
        + digits[4] * 6
        + digits[5] * 5
        + digits[6] * 4
        + digits[7] * 3 
        + digits[8] * 2
        + digits[9] * 1)
        % 11 == 0;

    checksum_correct
}