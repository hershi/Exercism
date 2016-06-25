use std::collections::BTreeMap;

pub fn transform(input : &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter()
         .flat_map( 
             |(&score,strings)| {
                 strings.iter().map(move |string| (string.to_lowercase(), score))
             }
         ).collect::<BTreeMap<String, i32>>()
}