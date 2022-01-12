type Vertex = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Rectangle {
    tl: Vertex,
    tr: Option<Vertex>,
    br: Option<Vertex>,
    bl: Option<Vertex>,
    all_connected: bool,
}

impl Rectangle {
    pub fn is_complete(&self) -> bool {
        self.tr.is_some() && self.br.is_some() && self.bl.is_some() && self.all_connected
    }
}

fn successors(rec: Rectangle, grid: &Vec<Vec<char>>) -> Vec<Rectangle> {
    let mut rv: Vec<Rectangle> = vec![];

    if rec.tr.is_none() {
        let connected_vertices_right: Vec<Vertex> = {
            let mut vtxs = vec![];
            let row_idx = rec.tl.0;
            let col_start = rec.tl.1;
            let col_end = grid[0].len();

            for col_idx in col_start..col_end {
                if rec.tl.1 == col_idx {
                    continue;
                }

                let ch = grid[row_idx][col_idx];
                if !"-+".contains(ch) {
                    break;
                }

                if grid[row_idx][col_idx] == '+' {
                    vtxs.push((row_idx, col_idx));
                }
            }

            vtxs
        };

        for vtx in connected_vertices_right {
            let mut new_rec = rec.clone();
            new_rec.tr = Some(vtx);
            rv.push(new_rec);
        }

        rv
    } else if rec.br.is_none() {
        let connected_vertices_down: Vec<Vertex> = {
            let mut vtxs = vec![];

            let col_idx = rec.tr.unwrap().1;
            let row_start = rec.tl.0;
            let row_end = grid.len();

            for row_idx in row_start..row_end {
                if rec.tr.unwrap().0 == row_idx {
                    continue;
                }

                let ch = grid[row_idx][col_idx];
                if !"|+".contains(ch) {
                    break;
                }

                if grid[row_idx][col_idx] == '+' {
                    vtxs.push((row_idx, col_idx));
                }
            }

            vtxs
        };

        for vtx in connected_vertices_down {
            let mut new_rec = rec.clone();
            new_rec.br = Some(vtx);
            rv.push(new_rec);
        }

        rv
    } else if rec.bl.is_none() {
        let connected_vertex_left: Option<Vertex> = {
            let row_idx = rec.br.unwrap().0;
            let col_range = ((rec.tl.1)..=(rec.br.unwrap().1)).rev();
            let mut rv_none = false;

            for col_idx in col_range {
                if rec.br.unwrap().1 == col_idx {
                    continue;
                }

                let ch = grid[row_idx][col_idx];
                if !"-+".contains(ch) {
                    rv_none = true;
                    break;
                }
            }

            if grid[rec.br.unwrap().0][rec.tl.1] != '+' {
                rv_none = true;
            }

            if rv_none {
                None
            } else {
                Some((row_idx, rec.tl.1))
            }
        };

        match connected_vertex_left {
            Some(v) => {
                let mut new_rec = rec.clone();
                new_rec.bl = Some(v);
                rv.push(new_rec);
                rv
            }

            None => rv,
        }
    } else if !rec.all_connected {
        let connected_vertex_up: bool = {
            let col_idx = rec.tl.1;
            let row_start = rec.tl.0;
            let row_end = rec.bl.unwrap().0;

            let mut rv = true;

            for row_idx in row_start..row_end {
                if rec.bl.unwrap().0 == row_idx {
                    continue;
                }

                let ch = grid[row_idx][col_idx];
                if !"|+".contains(ch) {
                    rv = false;
                    break;
                }
            }

            rv
        };

        match connected_vertex_up {
            true => {
                let mut new_rec = rec.clone();
                new_rec.all_connected = true;
                rv.push(new_rec);
                rv
            }

            false => rv,
        }
    } else {
        panic!("Reaching this branch is a logic bug.")
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let grid: Vec<Vec<char>> = lines.iter().map(|&s| s.chars().collect()).collect();
    let mut queue: Vec<Rectangle> = vec![];

    for (i, &line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '+' {
                let r = Rectangle {
                    tl: (i, j),
                    tr: None,
                    br: None,
                    bl: None,
                    all_connected: false,
                };

                queue.push(r);
            }
        }
    }

    let mut total = 0;
    while queue.len() > 0 {
        let rec = queue.pop().unwrap();
        if rec.is_complete() {
            total += 1;
        } else {
            queue.extend(&successors(rec, &grid))
        }
    }

    total
}
