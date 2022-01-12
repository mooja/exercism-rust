pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut acc = 0;
    for n in 0..limit {
        for f in factors {
            if *f == 0 {
                continue;
            }
            if n % f == 0 {
                acc += n;
                break;
            }
        }
    }
    acc
}