use std::collections::HashMap;

pub fn count(nucleotide : char, dna : &str) -> usize {
    dna.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(dna : &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for nucleotide in ['A','C','T','G'].iter() {
        result.insert(*nucleotide, count(*nucleotide, dna));
    }

    result    
}