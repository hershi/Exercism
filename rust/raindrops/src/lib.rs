pub fn raindrops(input : u32) -> String {
    let factors = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];
    
    let result = factors.iter()
                        .filter(|entry| input % entry.0 == 0)
                        .fold(String::new(), 
                              |mut intermediate_result, entry| {
                                  intermediate_result.push_str(entry.1); 
                                  intermediate_result
                              });
    if !result.is_empty() { result } else { input.to_string() } 
}