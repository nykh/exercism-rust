use std::collections::*;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nuc: char, dna: &str) -> Result<u32, ()> {    
    if VALID_NUCLEOTIDES.contains(&nuc) {
        let mut total = 0;
        for c in dna.chars() {
            if !VALID_NUCLEOTIDES.contains(&c) {
                return Err(())
            } else if c == nuc {
                total += 1;
            }
        }
        Ok(total)
    } else {
        Err(())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    let mut sums: HashMap<char, usize> = [('A', 0), ('C', 0), ('T', 0), ('G', 0)]
                                         .iter().cloned().collect();

    for c in dna.chars() {
        if !sums.contains_key(&c) {
            return Err(())
        }

        let counter = sums.entry(c).or_insert(0);
        *counter += 1;
    }
    
    Ok(sums)
} 