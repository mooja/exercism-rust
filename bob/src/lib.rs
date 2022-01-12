fn is_yell(msg: &str) -> bool {
    let mut has_letters = false;
    for ch in msg.chars() {
        if ch.is_alphabetic() {
            has_letters = true;
        }
        if ch.is_alphabetic() && ch.is_lowercase() {
            return false;
        }
    }
    true && has_letters
}

fn is_question(msg: &str) -> bool {
    msg.trim().ends_with("?")
}

pub fn reply(msg: &str) -> &str {
    if is_yell(msg) && is_question(msg) {
        "Calm down, I know what I'm doing!"
    } else if is_yell(msg) {
        "Whoa, chill out!"
    } else if is_question(msg) {
        "Sure."
    } 
    else if msg.trim().len() == 0 {
        "Fine. Be that way!"
    }
    else {
        "Whatever."
    }
}