pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    for i in 0..list.len() {
        if i + 1 == list.len() {
            break;
        };
        let addition = format!(
            "For want of a {0} the {1} was lost.\n",
            &list[i],
            &list[i + 1]
        );
        result.push_str(&addition);
    }

    if list.len() > 0 {
        result.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    result
}
