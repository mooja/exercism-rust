use std::collections::HashSet;

fn as_sorted(s: &str) -> String {
    let mut chars = s.to_lowercase().chars().collect::<Vec<char>>();
    chars.sort();
    chars.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut rv: HashSet<&'a str> = HashSet::new();
    let w_sorted = as_sorted(word);
    for cand in possible_anagrams {
        if cand.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let c_sorted = as_sorted(cand);
        if w_sorted == c_sorted {
            rv.insert(cand);
        }
    }
    rv
}
