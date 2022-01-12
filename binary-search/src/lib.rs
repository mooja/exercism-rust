pub fn find(array: &[i32], key: i32) -> Option<usize> {
    match array.binary_search(&key) {
        Err(_) => None,
        Ok(k) => Some(k)
    }
}