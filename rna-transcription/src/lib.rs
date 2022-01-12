#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: String

}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: String
}


impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotides = String::new();
        for (i, c) in dna.chars().enumerate() {
            if !"GCTA".contains(c) {
                return Err(i)
            }
            nucleotides.push(c);
        }
        Ok(Dna { nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        let mut rv = String::new();
        for c in self.nucleotides.chars() {
            match c {
                'G' => rv.push('C'),
                'C' => rv.push('G'),
                'T' => rv.push('A'),
                'A' => rv.push('U'),
                _ => panic!("invalid nuclotide")
            }
        }
        Rna::new(&rv).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotides = String::new();
        for (i, c) in rna.chars().enumerate() {
            if !"CGAU".contains(c) {
                return Err(i)
            }
            nucleotides.push(c);
        }
        Ok(Rna { nucleotides })
    }
}
