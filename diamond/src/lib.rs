pub fn get_diamond(c: char) -> Vec<String> {
    let mut rv = vec![];
    let side_len = 2*((c as usize) - 65 + 1) - 1;
    let a_to_c = 'A'..=c;
    let c_to_a = ('A'..c).rev();
    for ch in a_to_c.chain(c_to_a) {
        let mut line = vec![' '; side_len];
        let middle_idx = side_len / 2;
        let middle_offset = ch as usize - 'A' as usize;
        line[middle_idx + middle_offset] = ch;
        line[middle_idx - middle_offset] = ch;
        rv.push(line.iter().collect::<String>());
    }

    rv
}
