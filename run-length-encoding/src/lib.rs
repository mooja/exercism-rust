pub fn encode(source: &str) -> String {
    let mut v: Vec<(char, u32)> = vec![];
    for ch in source.chars() {
        match v.pop() {
            None => {
                v.push((ch, 1))
            }

            Some((last_char, last_count)) => {
                if ch == last_char  {
                    v.push((last_char, last_count + 1));
                } else {
                    v.push((last_char, last_count));
                    v.push((ch, 1));
                }
            }
        }
    }

    let mut rv = String::new();
    for (ch, count) in v {
        if count > 1 {
            rv.push_str(&count.to_string());
        }
        rv.push(ch);
    }

    rv
}

#[derive(Debug)]
enum Token {
    Character(char),
    Number(u32)
}

pub fn decode(source: &str) -> String {
    use Token::*;
    let mut tokens: Vec<Token> = vec![];
    for ch in source.chars() {
        if ch.is_alphabetic() || ch == ' ' {
            tokens.push(Character(ch));
        } else {
            let digit: u32 = ch.to_digit(10).unwrap();
            match tokens.last() {
                None => tokens.push(Number(digit)),

                Some(Character(_)) => {
                    tokens.push(Number(digit));
                }

                Some(Number(n)) => {
                    let new_num = n*10 + digit;
                    tokens.pop();
                    tokens.push(Number(new_num));
                }
            }
        }
    }

    let mut rv = String::new();
    let mut iter = tokens.iter().peekable();
    loop {
        let token = iter.next();
        match token {
            None => break,
            Some(Character(c)) => rv.push(*c),
            Some(Number(n)) => {
                let next_token_char = match iter.peek().unwrap() {
                    Character(c) => c,
                    Number(_) => panic!("oh no")
                };

                for _ in 1..*n {
                    rv.push(*next_token_char);
                }
            }
        }
    }

    rv
}
