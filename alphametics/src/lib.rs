extern crate itertools;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Solution = HashMap<char, u8>;

#[derive(Debug, Clone)]
struct Equation {
    ns: Vec<Vec<char>>,
}

impl Equation {
    fn subequation(&self) -> Option<Equation> {
        let longest = self.ns.iter().map(|s| s.len()).max().unwrap();
        if longest == 1 {
            return None;
        }

        let new_ns: Vec<Vec<char>> = self
            .ns
            .clone()
            .into_iter()
            .map(|chars| {
                chars
                    .into_iter()
                    .rev()
                    .take(longest - 1)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<char>>()
            })
            .collect();

        Some(Equation { ns: new_ns })
    }

    fn iter_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.ns.iter().map(|chs| chs.iter()).flatten().map(|&c| c)
    }

    fn free_vars(&self) -> (HashSet<char>, HashSet<u8>) {
        let free_chars = self.iter_chars().collect::<HashSet<_>>();
        let seen_ns = self
            .iter_chars()
            .filter_map(|c| c.is_numeric().then(|| c as u8 - '0' as u8))
            .collect::<HashSet<_>>();
        let free_nums = (0..10)
            .collect::<HashSet<u8>>()
            .difference(&seen_ns)
            .into_iter()
            .map(|n| *n)
            .collect();

        (free_chars, free_nums)
    }

    fn most_leading_char(&self) -> Option<char> {
        let max_term_len = self.ns.iter().map(|term| term.len()).max().unwrap();
        let n_longest_terms = self
            .ns
            .iter()
            .filter(|term| term.len() == max_term_len)
            .count();
        match n_longest_terms {
            1 => Some(
                self.ns
                    .iter()
                    .filter(|term| term.len() == max_term_len)
                    .map(|t| t.iter().copied().next().unwrap())
                    .next()
                    .unwrap(),
            ),
            _ => None,
        }
    }

    fn solutions_outer(&self) -> Vec<Solution> {
        let prefilled: Solution = match self.most_leading_char() {
            Some(ch) => std::iter::once((ch, 1)).collect(),
            None => HashMap::new(),
        };

        self.solutions(&prefilled)
    }

    fn solutions(&self, prefilled: &Solution) -> Vec<Solution> {
        let sub_solutions = match self.subequation() {
            None => vec![HashMap::new()],
            Some(subeq) => subeq.solutions(prefilled)
        };

        let (free_chars, free_ns) = self.free_vars();
        let free_chars = &free_chars - &prefilled.keys().copied().collect();

        if free_chars.len() == 0 {
            return sub_solutions;
        }

        let free_ns = &free_ns - &prefilled.values().copied().collect();

        let mut solutions = vec![];
        for sub_solution in sub_solutions {
            let free_chars = &free_chars - &sub_solution.keys().copied().collect();
            if free_chars.len() == 0 {
                solutions.push(sub_solution);
                continue;
            }

            let free_ns = &free_ns - &sub_solution.values().copied().collect();
            for perm in free_ns.into_iter().permutations(free_chars.len()) {
                let mut solution_candidate: Solution = free_chars
                    .clone()
                    .into_iter()
                    .zip(perm.into_iter())
                    .collect();

                solution_candidate.extend(prefilled);
                solution_candidate.extend(&sub_solution);

                if self.is_solved_by(&solution_candidate, false) {
                    solutions.push(solution_candidate);
                } else {
                }
            }
        }

        solutions
    }

    fn is_solved_by(&self, sol_candidate: &Solution, final_check: bool) -> bool {
        let mut ns = self.ns.clone();
        for (ch, n) in sol_candidate {
            for term in ns.iter_mut() {
                for term_ch in term.iter_mut() {
                    if ch == term_ch {
                        *term_ch = (*n + '0' as u8) as char;
                    }
                }
            }
        }

        let as_ints = ns
            .iter()
            .map(|char_vec| char_vec.iter().collect::<String>().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut lhs_sum = (&as_ints[..as_ints.len() - 1]).iter().sum::<u64>();
        let rhs_sum = as_ints[as_ints.len() - 1];

        if final_check {
            if ns.iter().any(|ch_vec| ch_vec[0] == '0') {
                return false;
            }
        } else {
            let mut lhs_ndigits = (lhs_sum as f32).log(10.0).floor() as u64;
            let rhs_ndigits = (rhs_sum as f32).log(10.0).floor() as u64;
            while lhs_ndigits > rhs_ndigits {
                lhs_sum -= 10u64.pow(lhs_ndigits as u32);
                lhs_ndigits = (lhs_sum as f32).log(10.0).floor() as u64;
            }
        }

        lhs_sum == rhs_sum
    }
}

impl<T: AsRef<str>> From<T> for Equation {
    fn from(source: T) -> Self {
        Equation {
            ns: source
                .as_ref()
                .trim()
                .split_whitespace()
                .filter_map(|s| {
                    let is_term = s.chars().all(|ch| ch.is_alphanumeric());
                    is_term.then(|| s.chars().collect::<Vec<_>>())
                })
                .collect(),
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let eq = Equation::from(input);
    let mut solutions = eq
        .solutions_outer()
        .into_iter()
        .filter(|sol: &Solution| eq.is_solved_by(sol, true))
        .collect::<Vec<_>>();

    if solutions.len() == 1 {
        Some(solutions.pop().unwrap())
    } else {
        None
    }
}