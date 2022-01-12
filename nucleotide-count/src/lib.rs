use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_chars = "TACG";
    if !valid_chars.contains(nucleotide) {
        return Err(nucleotide); 
    }

    let mut count = 0;
    for c in dna.chars() {
        if !valid_chars.contains(c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut rv = HashMap::new();
    for c in "TACG".chars() {
        rv.insert(c, count(c, dna) ?);
    }
    Ok(rv)
}
