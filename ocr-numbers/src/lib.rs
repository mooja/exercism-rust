static STABLE: [&str; 10] = [
" _ 
| |
|_|
   ",
"   
  |
  |
   ",
" _ 
 _|
|_ 
   ",
" _ 
 _|
 _|
   ",
"   
|_|
  |
   ",
" _ 
|_ 
 _|
   ",
" _ 
|_ 
|_|
   ",
" _ 
  |
  |
   ",
" _ 
|_|
|_|
   ",
" _ 
|_|
 _|
   ",
];

#[derive(Debug, PartialEq)]
pub struct Symbol(String);

impl Symbol {
    pub fn from(s: &str) -> Self {
        let sanitized = s.chars().filter(|&c| c!= '\n').collect::<String>();
        if sanitized.len() != 12 {
            panic!("symbol len is {} instead of 12, {}", sanitized.len(), s);
        }
        Symbol { 0: sanitized }
    }
}

pub fn symbol_to_char(symbol: Symbol) -> char {
    for (i, &s) in STABLE.iter().enumerate() {
        let s = Symbol::from(s);
        if symbol == s {
            return (i as u8 + 48) as char
        }
    }
    return '?'
}

pub struct SymbolsIterator <'a> {
    rows: Vec<&'a str>,
    row_cell_idx: usize,
    col_cell_idx: usize
}

impl <'a> Iterator for SymbolsIterator<'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        let n_cell_rows = self.rows.len() / 4;
        let n_cell_cols = self.rows[0].len() / 3;

        if self.col_cell_idx == n_cell_cols {
            self.col_cell_idx = 0;
            self.row_cell_idx += 1;
        }

        if self.row_cell_idx == n_cell_rows {
            return None
        }

        let mut cell_str = String::new();
        for inner_cell_row_idx in 0..4 {
            for inner_cell_col_idx in 0..3 {
                let row_idx = self.row_cell_idx * 4 + inner_cell_row_idx;
                let col_idx = self.col_cell_idx * 3 + inner_cell_col_idx;
                let c = self.rows[row_idx].as_bytes()[col_idx] as char;
                cell_str.push(c);
            }
            cell_str.push('\n');
        }

        self.col_cell_idx += 1;
        return Some(Symbol::from(&cell_str[..]));
    }
}

pub fn symbols_iter<'a> (input: &'a str) -> SymbolsIterator {
    SymbolsIterator {
        rows: input.split('\n').collect(),
        row_cell_idx: 0,
        col_cell_idx: 0
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

fn correct_dimensions(input: &str) -> Result<(), Error> {
    let n_rows = input.chars()
        .filter(|&c| c == '\n')
        .count() + 1;

    if n_rows == 0 || n_rows % 4 != 0 {
        return Err(Error::InvalidRowCount(n_rows));
    }

    let n_cols = input
        .split('\n')
        .map(|row_str| row_str.len())
        .collect::<Vec<_>>();
    
    let all_rows_multiple_of_three = n_cols.iter()
        .all(|&len| len != 0 && len % 3 == 0);

    if !all_rows_multiple_of_three {
        return Err(Error::InvalidColumnCount(n_cols[0]));
    }

    Ok(())
}


pub fn convert(input: &str) -> Result<String, Error> {
    correct_dimensions(input)?;

    let lines = input.split('\n').collect::<Vec<_>>();
    let n_cols = lines[0].len() / 3;
    let chrs_iter = symbols_iter(input)
        .map(|s| symbol_to_char(s));

    let mut rv = String::new();
    for (i, ch) in chrs_iter.enumerate() {
        if i != 0 && i % n_cols == 0 {
            rv.push(',');
        }
        rv.push(ch);
    }

    Ok(rv)
}
