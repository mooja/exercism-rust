// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x, y, d
        }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Robot {
            x: self.x, y: self.y, d: new_direction
        }
    }

    pub fn turn_left(self) -> Self {
        self.turn_right().turn_right().turn_right()
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.d {
            Direction::North => (self.x, self.y+1),
            Direction::East => (self.x+1, self.y),
            Direction::South => (self.x, self.y-1),
            Direction::West => (self.x-1, self.y),
        };

        Robot {
            x, y, d: self.d
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut r = Robot::new(self.x, self.y, self.d);
        for ch in instructions.chars() {
            match ch {
                'R' => r = r.turn_right(),
                'L' => r = r.turn_left(),
                'A' => r = r.advance(),
                _ => ()
            }
        }
        r
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
