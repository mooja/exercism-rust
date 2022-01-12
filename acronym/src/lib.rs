pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-'][..])
        .map(capitals)
        .fold(String::from(""), |a, b| a + &b)
}

fn capitals(word: &str) -> String {
    let word: String = String::from(word)
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    match word.len() {
        0 => String::from(""),
        1..=2 => String::from(word.chars().nth(0).unwrap().to_ascii_uppercase()),
        _ => {
            let mut rv = String::new();
            let mut idx1 = 0;
            let mut idx2 = 1;
            rv.push(word.chars().nth(0).unwrap().to_ascii_uppercase());
            while idx2 < word.len() {
                if word.chars().nth(idx1).unwrap().is_ascii_lowercase()
                    && word.chars().nth(idx2).unwrap().is_ascii_uppercase()
                {
                    rv.push(word.chars().nth(idx2).unwrap().to_ascii_uppercase());
                }
                idx1 += 1;
                idx2 += 1;
            }
            rv
        }
    }
}
