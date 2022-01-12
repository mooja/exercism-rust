use std::cmp::{max, min};

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &'a [u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match &self.scores.len() {
            0 => None,
            n => Some(self.scores[n - 1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match &self.scores.len() {
            0 => None,
            1 => Some(self.scores[0]),
            _ => Some(self.scores.iter().fold(self.scores[0], |a, b| max(a, *b))),
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = Vec::from(self.scores);
        v.sort();
        v.reverse();
        let up_to = min(3, v.len());
        v[..up_to].to_vec()
    }
}
