pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut rv = vec![];
    let mut idx = 2;
    let mut sieve: Vec<bool> = vec![true; (upper_bound+1) as usize];

    while idx <= upper_bound as usize {
        if sieve[idx] {
            rv.push(idx as u64);

            let mut idx_2 = idx;
            while idx_2 <= upper_bound as usize {
                sieve[idx_2] = false;
                idx_2 += idx;
            }
        }

        idx += 1;
    }
    rv
}
