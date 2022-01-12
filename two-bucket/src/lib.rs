use std::cmp::PartialEq;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BBucket {
    pub capacity: u8,
    pub level: u8,
}

impl BBucket {
    pub fn new(capacity: u8) -> Self {
        BBucket {
            level: 0,
            capacity: capacity,
        }
    }

    fn unfilled_volume(&self) -> u8 {
        self.capacity - self.level
    }

    pub fn give_water_to(&self, other: &BBucket) -> Self {
        let xfer_volume = self.level.min(other.unfilled_volume());
        Self {
            level: self.level - xfer_volume,
            ..(*self)
        }
    }

    pub fn recieve_from(&self, other: &BBucket) -> Self {
        let xfer_volume = self.unfilled_volume().min(other.level);
        Self {
            level: self.level + xfer_volume,
            ..(*self)
        }
    }

    pub fn fill(&self) -> Self {
        Self {
            level: self.capacity,
            ..(*self)
        }
    }

    pub fn empty(&self) -> Self {
        Self {
            level: 0,
            ..(*self)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BBucketState {
    b1: BBucket,
    b2: BBucket,
    nmoves: u8,
}

impl BBucketState {
    pub fn new(b1: u8, b2: u8) -> Self {
        BBucketState {
            b1: BBucket::new(b1),
            b2: BBucket::new(b2),
            nmoves: 0,
        }
    }

    pub fn fill_1(&self) -> Self {
        BBucketState {
            b1: self.b1.fill(),
            nmoves: self.nmoves + 1,
            ..(*self)
        }
    }

    pub fn fill_2(&self) -> Self {
        BBucketState {
            b2: self.b2.fill(),
            nmoves: self.nmoves + 1,
            ..(*self)
        }
    }

    pub fn empty_1(&self) -> Self {
        BBucketState {
            b1: self.b1.empty(),
            nmoves: self.nmoves + 1,
            ..(*self)
        }
    }

    pub fn empty_2(&self) -> Self {
        BBucketState {
            b2: self.b2.empty(),
            nmoves: self.nmoves + 1,
            ..(*self)
        }
    }

    fn pour_b1_to_b2(self) -> Self {
        let b1 = self.b1.give_water_to(&self.b2);
        let b2 = self.b2.recieve_from(&self.b1);

        BBucketState {
            b1: b1,
            b2: b2,
            nmoves: self.nmoves + 1,
        }
    }

    fn pour_b2_to_b1(self) -> Self {
        let b1 = self.b1.recieve_from(&self.b2);
        let b2 = self.b2.give_water_to(&self.b1);

        BBucketState {
            b1: b1,
            b2: b2,
            nmoves: self.nmoves + 1,
        }
    }

    pub fn successors(&self) -> Vec<Self> {
        let mut rv = vec![];

        rv.push(self.fill_1());
        rv.push(self.fill_2());
        rv.push(self.empty_1());
        rv.push(self.empty_2());
        rv.push(self.pour_b1_to_b2());
        rv.push(self.pour_b2_to_b1());

        rv
    }

    pub fn solves(&self, level: u8) -> bool {
        self.b1.level == level || self.b2.level == level
    }
}

impl PartialEq for BBucketState {
    fn eq(&self, other: &BBucketState) -> bool {
        self.b1 == other.b1 && self.b2 == other.b2
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let initial = match start_bucket {
        Bucket::One => BBucketState::new(capacity_1, capacity_2).fill_1(),
        Bucket::Two => BBucketState::new(capacity_1, capacity_2).fill_2(),
    };

    let opposite = match start_bucket {
        Bucket::One => BBucketState::new(capacity_1, capacity_2).fill_2(),
        Bucket::Two => BBucketState::new(capacity_1, capacity_2).fill_1(),
    };

    let mut queue = vec![initial];
    let mut seen: Vec<BBucketState> = vec![opposite];
    let mut solution: Option<BBucketState> = None;

    while queue.len() > 0 {
        let state = queue.pop().unwrap();
        seen.push(state);

        if state.solves(goal) {
            solution = Some(state);
            break;
        } else {
            for s in state.successors().iter() {
                if !(seen.contains(s)) {
                    queue.insert(0, *s);
                }
            }
        }
    }

    match solution {
        None => None,
        Some(solution) => {
            let goal_bucket = match solution.b1.level {
                x if x == goal => Bucket::One,
                _ => Bucket::Two,
            };

            let other = match goal_bucket {
                Bucket::One => solution.b2.level,
                Bucket::Two => solution.b1.level,
            };

            Some(BucketStats {
                goal_bucket: goal_bucket,
                moves: solution.nmoves,
                other_bucket: other,
            })
        }
    }
}
