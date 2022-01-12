pub fn nth(n: u32) -> u32 {
    let mut known_primes: Vec<u32> = Vec::new();
    let mut i = 2;

    while known_primes.len() < (n + 1) as usize {
        let mut skip = false;
        for kp in known_primes.iter() {
            if i % kp == 0 {
                skip = true;
                break;
            }
        }
        if skip {
            i += 1;
            continue;
        }

        let upper_limit = (i as f32).sqrt().ceil() as u32;
        for ii in 2..upper_limit {
            if i % ii == 0 {
                skip = true;
                break;
            }
        }
        if skip {
            i += 1;
            continue;
        }

        known_primes.push(i);
        i += 1;
    }

    i - 1
}