use std::collections::HashMap;
use std::collections::VecDeque;

type Domino = (u8, u8);
type Chain = Vec<Domino>;

#[derive(Default, Clone)]
struct DominoCounts {
    ds: HashMap<Domino, u32>,
}

fn normalize(domino: Domino) -> Domino {
    if domino.0 > domino.1 {
        (domino.1, domino.0)
    } else {
        domino
    }
}

fn flip(domino: Domino) -> Domino {
    (domino.1, domino.0)
}

impl DominoCounts {
    fn add_domino(&mut self, domino: Domino) {
        let domino = normalize(domino);
        let e = self.ds.entry(domino).or_insert(0);
        (*e) += 1;
    }

    fn remove_domino(&mut self, domino: Domino) {
        let domino = normalize(domino);
        let d = self.ds.get_mut(&domino).unwrap();
        *d -= 1;
    }

    fn with_subtracted_chain(&self, chain: &Chain) -> Self {
        let mut rv = self.clone();
        for &domino in chain {
            rv.remove_domino(domino);
        }

        rv
    }

    fn uniq_ds<'a>(&'a self) -> impl Iterator<Item = Domino> + 'a {
        self.ds
            .iter()
            .filter(|&(_, count)| *count > 0)
            .map(|(d, _)| *d)
    }
}

impl From<&[(u8, u8)]> for DominoCounts {
    fn from(input: &[(u8, u8)]) -> Self {
        let mut counts = DominoCounts::default();
        for &domino in input {
            counts.add_domino(domino);
        }

        counts
    }
}

fn initial_successors(counts: &DominoCounts) -> Vec<Chain> {
    counts
        .ds
        .iter()
        .fold(vec![], |mut acc, (&d, _counts)| match d {
            (m, n) if m == n => {
                acc.push(vec![d]);
                acc
            }

            _ => {
                acc.push(vec![d]);
                acc.push(vec![flip(d)]);
                acc
            }
        })
}

fn successors(chain: Vec<Domino>, original_counts: &DominoCounts) -> Vec<Chain> {
    let mut rv = vec![];
    let right_end = chain.last().unwrap().1;
    let adjusted_counts = original_counts.with_subtracted_chain(&chain);
    let candidates_iter = adjusted_counts
        .uniq_ds()
        .filter(|d| d.0 == right_end || d.1 == right_end);

    for d in candidates_iter {
        let mut new_chain = chain.clone();

        if right_end == d.0 {
            new_chain.push(d);
        } else {
            new_chain.push(flip(d));
        }

        rv.push(new_chain);
    }

    rv
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.len() == 0 {
        return Some(vec![]);
    }

    let counts = DominoCounts::from(input);
    let mut queue: VecDeque<Chain> = initial_successors(&counts).into();

    while queue.len() > 0 {
        let chain_candidate = queue.pop_back().unwrap();
        if chain_candidate.len() == input.len() {
            let first = chain_candidate[0];
            let last = chain_candidate[chain_candidate.len() - 1];
            if first.0 == last.1 {
                return Some(chain_candidate);
            }
        }

        for candidate in successors(chain_candidate, &counts).into_iter() {
            queue.push_back(candidate);
        }
    }

    None
}
