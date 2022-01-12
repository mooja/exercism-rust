#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessPosition {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition {
                row: rank,
                col: file,
            }),
            _ => None,
        }
    }
    pub fn can_move_to(&self) -> Vec<ChessPosition> {
        let diagonal_positions: Vec<ChessPosition> = {
            let mut ps: Vec<ChessPosition> = vec![];
            for i in -8..=7 {
                let cp = ChessPosition::new(self.row+i, self.col+i);
                match cp {
                    Some(pos) => ps.push(pos),
                    None => ()
                }

                let cp = ChessPosition::new(self.row-i, self.col+i);
                match cp {
                    Some(pos) => ps.push(pos),
                    None => ()
                }
            }
            ps
        };

        let mut axis_positions: Vec<ChessPosition> = {
            let mut ps = vec![];
            for i in -8..=8 {
                let pos = ChessPosition::new(self.row, i);
                match pos {
                    Some(p) => ps.push(p),
                    None => ()
                }

                let pos = ChessPosition::new(i, self.col);
                match pos {
                    Some(p) => ps.push(p),
                    None => ()
                }
            }
            ps
        };

        let mut rv = diagonal_positions;
        rv.append(&mut axis_positions);
        let mut rv: Vec<ChessPosition> = rv.into_iter().filter(|p| p != self).collect();
        rv.sort();
        rv.dedup();
        rv
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.can_move_to().contains(&other.pos)
    }
}
