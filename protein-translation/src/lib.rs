use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    hm: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.hm.get(codon).map(|&s| s)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins: Vec<&'a str> = vec![];

        for idx in 0..rna.len() {
            let idx = idx * 3;
            let codon: String = rna.chars().skip(idx).take(3).collect();

            if codon.len() == 0 {
                continue;
            }

            if codon.len() < 3 {
                return None;
            }

            let maybe_protein = self.hm.get(&codon.as_str());
            match maybe_protein {
                None => return None,
                Some(protein) if *protein == "stop codon" => break,
                Some(protein) => proteins.push(protein)
            }
        }

        Some(proteins)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        hm: pairs.into_iter().collect(),
    }
}
