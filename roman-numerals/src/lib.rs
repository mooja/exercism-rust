use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

static MAPPING: [(u32, char); 9] = [
    (10000, '?'),
    (5000, '?'),
    (1000, 'M'),
    (500, 'D'),
    (100, 'C'),
    (50, 'L'),
    (10, 'X'),
    (5, 'V'),
    (1, 'I'),
];

fn power_chars(power: u32) -> (char, char, char) {
    for (i, kv) in MAPPING.into_iter().enumerate() {
        if kv.0 == power {
            return (
                MAPPING.iter().nth(i as usize).unwrap().1,
                MAPPING.iter().nth(i as usize - 1).unwrap().1,
                MAPPING.iter().nth(i as usize - 2).unwrap().1,
            );
        }
    }

    panic!("cannot find next two powers for {}", power)
}

fn fmt_roman_power(d: u32, power: u32) -> String {
    let (p_ch, next_p_ch, next_next_p_ch) = power_chars(power);

    match d {
        1..=3 => (1..=d).map(|_| p_ch).collect(),
        4 => format!("{}{}", p_ch, next_p_ch),
        5 => format!("{}", next_p_ch),
        6 => format!("{}{}", next_p_ch, p_ch),
        7 => format!("{}{}{}", next_p_ch, p_ch, p_ch),
        8 => format!("{}{}{}{}", next_p_ch, p_ch, p_ch, p_ch),
        9 => format!("{}{}", p_ch, next_next_p_ch),
        _ => panic!("integer outside 1-9"),
    }
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let mut n = self.0;
        let mut rv = String::new();
        while n > 0 {
            let as_str = format!("{}", n);
            let first_digit: u8 = as_str.chars().next().unwrap() as u8 - ('0' as u8);
            let power: u32 = 10u32.pow(as_str.len() as u32 - 1);
            rv += &fmt_roman_power(first_digit as u32, power);
            n = n % power;
        }
        write!(_f, "{}", rv)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}
