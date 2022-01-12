pub fn brackets_are_balanced(string: &str) -> bool {
    let pairs = [('{', '}'), ('(', ')'), ('[', ']')];
    let mut stack: Vec<char> = Vec::new();
    for symbol in string.chars() {
        for pair in &pairs {
            let (opening, closing) = pair;
            if &symbol == opening {
                stack.push(symbol);
            } else if &symbol == closing {
                if stack.len() == 0 {
                    return false;
                }
                if stack.last().unwrap() != opening {
                    return false;
                }
                stack.pop();
            }
        }
    }
    stack.len() == 0
}
