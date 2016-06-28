extern crate csv;

use std::collections::HashMap;
use std::path::Path;

#[derive(Eq, PartialEq, Debug)]
pub struct RibonucleicAcid {
    acids : String,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DeoxyribonucleicAcid {
    acids : String,
}

impl RibonucleicAcid {
    pub fn new(rna_string : &str) -> RibonucleicAcid {
        RibonucleicAcid { acids: rna_string.to_string() }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(dna_string : &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { acids: dna_string.to_string() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let table = get_transcription_table();
        
        RibonucleicAcid { 
            acids: self.acids.chars()
                             .map(|c| *(table.get(&c).unwrap_or(&c)))
                             .collect::<String>() 
        }
    }
}

fn get_transcription_table() -> HashMap<char, char> {
    let filepath = Path::new("./transcription.csv");
    let mut reader = csv::Reader::from_file(filepath).unwrap().has_headers(false);
    reader.decode::<(char,char)>().filter(Result::is_ok)
                                  .map(Result::unwrap)
                                  .collect::<HashMap<char,char>>()
}