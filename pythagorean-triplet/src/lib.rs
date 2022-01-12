use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut rv = HashSet::new();
    for b in 1..=(sum / 2) {
        for a in 1..=b {
            let c = sum - a - b;
            if a*a + b*b == c*c {
                rv.insert([a, b, c]);
            }
        }
    }

    rv.into_iter().collect()
}
