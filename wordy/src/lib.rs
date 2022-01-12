pub struct WordProblem;

enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

enum Token {
    Number(i32),
    Op(Operation),
}

enum ParsingStates {
    ExpectingNumber,
    ExpectingOpOrEnd,
}

fn is_number(s: &str) -> bool {
    return s.parse::<i32>().is_ok();
}

fn parse_token(s: &str) -> Option<Operation> {
    match s {
        "plus" => Some(Operation::Addition),
        "minus" => Some(Operation::Subtraction),
        "multiplied" => Some(Operation::Multiplication),
        "divided" => Some(Operation::Division),
        _ => None,
    }
}

fn parse_tokens(s: &str) -> Result<Vec<Token>, String> {
    if s.len() == 0 {
        return Err("empty input".to_string());
    }

    if s.chars().last().unwrap() != '?' {
        return Err("input must end with ?".to_string());
    }

    let words: Vec<&str> = s[..s.len() - 1]
        .split(" ")
        .filter(|&word| word != "by")
        .collect();

    if words.len() < 3 {
        return Err("input must have at least 3 words.".to_string());
    }

    if words[0] != "What" && words[1] != "is" {
        return Err("input must start with 'What is".to_string());
    }

    use ParsingStates::*;
    let mut tokens = vec![];
    let mut state = ExpectingNumber;
    let mut words_iter = words.iter().skip(2);
    loop {
        match (state, words_iter.next()) {
            (ExpectingNumber, None) => return Err("Expected a number, got EOS".to_string()),

            (ExpectingNumber, Some(s)) if !is_number(&s) => {
                return Err(format!("Expected a number, got {}", &s).to_string())
            }

            (ExpectingNumber, Some(s)) => {
                tokens.push(Token::Number(s.parse::<i32>().unwrap()));
                state = ExpectingOpOrEnd;
            }

            (ExpectingOpOrEnd, None) => {
                return Ok(tokens);
            }

            (ExpectingOpOrEnd, Some(s)) if parse_token(s).is_none() => {
                return Err(format!("Expected a operation, got {}", &s).to_string());
            }

            (ExpectingOpOrEnd, Some(s)) => {
                tokens.push(Token::Op(parse_token(&s).unwrap()));
                state = ExpectingNumber;
            }
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let maybe_tokens = parse_tokens(command);
    if maybe_tokens.is_err() {
        return None;
    }

    let tokens = maybe_tokens.unwrap();
    let mut acc = 0;
    let mut prev_op = Operation::Addition;

    for token in tokens {
        match token {
            Token::Number(n) => match prev_op {
                Operation::Addition => acc += n,
                Operation::Subtraction => acc -= n,
                Operation::Multiplication => acc *= n,
                Operation::Division => acc /= n,
            },

            Token::Op(operation) => {
                prev_op = operation;
            }
        }
    }

    Some(acc)
}
