use std::char;


fn neighbor_bombs(minefield: &[&str], row: usize, col: usize) -> u8 {
    let height = minefield.len();
    let width = minefield[0].len();

    let row_lo = if row == 0 { 0 } else { row - 1 };
    let row_hi = if row + 1 == height { row } else { row + 1 };
    let col_lo = if col == 0 { 0 } else { col - 1 };
    let col_hi = if col + 1 == width { col } else { col + 1 };

    let mut bomb_count = 0;
    for row_idx in row_lo..=row_hi {
        for col_idx in col_lo..=col_hi {
            if minefield[row_idx].as_bytes()[col_idx] == '*' as u8 {
                bomb_count += 1;
            }
        }
    }
    bomb_count
}


pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut rv: Vec<String> = vec![];

    for row_idx in 0..minefield.len() {
        let mut row_str = String::new();
        for col_idx in 0..minefield[0].len() {
            let cell = minefield[row_idx].as_bytes()[col_idx];
            if cell as char == '*' {
                row_str.push('*');
                continue;
            }

            match neighbor_bombs(minefield, row_idx, col_idx) {
                0 => row_str.push(' '),
                nbombs => row_str.push(char::from_digit(nbombs as u32, 10).unwrap()),
            }
        }
        rv.push(row_str);
    }

    rv
}