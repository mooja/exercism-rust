#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors(num: u64) -> Vec<u64> {
    let mut rv = vec![1];
    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            rv.push(i);
        }
    }
    rv
}

pub fn classify(num: u64) -> Option<Classification> {
    use Classification::*;
    match num {
        0 => None,
        1 => Some(Deficient),
        _ => match factors(num).iter().sum::<u64>() {
            n if n > num => Some(Abundant),
            n if n == num => Some(Perfect),
            n if n < num => Some(Deficient),
            _ => None,
        },
    }
}
