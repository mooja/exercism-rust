pub fn is_armstrong_number(num: u32) -> bool {
    let ndigits = num.to_string().len() as u32;
    num == num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(ndigits))
        .sum()
}
