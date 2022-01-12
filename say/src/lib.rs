use std::collections::HashMap;

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".into();
    }

    let m: Vec<(u64, &str)> = vec![
        (1_000_000_000_000_000_000, "quintillion"),
        (1_000_000_000_000_000, "quadrillion"),
        (1_000_000_000_000, "trillion"),
        (1_000_000_000, "billion"),
        (1_000_000, "million"),
        (1000, "thousand"),
        (100, "hundred"),
        (90, "ninety"),
        (80, "eighty"),
        (70, "seventy"),
        (60, "sixty"),
        (50, "fifty"),
        (40, "forty"),
        (30, "thirty"),
        (20, "twenty"),
        (19, "nineteen"),
        (18, "eighteen"),
        (17, "seventeen"),
        (16, "sixteen"),
        (15, "fifteen"),
        (14, "fourteen"),
        (13, "thirteen"),
        (12, "twelve"),
        (11, "eleven"),
        (10, "ten"),
        (9, "nine"),
        (8, "eight"),
        (7, "seven"),
        (6, "six"),
        (5, "five"),
        (4, "four"),
        (3, "three"),
        (2, "two"),
        (1, "one"),
    ];

    let hm = m
        .iter()
        .map(|&(n, s)| (n, s))
        .collect::<HashMap<u64, &str>>();

    let mut words_vec: Vec<String> = vec![];
    for (divisor, s) in m {
        if n / divisor == 0 {
            continue;
        }

        match divisor {
            1..=19 => {
                words_vec.push(s.into());
                n -= divisor;
            }

            20..=99 => {
                let d = (n / 10) * 10;
                let ds = hm.get(&d).unwrap();

                let r = n % 10;

                if r != 0 {
                    let rs = hm.get(&r).unwrap();
                    words_vec.push(format!("{}-{}", ds, rs));
                } else {
                    words_vec.push((*ds).into());
                }

                n -= d;
                n -= r;
            }

            _ => {
                let d = n / divisor;
                let ds = encode(d);

                words_vec.push((*ds).into());
                words_vec.push(s.into());

                n -= d * divisor;
            }
        }
    }

    words_vec.join(" ")
}
