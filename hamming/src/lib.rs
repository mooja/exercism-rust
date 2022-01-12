pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let zipped = s1.chars().zip(s2.chars());
    let distance = zipped.fold(0, |acc, (a, b)| match a == b {
        true => 0,
        false => acc + 1,
    });

    Some(distance)
}
