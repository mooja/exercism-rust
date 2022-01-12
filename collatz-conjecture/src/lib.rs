pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }

    let mut nsteps = 0;
    let mut n = n;

    while n != 1 {
        match n % 2 == 0 {
            true => {
                n = n / 2;

            }

            false => {
                n = 3 * n + 1;
            }
        }

        nsteps += 1;
    }

    Some(nsteps)
}
