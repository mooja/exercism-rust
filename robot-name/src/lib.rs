extern crate rand;

use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    pub static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

#[derive(Default)]
pub struct Robot(String);

fn rand_name() -> String {
    let mut rng = thread_rng();
    let c1 = (rng.gen_range(65..=90) as u8) as char;
    let c2 = (rng.gen_range(65..=90) as u8) as char;
    let digits = rng.gen_range(0..1000);
    format!("{}{}{:03}", c1, c2, digits)
}

fn unused_name() -> String {
    USED_NAMES.with(|rc| loop {
        let name_candidate = rand_name();
        if !rc.borrow().contains(&name_candidate) {
            rc.borrow_mut().insert(name_candidate.clone());
            return name_candidate;
        }
    })
}

impl Robot {
    pub fn new() -> Self {
        Robot(unused_name())
    }

    pub fn name(&self) -> &str {
        self.0.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.0 = unused_name();
    }
}
