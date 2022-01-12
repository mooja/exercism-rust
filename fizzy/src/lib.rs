use std::fmt::Display;

pub struct Matcher<T: Display + Copy> {
    f: Box<dyn Fn(T) -> bool>,
    sub: String,
}

impl<T: Display + Copy> Matcher<T> {
    pub fn new<F, S>(_matcher: F, _subs: S) -> Matcher<T>
    where
        F: 'static + Fn(T) -> bool,
        S: Display,
    {
        Matcher {
            f: Box::new(_matcher),
            sub: format!("{}", _subs),
        }
    }
}

#[derive(Default)]
pub struct Fizzy<T: Display + Copy>(Vec<Matcher<T>>);

impl<T: 'static + Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { 0: vec![] }
    }

    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.0.push(_matcher);
        self
    }

    pub fn apply(self, _iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        _iter.map(move |item| {
            let mut s = String::new();

            for matcher in self.0.iter() {
                if (matcher.f)(item) {
                    s += &matcher.sub;
                }
            }

            if s == "" {
                s += &format!("{}", item);
            }

            s
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + PartialEq + Copy + From<u8> + std::ops::Rem<Output = T>,
{
    let fizz_matcher = Matcher::new(|item: T| item % T::from(3) == T::from(0), "fizz");
    let buzz_matcher = Matcher::new(|item: T| item % T::from(5) == T::from(0), "buzz");

    Fizzy {
        0: vec![fizz_matcher, buzz_matcher],
    }
}
