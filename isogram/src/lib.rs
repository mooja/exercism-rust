use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    for ch in candidate.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch == ' ' || ch == '-' {
            continue;
        } else if hs.contains(&ch) {
            return false;
        } else {
            hs.insert(ch);
        }
    }
    true
}
