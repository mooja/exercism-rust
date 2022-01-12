#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    a: u64,
    b: u64,
    val: u64
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            a: a.min(b),
            b: a.max(b),
            val: a*b
        }
    }

    pub fn value(&self) -> u64 {
        self.a * self.b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.a = a.min(b);
        self.b = a.max(b);
        self.val = a * b;
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let digits = s.chars();
    let digits_reversed = s.chars().rev();

    digits.zip(digits_reversed).all(|(a, b)| a == b)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let (sm, lg) = {
        let mut smallest_so_far:  Option<Palindrome> = None;
        let mut largest_so_far: Option<Palindrome> = None;

        for f1 in min..=max {
            for f2 in f1..=max {
                let candidate = f1*f2;
                if is_palindrome(candidate) {
                    if smallest_so_far.is_none() || candidate < smallest_so_far.as_ref().unwrap().value()  {
                        smallest_so_far = Some(Palindrome::new(f1, f2));
                    }

                    if largest_so_far.is_none() || candidate >= largest_so_far.as_ref().unwrap().value()  {
                        largest_so_far = Some(Palindrome::new(f1, f2));
                    }
                }
            }
        }

        (smallest_so_far, largest_so_far)
    };

    match (sm, lg) {
        (Some(p1), Some(p2)) => Some((p1, p2)),
        _ => None,
    }
}