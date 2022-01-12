extern crate regex;
extern crate lazy_static;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn normalized(w: &str) -> String {
    lazy_static! {
        static ref left_re: Regex = Regex::new(r"^\W").unwrap();
        static ref right_re: Regex = Regex::new(r"\W$").unwrap();
    }

    let w = w.to_ascii_lowercase();
    let w = left_re.replace(w.as_str(), "").to_string();
    let w = right_re.replace(w.as_str(), "").to_string();
    w
}

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut rv: HashMap<String, u32> = HashMap::new();
    for w in Regex::new(r"[\w']+").unwrap().find_iter(words) {
        let normalized = normalized(w.as_str());
        *rv.entry(normalized).or_insert(0) += 1;
    }
    
    rv
}
