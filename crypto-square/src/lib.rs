pub fn encrypt(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string();
    }

    let normalized = input
        .chars()
        .filter_map(|ch| ch.is_alphanumeric().then(|| ch.to_ascii_lowercase()))
        .collect::<Vec<char>>();

    let mut c = (normalized.len() as f32).sqrt().floor() as usize;
    let r = c;
    if c * c < normalized.len() {
        c += 1;
    }

    let by_cols = (0..c)
        .flat_map(|i| (0..r).map(move |j| (i, j)))
        .map(|(i, j)| normalized.get(c * j + i).or(Some(&' ')).unwrap());

    by_cols
        .collect::<Vec<_>>()
        .chunks(r as usize)
        .map(|chrs| chrs.into_iter().copied().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
