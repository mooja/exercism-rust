pub fn factors(n: u64) -> Vec<u64> {
    let mut rv = Vec::new();
    let mut n = n;
    let mut x = 2;

    while n != 1 {
        while n % x == 0 {
            rv.push(x);
            n = n / x;
        }
        x = x + 1;
    }
    rv
}
