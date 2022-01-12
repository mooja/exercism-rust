fn mod_pow(mut n: u64, mut e: u64, m: u64) -> u64 {
    if m == 1 { return 0 }

    let mut rv = 1;
    n = n % m;
    while e > 0 {
        if e % 2 == 1 {
            rv = rv * n % m;
        }
        e = e >> 1;
        n = n * n % m
    }

    rv
}

pub fn private_key(p: u64) -> u64 {
   3 % p 
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}
