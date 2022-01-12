extern crate num_bigint;

use std::iter;

use num_bigint::BigInt;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Decimal {
    n: BigInt,
    d_exp: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut input_iter = input.split('.');
        // Prase the left side.
        let left_str = input_iter.next()?;
        let mut n: BigInt = left_str.parse::<BigInt>().ok()?;

        // Parse the right side. Trim trailing zeroes.
        let right_str = match input_iter.next() {
            None => "",
            Some(s) => s.trim_end_matches('0'),
        };

        // Denominator exponent = number of digits after decimal point
        let d_exp: u32 = right_str.len() as u32;

        // If the exponent is zero, return the result.
        // Otherwise, multiply n by the inverse exponent and add the right side.
        match d_exp {
            0 => Some(Decimal { n: n, d_exp: 0 }),

            _ => {
                n *= (BigInt::from(10)).pow(d_exp);

                let mut right_n: BigInt = match right_str.parse::<BigInt>() {
                    Err(_) => BigInt::from(0),
                    Ok(n) => n,
                };

                if input.starts_with("-") {
                    right_n *= BigInt::from(-1);
                }

                n += right_n;

                Some(Decimal { n: n, d_exp: d_exp })
            }
        }
    }

    pub fn cleanup_precision(&mut self) {
        let n_as_str: String = self.as_str();
        *self = Decimal::try_from(&n_as_str).unwrap();
    }

    fn is_negative(&self) -> bool {
        self.n < BigInt::from(0)
    }

    fn as_str(&self) -> String {
        let n_str: String = format!("{}", self.n);
        let just_n: String = n_str.chars().filter(|ch| ch.is_numeric()).collect();
        let n_is_neg = n_str.starts_with("-");
        let n_len = just_n.len();
        let decimal_idx = n_len as i32 - self.d_exp as i32;
        let n_leading_zeroes = if decimal_idx < 0 {
            decimal_idx * (-1)
        } else {
            0
        };

        let mut rv = String::new();
        if n_is_neg {
            rv += "-";
        }

        rv += "0";

        match n_leading_zeroes > 0 {
            true => {
                rv += ".";
                rv += iter::repeat('0')
                    .take(n_leading_zeroes as usize)
                    .collect::<String>()
                    .as_ref();
                rv += &just_n;
                rv
            }

            false => {
                rv += &just_n[0..decimal_idx as usize];
                rv += ".";
                rv += &just_n[(decimal_idx as usize)..];
                rv
            }
        }
    }

    fn norm_to_exponent(&self, target_exp: u32) -> BigInt {
        let mut n = self.n.clone();

        if self.d_exp == target_exp {
            return n;
        }

        if self.d_exp > target_exp {
            panic!("Should not be raising to a smaller exponent when comparing.");
        }

        let exp_diff = target_exp - self.d_exp;
        n *= BigInt::from(10).pow(exp_diff);

        n
    }

    fn negate(mut self) -> Self {
        self.n *= BigInt::from(-1);
        self
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, rhs: &Decimal) -> Option<Ordering> {
        let largest_denom_exponent = self.d_exp.max(rhs.d_exp);
        let rhs_n = rhs.norm_to_exponent(largest_denom_exponent);
        let self_n = self.norm_to_exponent(largest_denom_exponent);

        self_n.partial_cmp(&rhs_n)
    }
}

impl std::ops::Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Decimal) -> Self {
        let largest_denom_exponent = self.d_exp.max(rhs.d_exp);
        let self_n = self.norm_to_exponent(largest_denom_exponent);
        let rhs_n = rhs.norm_to_exponent(largest_denom_exponent);

        let mut rv = Decimal {
            n: self_n + rhs_n,
            d_exp: largest_denom_exponent,
        };

        rv.cleanup_precision();
        rv
    }
}

impl std::ops::Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Decimal) -> Self {
        let rhs = rhs.negate();
        self + rhs
    }
}

impl std::ops::Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Decimal) -> Self {
        let mut self_ = self;
        self_.n *= rhs.n;
        self_.d_exp += rhs.d_exp;
        self_.cleanup_precision();
        self_
    }
}
