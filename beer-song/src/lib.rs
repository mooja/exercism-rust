pub fn verse(n: u32) -> String {
    match n {
        0 => String::from(format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")),
        1 => String::from(format!("{0} bottle of beer on the wall, {0} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n)),
        2 => String::from(format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", n)),
        _ => String::from(format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1))
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    let mut current_step = start as i32;
    while current_step >= end as i32 {
        result.push_str(&verse(current_step as u32));
        result.push_str(&"\n".to_string());
        current_step -= 1;
    }
    if result.ends_with("\n\n") {
        result.pop();
    }
    result
}