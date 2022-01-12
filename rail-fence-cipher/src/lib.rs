pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let num_chars = text
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .count();

        let mut rails = vec![];
        for _ in 0..self.0 {
            rails.push(vec![' '; num_chars]);
        }

        let mut row_idx = 0;
        let mut col_idx = 0;
        let mut row_direction: i32 = -1;

        for c in text.chars() {
            rails[row_idx][col_idx] = c;
            if row_idx == 0 || row_idx == (self.0 - 1) as usize {
                row_direction = row_direction * (-1);
            }
            col_idx += 1;
            row_idx = ((row_idx as i32) + row_direction) as usize;
        }

        let mut ciphertext = String::new();
        for row_idx in 0..rails.len() {
            for col_idx in 0..rails[0].len() {
                if rails[row_idx][col_idx] != ' ' {
                    ciphertext.push(rails[row_idx][col_idx]);
                }
            }
        }

        ciphertext
    }

    pub fn decode(&self, cipher: &str) -> String {
        let num_chars = cipher
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .count();

        let mut rails = vec![];
        for _ in 0..self.0 {
            rails.push(vec![' '; num_chars]);
        }

        let mut row_idx = 0;
        let mut col_idx = 0;
        let mut row_direction: i32 = -1;

        for _ in 0..num_chars {
            rails[row_idx][col_idx] = '?';
            if row_idx == 0 || row_idx == (self.0 - 1) as usize {
                row_direction = row_direction * (-1);
            }
            col_idx += 1;
            row_idx = ((row_idx as i32) + row_direction) as usize;
        }

        let mut ciphertext_iter = cipher.chars();
        for row_idx in 0..rails.len() {
            for ch in &mut rails[row_idx] {
                if *ch == '?' {
                    *ch = ciphertext_iter.next().unwrap();
                }
            }
        }

        let mut plaintext = String::new();
        for col_idx in 0..rails[0].len() {
            for row_idx in 0..self.0 {
                let c = rails[row_idx as usize][col_idx];
                if c != ' ' {
                    plaintext.push(c);
                }
            }
        }

        plaintext
    }
}
