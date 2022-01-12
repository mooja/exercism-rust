use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut rv = BTreeMap::new();
    for (&k, values) in h {
        for &v in values {
            rv.insert(v.to_ascii_lowercase(), k);
        }
    }
    rv
}
