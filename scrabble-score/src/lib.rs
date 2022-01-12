use std::collections::HashMap;

static SCORE_DATA: &str = "
    A, E, I, O, U, L, N, R, S, T       1
    D, G                               2
    B, C, M, P                         3
    F, H, V, W, Y                      4
    K                                  5
    J, X                               8
    Q, Z                               10
";

fn tokenize(line: &str) -> (Vec<char>, u64) {
    let mut chunks = vec![];
    let mut chunk = String::new();
    for ch in line.chars() {
        if ch.is_alphanumeric() {
            chunk.push(ch);
        } else if chunk.len() > 0 {
            chunks.push(chunk.clone());
            chunk = String::new();
        }
    }
    chunks.push(chunk);

    let chars = chunks
        .iter()
        .take(chunks.len() - 1)
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();

    let score = chunks[chunks.len() - 1].parse::<u64>().unwrap_or(0);
    (chars, score)
}

fn make_ch_scores() -> HashMap<char, u64> {
    let mut rv = HashMap::new();
    for line in SCORE_DATA.trim().lines() {
        let (chars, score) = tokenize(&line.to_lowercase()[..]);
        for ch in chars {
            rv.insert(ch, score);
        }
    }

    rv
}

pub fn score(word: &str) -> u64 {
    let scores = make_ch_scores();
    word.to_lowercase()
        .chars()
        .fold(0, |acc, ch| match scores.get(&ch) {
            None => acc,
            Some(score) => acc + (*score as u64),
        })
}
