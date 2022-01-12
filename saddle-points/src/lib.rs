fn col_smallest_ns(input: &[Vec<u64>]) -> Vec<u64> {
    let mut rv = vec![];
    let height = input.len();
    let width = input[0].len();
    for c in 0..width {
        let mut col_nums = vec![];
        for r in 0..height {
            col_nums.push(input[r][c]);
        }

        if col_nums.len() > 0 {
            rv.push(*col_nums.iter().min().unwrap());
        }
    }
    rv
}

fn row_largest_ns(input: &[Vec<u64>]) -> Vec<u64> {
    let mut rv = vec![];
    let height = input.len();
    let width = input[0].len();
    for r in 0..height {
        let mut row_nums = vec![];
        for c in 0..width {
            row_nums.push(input[r][c])
        }

        if row_nums.len() > 0 {
            rv.push(*row_nums.iter().max().unwrap());
        }
    }
    rv
}


pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut rv = vec![];
    let height = input.len();
    let width = input[0].len();
    let row_largest = row_largest_ns(input);
    let col_smallest = col_smallest_ns(input);
    for r in 0..height {
        for c in 0..width {
            let n = input[r][c];
            if n == col_smallest[c] && n == row_largest[r] {
                rv.push((r, c));
            }
        }
    }
    rv
}
