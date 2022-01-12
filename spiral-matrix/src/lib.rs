
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size: usize = size as usize;
    let mut rv: Vec<Vec<u32>> = vec![vec![0; size]; size];
    let mut counter = 1;
    let mut pos: (i32, i32) = (0, 0);
    let mut direction = (1, 0);
    let mut num_recent_turns = 0;
    loop {
        match pos {
            // if we are out of bounds, rotate direction
            (r, c) if r < 0 || c < 0 || r as usize == size || c as usize == size => {
                if num_recent_turns > 2{
                    break;
                }

                match direction {
                    (1, 0) => { direction = (0, -1); pos = (pos.0-1, pos.1) },
                    (0, -1) => { direction = (-1, 0); pos = (pos.0, pos.1+1) },
                    (-1, 0) => { direction = (0, 1); pos = (pos.0+1, pos.1) },
                    (0, 1) => { direction = (1, 0); pos = (pos.0, pos.1-1) },
                    _ => ()
                }
                num_recent_turns += 1;
            }

            // if we have to turn, turn set a flag
            (r, c) if rv[c as usize][r as usize] != 0 => {
                if num_recent_turns > 2 {
                    break;
                }

                match direction {
                    (1, 0) => { direction = (0, -1); pos = (pos.0-1, pos.1) },
                    (0, -1) => { direction = (-1, 0); pos = (pos.0, pos.1+1) },
                    (-1, 0) => { direction = (0, 1); pos = (pos.0+1, pos.1) },
                    (0, 1) => { direction = (1, 0); pos = (pos.0, pos.1-1);  },
                    _ => ()
                }
                num_recent_turns += 1;
            }

            (r, c) => {
                rv[c as usize][r as usize] = counter;
                counter += 1;
                num_recent_turns = 0;
            }
        }
        pos = (pos.0 + direction.0, pos.1 + direction.1);
    }
    rv
}
