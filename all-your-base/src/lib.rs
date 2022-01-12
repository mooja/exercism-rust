#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    } else if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut acc = 0;
    for digit in number {
        if *digit >= from_base {
            return Err(Error::InvalidDigit(*digit));
        }
        acc *= from_base;
        acc += digit;
    }

    let mut rv: Vec<u32> = vec![];
    loop {
        let n = acc % to_base;
        rv.push(n);
        acc /= to_base;
        if acc == 0 {
            break;
        }
    }
    rv.reverse();

    Ok(rv)
}