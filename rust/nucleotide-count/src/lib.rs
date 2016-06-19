use std::collections::HashMap;

pub fn count(nucleotide : char, dna : &str) -> usize {
    dna.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(dna : &str) -> HashMap<char, usize> {
    "ACTG".chars().map(|n| (n, count(n, dna))).collect()
}