use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let alphabet: HashSet<char> = ('a'..='z').collect();
    sentence
        .to_ascii_lowercase()
        .chars()
        .collect::<HashSet<char>>()
        .is_superset(&alphabet)
}
