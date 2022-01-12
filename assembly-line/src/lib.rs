// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as u32;

    match speed {
        0 => 0.0,
        n @ 1..=4 => (n * 221) as f64,
        n @ 5..=8 => (n * 221) as f64 * 0.9,
        n @ 9..=10 => (n * 221) as f64 * 0.77,
        _ => unimplemented!(),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}
