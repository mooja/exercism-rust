pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        match self.row_count {
            0 => {}

            1 => {
                rows.push(vec![1]);
            }

            row_count => {
                rows.push(vec![1]);
                rows.push(vec![1, 1]);

                for i in 2..row_count {
                    let prev_row = rows.get((i as usize) - 1).unwrap();
                    let mut new_row: Vec<u32> = vec![];
                    let (first, last) = (
                        prev_row.first().unwrap().clone(),
                        prev_row.last().unwrap().clone(),
                    );

                    new_row.push(first);
                    for ii in 0..(prev_row.len() - 1) {
                        let a = prev_row.get(ii).unwrap().clone();
                        let b = prev_row.get(ii + 1).unwrap().clone();
                        new_row.push(a + b);
                    }
                    new_row.push(last);
                    rows.push(new_row);
                }
            }
        }
        rows
    }
}
