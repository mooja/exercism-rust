struct Match<'a> {
    m: &'a str,
    rest: &'a str,
}

fn rule_1_match(word: &str) -> Option<Match> {
    let first_char = &word[..1];
    if "aeiou".contains(first_char) {
        let m = Match {
            m: first_char,
            rest: &word[1..],
        };

        return Some(m);
    }

    let first_two_chars = &word[..2];
    if first_two_chars == "xr" || first_two_chars == "yt" {
        let m = Match {
            m: first_two_chars,
            rest: &word[2..],
        };

        return Some(m);
    }

    None
}

fn rule_2_match(word: &str) -> Option<Match> {
    let mut consonants_streak_len = 0;
    for ch in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(ch) {
            consonants_streak_len += 1;
        } else {
            break;
        }
    }

    if consonants_streak_len == 0 {
        return None;
    }

    // rule 4
    let streak = &word[0..consonants_streak_len];
    let streak_contains_y = streak.contains('y');
    let streaks_starts_with_y = &streak[0..1] == "y";
    if streak_contains_y && !streaks_starts_with_y {
        let offset = streak.find('y').unwrap();
        consonants_streak_len -= streak.len() - offset;
    }

    let m = Match {
        m: &word[..consonants_streak_len],
        rest: &word[consonants_streak_len..],
    };

    Some(m)
}

fn rule_3_match(word: &str) -> Option<Match> {
    let r2_m = rule_2_match(word);
    if r2_m.is_none() {
        return None;
    }

    let r2_m = r2_m.unwrap();
    if !r2_m.m.ends_with('q') {
        return None;
    }

    let r3_m = String::from(r2_m.m) + "u";
    if !word.starts_with(&r3_m) {
        return None;
    }

    Some(Match {
        m: &word[..r3_m.len()],
        rest: &word[r3_m.len()..],
    })
}

fn translate_word(input: &str) -> String {
    let r1_m = rule_1_match(input);
    if r1_m.is_some() {
        let mut rv = String::from(input);
        rv += "ay";
        return rv;
    }

    let r3_m = rule_3_match(input);
    if r3_m.is_some() {
        let m = r3_m.unwrap();
        let mut rv = String::from(m.rest);
        rv += m.m;
        rv += "ay";
        return rv;
    }

    let r2_m = rule_2_match(input);
    if r2_m.is_some() {
        let m = r2_m.unwrap();
        let mut rv = String::from(m.rest);
        rv += m.m;
        rv += "ay";
        return rv;
    }

    panic!("no rules matched \"{}\"", input);
}

pub fn translate(input: &str) -> String {
    input
        .split(" ")
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}
