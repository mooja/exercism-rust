// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let initial: HashMap<&str, u32> = HashMap::new();

    let mut counts_mag = magazine.iter().fold(initial.clone(), |mut acu, &w| {
        *acu.entry(w).or_default() += 1;
        acu
    });

    let counts_note = note.iter().fold(initial, |mut acu, &w| {
        *acu.entry(w).or_default() += 1;
        acu
    });

    for (w, count) in counts_note {
        let mag_e = counts_mag.entry(w).or_default();
        if count > *mag_e {
            return false;
        }
    }

    true
}
